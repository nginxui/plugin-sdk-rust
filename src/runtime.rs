//! Wires one [`Plugin`] onto one connection: the lifecycle methods, the
//! capability methods and the optional transports.

use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::time::Duration;

use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{watch, Notify};

use crate::context::Context;
use crate::dispatch::{decode, encode, Inflight, Registry, RtHandler, StreamFactory};
use crate::env;
use crate::grpc::{self, GrpcTransport};
use crate::host::{clear_current_host, set_current_host, with_task_host, Host, Info};
use crate::http::{self, HttpTransport};
use crate::jsonrpc::{BoxFuture, Conn};
use crate::options::{Env, Options};
use crate::plugin::{ConfigureFn, Plugin, ShutdownFn};
use crate::protocol::{
    method, transport, ConfigureParams, EmptyResult, Error, InitializeParams, InitializeResult,
    API_VERSION,
};

/// Bounds how long `plugin.shutdown` waits for the capability calls that are
/// still running.
pub const SHUTDOWN_DRAIN: Duration = Duration::from_secs(30);

/// How long the exit path waits for the writer to flush the last replies.
const FLUSH_TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) struct Runtime {
    capabilities: Vec<String>,
    http: Option<Arc<dyn crate::http::HttpHandler>>,
    has_log_sink: bool,
    configure: Option<ConfigureFn>,
    shutdown: Option<ShutdownFn>,

    opts: Options,
    disable_grpc: bool,
    conn: Conn,
    host: Host,
    stop: watch::Sender<bool>,

    /// The secret every request to the http handler has to carry.
    http_secret: String,

    /// What the plugin serves, filled once by `install`.
    handlers: OnceLock<HashMap<String, RtHandler>>,
    streams: HashMap<&'static str, StreamFactory>,
    inflight: Arc<Inflight>,

    grpc: tokio::sync::Mutex<Option<Arc<GrpcTransport>>>,
    http_transport: Mutex<Option<Arc<HttpTransport>>>,

    exit: Notify,
}

impl Runtime {
    /// The handler serving `name`, on stdio and gRPC alike.
    pub(crate) fn handler(&self, name: &str) -> Option<RtHandler> {
        self.handlers.get()?.get(name).cloned()
    }

    /// The opener of the stream handler of a streaming rpc.
    pub(crate) fn stream_factory(&self, name: &str) -> Option<&StreamFactory> {
        self.streams.get(name)
    }

    pub(crate) fn host(&self) -> Host {
        self.host.clone()
    }

    /// The context of one call.
    pub(crate) fn new_context(&self) -> Context {
        Context::new(self.host.clone(), self.stop.subscribe())
    }

    fn data_dir(&self) -> PathBuf {
        self.host.info().data_dir
    }

    fn request_exit(&self) {
        self.exit.notify_one();
    }

    async fn wait_inflight(&self) {
        let _ = tokio::time::timeout(SHUTDOWN_DRAIN, self.inflight.wait_idle()).await;
    }

    // Handlers of the lifecycle.

    async fn on_initialize(self: Arc<Self>, _ctx: Context, params: Value) -> Result<Value, Error> {
        let params: InitializeParams = decode(strip_nulls(params))?;
        self.host.set_handshake(params);

        let mut res = InitializeResult {
            api_version: API_VERSION,
            capabilities: self.capabilities.clone(),
            transports: vec![transport::STDIO.to_owned()],
            ..InitializeResult::default()
        };
        if self.http.is_some() {
            let t = self
                .start_http()
                .map_err(|e| Error::internal(format!("http listener: {e}")))?;
            t.advertise(&mut res);
        }
        if let Some(t) = self.start_grpc().await {
            t.advertise(&mut res);
        }
        encode(res)
    }

