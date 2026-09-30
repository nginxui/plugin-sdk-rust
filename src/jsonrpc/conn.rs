use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Map, Value};
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::{mpsc, oneshot, watch, Notify};

use super::frame::{read_line, Line, MAX_MESSAGE_BYTES, VERSION};
use crate::panic::{panic_message, CatchUnwind};
use crate::protocol::{code, Error};

/// Bounds how long [`Conn::serve`] waits for the handlers that are still
/// running when the input ends, so their replies are not lost.
pub const DRAIN_TIMEOUT: Duration = Duration::from_secs(30);

/// Queue length of the outgoing lines.
const WRITE_QUEUE: usize = 256;

/// A boxed, sendable future.
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Serves one inbound method. It gets the `params` member, `null` when the
/// message has none. Returning an [`Error`] sends it verbatim.
pub type Handler = Arc<dyn Fn(Value) -> BoxFuture<Result<Value, Error>> + Send + Sync>;

type InlineHandler = Arc<dyn Fn(Value) -> Result<Value, Error> + Send + Sync>;

/// Wraps an async closure as a [`Handler`].
pub fn handler<F, Fut>(f: F) -> Handler
where
    F: Fn(Value) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Value, Error>> + Send + 'static,
{
    Arc::new(move |params| Box::pin(f(params)))
}

/// Why a call or notification failed.
#[derive(Debug, thiserror::Error)]
pub enum CallError {
    /// The connection is closed.
    #[error("jsonrpc: connection closed")]
    Closed,
    /// The message exceeds 4 MiB.
    #[error("jsonrpc: message exceeds the 4 MiB limit")]
    TooLarge,
    /// The peer answered with an error.
    #[error("{0}")]
    Rpc(Error),
    /// The params could not be serialized.
    #[error("jsonrpc: marshal params for {method}: {message}")]
    Marshal {
        /// The method of the call.
        method: String,
        /// The serializer error.
        message: String,
    },
    /// The result did not decode into the requested type.
    #[error("jsonrpc: unmarshal result of {method}: {message}")]
    Unmarshal {
        /// The method of the call.
        method: String,
        /// The decoder error.
        message: String,
    },
}

enum Entry {
    Async(Handler),
    Inline(InlineHandler),
}

struct Reply {
    result: Value,
    error: Option<Error>,
}

#[derive(Default)]
struct Inflight {
    count: AtomicUsize,
    notify: Notify,
}

struct InflightGuard(Arc<Inflight>);

impl Drop for InflightGuard {
    fn drop(&mut self) {
        if self.0.count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.0.notify.notify_waiters();
        }
    }
}

impl Inflight {
    fn enter(self: &Arc<Self>) -> InflightGuard {
        self.count.fetch_add(1, Ordering::SeqCst);
        InflightGuard(self.clone())
    }

    async fn wait_idle(&self) {
        loop {
            let notified = self.notify.notified();
            tokio::pin!(notified);
            // Register interest before checking the count.
            notified.as_mut().enable();
            if self.count.load(Ordering::SeqCst) == 0 {
                return;
            }
            notified.await;
        }
    }
}

type BoxedReader = Box<dyn AsyncBufRead + Send + Unpin>;

struct Inner {
    reader: Mutex<Option<BoxedReader>>,
    tx: mpsc::Sender<Vec<u8>>,
    writer: Mutex<Option<tokio::task::JoinHandle<()>>>,
    handlers: RwLock<HashMap<String, Entry>>,
    pending: Mutex<HashMap<String, oneshot::Sender<Reply>>>,
    next_id: AtomicI64,
    inflight: Arc<Inflight>,
    closed: watch::Sender<bool>,
}

/// A concurrency safe JSON-RPC 2.0 peer over a reader and writer pair.
///
/// It is cheap to clone, all clones share one connection.
#[derive(Clone)]
pub struct Conn {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conn")
            .field("closed", &self.is_closed())
            .finish_non_exhaustive()
    }
}

