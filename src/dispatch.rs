//! Wiring shared by the capability modules: the table of handlers the
//! runtime serves on stdio and gRPC alike.

use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::Notify;

use crate::context::Context;
use crate::jsonrpc::BoxFuture;
use crate::protocol::{EmptyResult, Error};

/// A handler as the runtime serves it: the context of the call and the
/// `params` of the message, `null` when it has none.
pub(crate) type RtHandler =
    Arc<dyn Fn(Context, Value) -> BoxFuture<Result<Value, Error>> + Send + Sync>;

/// Consumes one client stream of a streaming rpc.
#[async_trait::async_trait]
pub(crate) trait StreamHandler: Send {
    /// Takes one request message, in protobuf encoding.
    async fn add(&mut self, ctx: &Context, message: &[u8]) -> Result<(), Error>;
    /// Runs once the caller closed the stream and returns the response
    /// message in protobuf encoding.
    async fn finish(&mut self, ctx: &Context) -> Result<Vec<u8>, Error>;
}

pub(crate) type StreamFactory = Box<dyn Fn() -> Box<dyn StreamHandler> + Send + Sync>;

/// Counts the capability calls `plugin.shutdown` waits for.
#[derive(Default)]
pub(crate) struct Inflight {
    count: AtomicUsize,
    notify: Notify,
}

/// Holds one slot of [`Inflight`] while alive.
pub(crate) struct InflightGuard(Arc<Inflight>);

impl Drop for InflightGuard {
    fn drop(&mut self) {
        if self.0.count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.0.notify.notify_waiters();
        }
    }
}

impl Inflight {
    pub(crate) fn enter(self: &Arc<Self>) -> InflightGuard {
        self.count.fetch_add(1, Ordering::SeqCst);
        InflightGuard(self.clone())
    }

    pub(crate) fn is_idle(&self) -> bool {
        self.count.load(Ordering::SeqCst) == 0
    }

    /// Completes once no slot is held.
    pub(crate) async fn wait_idle(&self) {
        loop {
            let notified = self.notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if self.is_idle() {
                return;
            }
            notified.await;
        }
    }
}

/// What a plugin serves: handlers by method name and streams by rpc name.
pub(crate) struct Registry {
    pub(crate) handlers: HashMap<String, RtHandler>,
    pub(crate) streams: HashMap<&'static str, StreamFactory>,
    pub(crate) inflight: Arc<Inflight>,
}

impl Registry {
    pub(crate) fn new(inflight: Arc<Inflight>) -> Registry {
        Registry {
            handlers: HashMap::new(),
            streams: HashMap::new(),
            inflight,
        }
    }

    /// Registers a typed capability method that plugin.shutdown waits for.
    pub(crate) fn tracked<P, R, F, Fut>(&mut self, method: &str, f: F)
    where
        P: DeserializeOwned + Default + Send + 'static,
        R: Serialize + 'static,
        F: Fn(Context, P) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<R, Error>> + Send + 'static,
    {
        let inflight = self.inflight.clone();
        let handler = typed(f);
        let tracked: RtHandler = Arc::new(move |ctx, params| {
            let guard = inflight.enter();
            let fut = handler(ctx, params);
            Box::pin(async move {
                let _guard = guard;
                fut.await
            })
        });
        self.handlers.insert(method.to_owned(), tracked);
    }
}

/// Decodes `params`, mapping a malformed payload to -32602. An absent or
/// `null` payload is the default message.
pub(crate) fn decode<T: DeserializeOwned + Default>(params: Value) -> Result<T, Error> {
    if params.is_null() {
        return Ok(T::default());
    }
    serde_json::from_value(params).map_err(|e| Error::invalid_params(e.to_string()))
}

/// Encodes a result, which cannot fail for the protocol types.
pub(crate) fn encode<R: Serialize>(result: R) -> Result<Value, Error> {
    serde_json::to_value(result).map_err(|e| Error::internal(format!("marshal result: {e}")))
}

/// Builds a handler that decodes its params and encodes its result.
pub(crate) fn typed<P, R, F, Fut>(f: F) -> RtHandler
where
    P: DeserializeOwned + Default + Send + 'static,
    R: Serialize + 'static,
    F: Fn(Context, P) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<R, Error>> + Send + 'static,
{
    Arc::new(move |ctx, raw| {
        let params = match decode::<P>(raw) {
            Ok(p) => p,
            Err(e) => return Box::pin(std::future::ready(Err(e))),
        };
        let fut = f(ctx, params);
        Box::pin(async move { encode(fut.await?) })
    })
}

/// The reply of a method with nothing to return.
pub(crate) fn empty() -> Result<EmptyResult, Error> {
    Ok(EmptyResult {})
}