    /// Opens the `http` capability listener once.
    fn start_http(self: &Arc<Self>) -> Result<Arc<HttpTransport>, String> {
        let mut slot = self.http_transport.lock().expect("http lock");
        if let Some(t) = slot.as_ref() {
            return Ok(t.clone());
        }
        let Some(handler) = self.http.clone() else {
            return Err("the plugin does not serve http".to_owned());
        };
        let host = self.host.clone();
        let stop = self.stop.clone();
        let make_context: Arc<dyn Fn() -> Context + Send + Sync> =
            Arc::new(move || Context::new(host.clone(), stop.subscribe()));
        let t = Arc::new(http::start(
            &self.data_dir(),
            self.opts.http_network,
            &self.http_secret,
            handler,
            make_context,
        )?);
        *slot = Some(t.clone());
        Ok(t)
    }

    fn current_http(&self) -> Option<Arc<HttpTransport>> {
        self.http_transport.lock().expect("http lock").clone()
    }

    /// Ends the `http` listener at once and removes its socket.
    fn close_http(&self) {
        let t = self.http_transport.lock().expect("http lock").take();
        if let Some(t) = t {
            t.close();
        }
    }

    /// Starts the gRPC transport once. It returns `None` when gRPC is
    /// disabled or could not start, in which case the plugin stays on stdio.
    async fn start_grpc(self: &Arc<Self>) -> Option<Arc<GrpcTransport>> {
        if self.disable_grpc {
            if !self.has_log_sink {
                return None;
            }
            crate::logger::warn(
                "the log.sink capability needs the gRPC transport, serving it although it was disabled",
            );
        }

        let mut slot = self.grpc.lock().await;
        if let Some(t) = slot.as_ref() {
            return Some(t.clone());
        }
        match grpc::start(
            self.clone(),
            &self.data_dir(),
            self.opts.grpc_network,
            self.opts.grpc_fallback_dirs.as_deref(),
        ) {
            Ok(t) => {
                let t = Arc::new(t);
                *slot = Some(t.clone());
                Some(t)
            }
            Err(e) => {
                crate::logger::warn(format!(
                    "gRPC transport unavailable, serving stdio only: {e}"
                ));
                None
            }
        }
    }

    /// Stops the gRPC transport and removes its socket.
    async fn stop_grpc(&self) {
        let t = self.grpc.lock().await.take();
        if let Some(t) = t {
            t.stop().await;
        }
    }

    async fn on_configure(self: Arc<Self>, ctx: Context, params: Value) -> Result<Value, Error> {
        let params: ConfigureParams = decode(strip_nulls(params))?;
        self.host.set_settings(params.settings);
        if let Some(configure) = &self.configure {
            configure(ctx, self.host.settings()).await?;
        }
        encode(EmptyResult {})
    }

    async fn on_shutdown(self: Arc<Self>, ctx: Context, _params: Value) -> Result<Value, Error> {
        // Answer only once the capability calls still in flight are done.
        self.wait_inflight().await;

        // Stop taking new http requests first. The requests still running
        // get until the shutdown hook returned, which may unblock the long
        // lived ones, plus a short grace period.
        let http = self.current_http();
        if let Some(t) = &http {
            t.begin_stop();
        }

        let outcome = match &self.shutdown {
            Some(shutdown) => shutdown(ctx).await,
            None => Ok(()),
        };
        if let Some(t) = &http {
            t.wait().await;
        }
        outcome?;
        encode(EmptyResult {})
    }
}

/// Removes the members that are `null` from an object, since a `null` member
/// means its default (WIRE-10).
fn strip_nulls(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            Value::Object(map.into_iter().filter(|(_, v)| !v.is_null()).collect())
        }
        other => other,
    }
}

/// Wraps a lifecycle method that needs the runtime. The handler holds the
/// runtime weakly, so the tables that live in the runtime and in the
/// connection do not keep it alive.
fn bind<F, Fut>(rt: &Arc<Runtime>, f: F) -> RtHandler
where
    F: Fn(Arc<Runtime>, Context, Value) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Value, Error>> + Send + 'static,
{
    let weak: Weak<Runtime> = Arc::downgrade(rt);
    Arc::new(move |ctx, params| match weak.upgrade() {
        Some(rt) => Box::pin(f(rt, ctx, params)),
        None => Box::pin(std::future::ready(Err(Error::internal(
            "the plugin is stopping",
        )))),
    })
}