impl Conn {
    /// Builds a peer reading requests from `reader` and writing messages to
    /// `writer`. It must be called inside a tokio runtime.
    pub fn new<R, W>(reader: R, writer: W) -> Conn
    where
        R: AsyncRead + Send + Unpin + 'static,
        W: AsyncWrite + Send + Unpin + 'static,
    {
        let (tx, rx) = mpsc::channel(WRITE_QUEUE);
        let (closed, _) = watch::channel(false);
        let inner = Arc::new(Inner {
            reader: Mutex::new(Some(Box::new(BufReader::with_capacity(64 << 10, reader)))),
            tx,
            writer: Mutex::new(None),
            handlers: RwLock::new(HashMap::new()),
            pending: Mutex::new(HashMap::new()),
            next_id: AtomicI64::new(0),
            inflight: Arc::new(Inflight::default()),
            closed,
        });
        let task = tokio::spawn(write_loop(writer, rx, inner.closed.subscribe(), {
            let weak = Arc::downgrade(&inner);
            move || {
                if let Some(inner) = weak.upgrade() {
                    inner.close();
                }
            }
        }));
        *inner.writer.lock().expect("writer lock") = Some(task);
        Conn { inner }
    }

    /// Registers a handler for `method`, replacing an earlier one.
    pub fn handle(&self, method: impl Into<String>, handler: Handler) {
        self.inner
            .handlers
            .write()
            .expect("handlers lock")
            .insert(method.into(), Entry::Async(handler));
    }

    /// Registers a synchronous handler that runs on the read loop, before the
    /// next line is read. Use it for work that must be ordered before the
    /// messages that follow, and keep it short.
    pub fn handle_inline<F>(&self, method: impl Into<String>, handler: F)
    where
        F: Fn(Value) -> Result<Value, Error> + Send + Sync + 'static,
    {
        self.inner
            .handlers
            .write()
            .expect("handlers lock")
            .insert(method.into(), Entry::Inline(Arc::new(handler)));
    }

    /// Reports whether a handler is registered for `method`.
    pub fn handled(&self, method: &str) -> bool {
        self.inner
            .handlers
            .read()
            .expect("handlers lock")
            .contains_key(method)
    }

    /// Returns the handler registered for `method`. Other transports use it
    /// to serve a method exactly as this connection would.
    pub fn handler(&self, method: &str) -> Option<Handler> {
        let handlers = self.inner.handlers.read().expect("handlers lock");
        match handlers.get(method)? {
            Entry::Async(h) => Some(h.clone()),
            Entry::Inline(h) => {
                let h = h.clone();
                Some(Arc::new(move |params| {
                    let h = h.clone();
                    Box::pin(async move { h(params) })
                }))
            }
        }
    }

    /// Drops every handler. Handlers may hold a clone of the connection, this
    /// breaks the cycle when the connection is done.
    pub fn clear_handlers(&self) {
        self.inner.handlers.write().expect("handlers lock").clear();
    }

    /// Reports whether the connection is closed.
    pub fn is_closed(&self) -> bool {
        *self.inner.closed.borrow()
    }

    /// Completes once the connection is closed.
    pub async fn closed(&self) {
        wait_closed(&mut self.inner.closed.subscribe()).await;
    }

    /// Closes the connection and unblocks every pending call. Lines that are
    /// already queued are still written, see [`Conn::flush`].
    pub fn close(&self) {
        self.inner.close();
    }

    /// Waits until the writer has written every queued line, at most
    /// `timeout`. Call it after [`Conn::close`] before the process exits.
    pub async fn flush(&self, timeout: Duration) {
        let handle = self.inner.writer.lock().expect("writer lock").take();
        if let Some(handle) = handle {
            let _ = tokio::time::timeout(timeout, handle).await;
        }
    }

    /// Sends a request and waits for the matching reply. `params` is left out
    /// when it serializes to `null`, so `()` sends none.
    pub async fn call(&self, method: &str, params: impl Serialize) -> Result<Value, CallError> {
        if self.is_closed() {
            return Err(CallError::Closed);
        }
        let params = to_params(method, params)?;

        let id = self.inner.next_id.fetch_add(1, Ordering::SeqCst) + 1;
        let (tx, rx) = oneshot::channel();
        let key = id.to_string();
        self.inner
            .pending
            .lock()
            .expect("pending lock")
            .insert(key.clone(), tx);
        let _cleanup = PendingGuard {
            inner: &self.inner,
            key,
        };

        let mut msg = Map::new();
        msg.insert("jsonrpc".into(), json!(VERSION));
        msg.insert("id".into(), json!(id));
        msg.insert("method".into(), json!(method));
        if let Some(params) = params {
            msg.insert("params".into(), params);
        }
        self.inner.write(Value::Object(msg)).await?;

        let mut closed = self.inner.closed.subscribe();
        tokio::select! {
            reply = rx => match reply {
                Ok(reply) => match reply.error {
                    Some(err) => Err(CallError::Rpc(err)),
                    None => Ok(reply.result),
                },
                Err(_) => Err(CallError::Closed),
            },
            _ = wait_closed(&mut closed) => Err(CallError::Closed),
        }
    }

    /// Like [`Conn::call`], decoding the result into `R`. A `null` or absent
    /// result decodes when `R` accepts it, `()` and `Option<_>` do.
    pub async fn call_typed<R: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
    ) -> Result<R, CallError> {
        let value = self.call(method, params).await?;
        serde_json::from_value(value).map_err(|e| CallError::Unmarshal {
            method: method.to_owned(),
            message: e.to_string(),
        })
    }

    /// Sends a notification, which carries no id and is never answered.
    pub async fn notify(&self, method: &str, params: impl Serialize) -> Result<(), CallError> {
        if self.is_closed() {
            return Err(CallError::Closed);
        }
        let msg = notification(method, to_params(method, params)?);
        self.inner.write(msg).await
    }

    /// Queues a notification without waiting. It fails when the connection
    /// is closed or the queue is full, which lets a logger fall back to
    /// another sink instead of blocking.
    pub fn try_notify(&self, method: &str, params: impl Serialize) -> Result<(), CallError> {
        if self.is_closed() {
            return Err(CallError::Closed);
        }
        let msg = notification(method, to_params(method, params)?);
        let line = encode(&msg)?;
        self.inner.tx.try_send(line).map_err(|_| CallError::Closed)
    }

    /// Reads messages until the input ends or the connection is closed.
    ///
    /// Inbound requests run in their own task so that the peer may pipeline
    /// them. A clean end of input waits for the running handlers, at most
    /// [`DRAIN_TIMEOUT`], and returns `Ok`. It can be called once.
    pub async fn serve(&self) -> io::Result<()> {
        let taken = self.inner.reader.lock().expect("reader lock").take();
        let Some(mut reader) = taken else {
            return Err(io::Error::other("jsonrpc: serve was already called"));
        };
        let result = self.read_loop(&mut reader).await;
        self.close();
        result
    }

    async fn read_loop(&self, reader: &mut BoxedReader) -> io::Result<()> {
        let mut closed = self.inner.closed.subscribe();
        loop {
            let line = tokio::select! {
                biased;
                _ = wait_closed(&mut closed) => return Ok(()),
                line = read_line(reader, MAX_MESSAGE_BYTES) => line,
            };
            match line {
                Ok(Line::Eof) => {
                    self.drain().await;
                    return Ok(());
                }
                Ok(Line::TooLong) => {
                    self.write_error(
                        Value::Null,
                        Error::new(
                            code::INVALID_REQUEST,
                            "jsonrpc: message exceeds the 4 MiB limit",
                            None,
                        ),
                    );
                }
                Ok(Line::Data(line)) => {
                    let trimmed = line.trim_ascii();
                    if !trimmed.is_empty() {
                        self.dispatch(trimmed);
                    }
                }
                Err(e) => {
                    if is_clean_end(&e) {
                        self.drain().await;
                        return Ok(());
                    }
                    return Err(e);
                }
            }
        }
    }

    /// Waits for the running handlers to answer, giving up after
    /// [`DRAIN_TIMEOUT`] so a stuck handler cannot hold the process open.
    async fn drain(&self) {
        let _ = tokio::time::timeout(DRAIN_TIMEOUT, self.inner.inflight.wait_idle()).await;
    }

    /// Routes one raw line, which may hold a single message or a batch.
    fn dispatch(&self, raw: &[u8]) {
        let value: Value = match serde_json::from_slice(raw) {
            Ok(v) => v,
            Err(_) => {
                self.write_error(
                    Value::Null,
                    Error::new(code::PARSE_ERROR, "parse error", None),
                );
                return;
            }
        };
        match value {
            // Replies to a batch are emitted as individual NDJSON lines.
            Value::Array(items) => {
                for item in items {
                    self.handle_one(item);
                }
            }
            other => self.handle_one(other),
        }
    }

    fn handle_one(&self, value: Value) {
        let Value::Object(mut obj) = value else {
            self.write_error(
                Value::Null,
                Error::new(
                    code::INVALID_REQUEST,
                    "invalid request: not a JSON object",
                    None,
                ),
            );
            return;
        };

        let id = obj.remove("id").filter(|id| !id.is_null());
        let version_ok = obj.get("jsonrpc").and_then(Value::as_str) == Some(VERSION);

        let method = match obj.remove("method") {
            None | Some(Value::Null) => {
                // A response, or garbage.
                if obj.contains_key("result") || obj.contains_key("error") {
                    self.deliver(id, obj);
                } else if let Some(id) = id {
                    self.write_error(
                        id,
                        Error::new(
                            code::INVALID_REQUEST,
                            "invalid request: no method, result or error",
                            None,
                        ),
                    );
                }
                return;
            }
            Some(Value::String(method)) => method,
            Some(_) => {
                self.write_error(
                    id.unwrap_or(Value::Null),
                    Error::new(
                        code::INVALID_REQUEST,
                        "invalid request: method must be a string",
                        None,
                    ),
                );
                return;
            }
        };

        if !version_ok {
            // A notification is never answered, whatever it looks like.
            if let Some(id) = id {
                self.write_error(
                    id,
                    Error::new(
                        code::INVALID_REQUEST,
                        "invalid request: jsonrpc must be \"2.0\"",
                        None,
                    ),
                );
            }
            return;
        }

        let params = obj.remove("params").unwrap_or(Value::Null);
        let is_notification = id.is_none();

        enum Found {
            Async(Handler),
            Inline(InlineHandler),
        }
        let found = {
            let handlers = self.inner.handlers.read().expect("handlers lock");
            match handlers.get(&method) {
                Some(Entry::Async(h)) => Some(Found::Async(h.clone())),
                Some(Entry::Inline(h)) => Some(Found::Inline(h.clone())),
                None => None,
            }
        };

        match found {
            None => {
                if let Some(id) = id {
                    self.write_error(id, Error::method_not_found(&method));
                }
            }
            Some(Found::Inline(h)) => {
                let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| h(params)));
                let outcome = outcome
                    .unwrap_or_else(|panic| Err(panic_error(&method, panic_message(&*panic))));
                if let Some(id) = id {
                    self.reply(id, outcome);
                }
            }
            Some(Found::Async(h)) => {
                let guard = self.inner.inflight.enter();
                let conn = self.clone();
                tokio::spawn(async move {
                    let _guard = guard;
                    let outcome = match CatchUnwind(h(params)).await {
                        Ok(outcome) => outcome,
                        Err(panic) => Err(panic_error(&method, panic_message(&*panic))),
                    };
                    // A notification is never answered, whatever the handler
                    // returned.
                    if !is_notification {
                        conn.reply(id.unwrap_or(Value::Null), outcome);
                    }
                });
            }
        }
    }

    fn deliver(&self, id: Option<Value>, mut obj: Map<String, Value>) {
        let Some(id) = id else { return };
        let sender = self
            .inner
            .pending
            .lock()
            .expect("pending lock")
            .remove(&id.to_string());
        let Some(sender) = sender else { return };

        let error = match obj.remove("error") {
            None | Some(Value::Null) => None,
            Some(v) => Some(serde_json::from_value::<Error>(v).unwrap_or_else(|e| {
                Error::internal(format!("malformed error object in reply: {e}"))
            })),
        };
        let result = obj.remove("result").unwrap_or(Value::Null);
        // The waiting call may have given up already.
        let _ = sender.send(Reply { result, error });
    }

    fn reply(&self, id: Value, outcome: Result<Value, Error>) {
        match outcome {
            Ok(result) => {
                let msg = json!({"jsonrpc": VERSION, "id": id, "result": result});
                match encode(&msg) {
                    Ok(line) => self.enqueue(line),
                    Err(_) => self.write_error(
                        id,
                        Error::internal("marshal result: the reply exceeds the 4 MiB limit"),
                    ),
                }
            }
            Err(err) => self.write_error(id, err),
        }
    }

    fn write_error(&self, id: Value, err: Error) {
        let msg = json!({"jsonrpc": VERSION, "id": id, "error": err});
        if let Ok(line) = encode(&msg) {
            self.enqueue(line);
        }
    }

    /// Queues a line from the read loop or a handler task. When the queue is
    /// full the line waits in a task, so the read loop never blocks.
    fn enqueue(&self, line: Vec<u8>) {
        if self.is_closed() {
            return;
        }
        if let Err(mpsc::error::TrySendError::Full(line)) = self.inner.tx.try_send(line) {
            let tx = self.inner.tx.clone();
            tokio::spawn(async move {
                let _ = tx.send(line).await;
            });
        }
    }
}