/// Builds the table of everything the plugin serves.
fn build_handlers(
    rt: &Arc<Runtime>,
    plugin: Plugin,
    registry: Registry,
) -> HashMap<String, RtHandler> {
    let mut handlers = registry.handlers;

    handlers.insert(
        method::INITIALIZE.to_owned(),
        bind(rt, |rt, ctx, params| rt.on_initialize(ctx, params)),
    );
    handlers.insert(
        method::CONFIGURE.to_owned(),
        bind(rt, |rt, ctx, params| rt.on_configure(ctx, params)),
    );
    handlers.insert(
        method::SHUTDOWN.to_owned(),
        bind(rt, |rt, ctx, params| rt.on_shutdown(ctx, params)),
    );
    handlers.insert(
        method::PING.to_owned(),
        Arc::new(|_, _| Box::pin(std::future::ready(encode(EmptyResult {})))),
    );
    let weak = Arc::downgrade(rt);
    handlers.insert(
        method::EXIT.to_owned(),
        Arc::new(move |_, _| {
            if let Some(rt) = weak.upgrade() {
                rt.request_exit();
            }
            Box::pin(std::future::ready(Ok(Value::Null)))
        }),
    );

    if !plugin.events.is_empty() {
        handlers.insert(
            method::EVENTS_ON.to_owned(),
            crate::events::events_handler(plugin.events),
        );
    }

    for (name, handler) in plugin.methods {
        if grpc::is_streaming_rpc(&name) {
            // A streaming rpc has no JSON-RPC form, stdio keeps answering
            // method not found for it (spec WIRE-12).
            crate::logger::warn(format!(
                "ignoring the stdio handler for {name}, a streaming rpc travels on gRPC only"
            ));
            continue;
        }
        handlers.insert(name, handler);
    }

    // Whatever the handler logs goes through the host of this plugin.
    handlers
        .into_iter()
        .map(|(name, handler)| {
            let host = rt.host.clone();
            let scoped: RtHandler = Arc::new(move |ctx, params| {
                Box::pin(with_task_host(host.clone(), handler(ctx, params)))
            });
            (name, scoped)
        })
        .collect()
}

/// Serves the plugin over `reader` and `writer` and returns when the host
/// asked the process to stop or `reader` ended.
///
/// It is the testable core of [`serve`](crate::serve) and installs no signal
/// handler. The gRPC transport, when enabled, is started on
/// `plugin.initialize` and stopped before `run` returns.
pub async fn run<R, W>(plugin: Plugin, reader: R, writer: W) -> io::Result<()>
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
{
    run_until(
        plugin,
        reader,
        writer,
        Options::default(),
        std::future::pending(),
    )
    .await
}

/// Like [`run`], with options.
pub async fn run_with<R, W>(
    plugin: Plugin,
    reader: R,
    writer: W,
    options: Options,
) -> io::Result<()>
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
{
    run_until(plugin, reader, writer, options, std::future::pending()).await
}

/// Like [`run_with`], and also returns, after cleaning up, when `shutdown`
/// completes. [`serve`](crate::serve) passes the signal future here.
pub async fn run_until<R, W, S>(
    plugin: Plugin,
    reader: R,
    writer: W,
    options: Options,
    shutdown: S,
) -> io::Result<()>
where
    R: AsyncRead + Send + Unpin + 'static,
    W: AsyncWrite + Send + Unpin + 'static,
    S: Future<Output = ()>,
{
    let conn = Conn::new(reader, writer);
    let env_source = Env::new(&options);
    let info = env_info(&env_source);
    let host = Host::new(conn.clone(), info);

    // Taken now, whatever the plugin serves, so that no child process ever
    // inherits it.
    let http_secret = env_source.take(env::PLUGIN_HTTP_SECRET).unwrap_or_default();
    let disable_grpc =
        options.disable_grpc || env_source.get(env::DISABLE_GRPC).as_deref() == Some("1");

    set_current_host(host.clone());
    let (stop, _) = watch::channel(false);

    let mut registry = Registry::new(Arc::new(Inflight::default()));
    if let Some(h) = plugin.dns01.clone() {
        crate::dns01::register(&mut registry, h);
    }
    if let Some(h) = plugin.notify.clone() {
        crate::notify::register(&mut registry, h);
    }
    if let Some(h) = plugin.probe.clone() {
        crate::probe::register(&mut registry, h);
    }
    if let Some(h) = plugin.mcp.clone() {
        crate::mcp::register(&mut registry, h);
    }
    if let Some(h) = plugin.storage.clone() {
        crate::storage::register(&mut registry, h);
    }
    if let Some(h) = plugin.deploy.clone() {
        crate::deploy::register(&mut registry, h);
    }
    if let Some(h) = plugin.blocklist.clone() {
        crate::blocklist::register(&mut registry, h);
    }
    if let Some(h) = plugin.discovery.clone() {
        crate::discovery::register(&mut registry, h);
    }
    if let Some(h) = plugin.log_sink.clone() {
        crate::logsink::register(&mut registry, h);
    }
    let inflight = registry.inflight.clone();
    let streams = std::mem::take(&mut registry.streams);

    let rt = Arc::new(Runtime {
        capabilities: plugin.declared_capabilities(),
        http: plugin.http.clone(),
        has_log_sink: plugin.log_sink.is_some(),
        configure: plugin.configure.clone(),
        shutdown: plugin.shutdown.clone(),
        opts: options,
        disable_grpc,
        conn: conn.clone(),
        host: host.clone(),
        stop,
        http_secret,
        handlers: OnceLock::new(),
        streams,
        inflight,
        grpc: tokio::sync::Mutex::new(None),
        http_transport: Mutex::new(None),
        exit: Notify::new(),
    });

    let handlers = build_handlers(&rt, plugin, registry);
    install(&rt, &handlers);
    let _ = rt.handlers.set(handlers);

    let mut serve_task = {
        let conn = conn.clone();
        tokio::spawn(async move { conn.serve().await })
    };

    let outcome = tokio::select! {
        biased;
        () = rt.exit.notified() => Ok(()),
        joined = &mut serve_task => match joined {
            Ok(result) => result,
            Err(e) => Err(io::Error::other(e.to_string())),
        },
        () = shutdown => Ok(()),
    };

    rt.stop.send_replace(true);
    conn.close();
    rt.stop_grpc().await;
    rt.close_http();
    conn.clear_handlers();
    serve_task.abort();
    conn.flush(FLUSH_TIMEOUT).await;
    clear_current_host(&host);
    outcome
}

/// Installs the table on the connection.
fn install(rt: &Arc<Runtime>, handlers: &HashMap<String, RtHandler>) {
    for (name, handler) in handlers {
        let handler = handler.clone();
        let host = rt.host.clone();
        let stop = rt.stop.clone();
        rt.conn.handle(
            name.clone(),
            Arc::new(move |params| -> BoxFuture<Result<Value, Error>> {
                handler(Context::new(host.clone(), stop.subscribe()), params)
            }),
        );
    }

    // The lifecycle notification has to take effect before the next line is
    // read, so it runs on the read loop.
    let host = rt.host.clone();
    rt.conn.handle_inline(method::INITIALIZED, move |_| {
        host.set_ready();
        Ok(Value::Null)
    });
}

fn env_info(env_source: &Env) -> Info {
    let api_version = env_source
        .get(env::PLUGIN_API_VERSION)
        .and_then(|raw| raw.parse::<i32>().ok())
        .unwrap_or(API_VERSION);
    Info {
        plugin_id: env_source.get(env::PLUGIN_ID).unwrap_or_default(),
        api_version,
        data_dir: PathBuf::from(env_source.get(env::PLUGIN_DATA_DIR).unwrap_or_default()),
        host_version: env_source.get(env::HOST_VERSION).unwrap_or_default(),
        ..Info::default()
    }
}