impl Inner {
    fn close(&self) {
        self.closed.send_replace(true);
        // Dropping the senders fails every pending call.
        self.pending.lock().expect("pending lock").clear();
    }

    async fn write(&self, msg: Value) -> Result<(), CallError> {
        let line = encode(&msg)?;
        let mut closed = self.closed.subscribe();
        tokio::select! {
            sent = self.tx.send(line) => sent.map_err(|_| CallError::Closed),
            _ = wait_closed(&mut closed) => Err(CallError::Closed),
        }
    }
}

struct PendingGuard<'a> {
    inner: &'a Inner,
    key: String,
}

impl Drop for PendingGuard<'_> {
    fn drop(&mut self) {
        self.inner
            .pending
            .lock()
            .expect("pending lock")
            .remove(&self.key);
    }
}

fn to_params(method: &str, params: impl Serialize) -> Result<Option<Value>, CallError> {
    match serde_json::to_value(params) {
        Ok(Value::Null) => Ok(None),
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(CallError::Marshal {
            method: method.to_owned(),
            message: e.to_string(),
        }),
    }
}

fn notification(method: &str, params: Option<Value>) -> Value {
    let mut msg = Map::new();
    msg.insert("jsonrpc".into(), json!(VERSION));
    msg.insert("method".into(), json!(method));
    if let Some(params) = params {
        msg.insert("params".into(), params);
    }
    Value::Object(msg)
}

/// Serializes one message as a line, enforcing the size limit.
fn encode(msg: &Value) -> Result<Vec<u8>, CallError> {
    let mut line = serde_json::to_vec(msg).map_err(|e| CallError::Marshal {
        method: String::new(),
        message: e.to_string(),
    })?;
    if line.len() + 1 > MAX_MESSAGE_BYTES {
        return Err(CallError::TooLarge);
    }
    line.push(b'\n');
    Ok(line)
}

async fn write_loop<W: AsyncWrite + Unpin>(
    mut writer: W,
    mut rx: mpsc::Receiver<Vec<u8>>,
    mut closed: watch::Receiver<bool>,
    on_failure: impl Fn(),
) {
    loop {
        let line = tokio::select! {
            biased;
            line = rx.recv() => line,
            _ = wait_closed(&mut closed) => {
                // Write what is already queued, then stop.
                rx.close();
                while let Some(line) = rx.recv().await {
                    if write_line(&mut writer, &line).await.is_err() {
                        break;
                    }
                }
                let _ = writer.shutdown().await;
                return;
            }
        };
        let Some(line) = line else { return };
        if write_line(&mut writer, &line).await.is_err() {
            on_failure();
            return;
        }
    }
}

async fn write_line<W: AsyncWrite + Unpin>(writer: &mut W, line: &[u8]) -> io::Result<()> {
    writer.write_all(line).await?;
    writer.flush().await
}

/// Completes once the flag is set. The sender lives as long as the connection
/// does, so a closed channel counts as closed too.
async fn wait_closed(rx: &mut watch::Receiver<bool>) {
    let _ = rx.wait_for(|closed| *closed).await;
}

fn is_clean_end(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::UnexpectedEof
            | io::ErrorKind::BrokenPipe
            | io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionAborted
    )
}

fn panic_error(method: &str, message: String) -> Error {
    Error::new(
        code::INTERNAL_ERROR,
        format!("panic in {method}: {message}"),
        None,
    )
}
