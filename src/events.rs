//! Event delivery through `events.on`.

use std::sync::Arc;

use serde_json::Value;

use crate::context::Context;
use crate::dispatch::RtHandler;
use crate::jsonrpc::BoxFuture;
use crate::protocol::{Error, EventNotification};

/// Handles one event delivered through `events.on`.
pub(crate) type EventFn = Arc<dyn Fn(Context, EventNotification) -> BoxFuture<()> + Send + Sync>;

/// Builds the `events.on` handler that dispatches a delivered event to the
/// handler registered for its type. An event without a handler is ignored.
pub(crate) fn events_handler(handlers: std::collections::HashMap<String, EventFn>) -> RtHandler {
    let handlers = Arc::new(handlers);
    Arc::new(move |ctx: Context, params: Value| {
        let handlers = handlers.clone();
        Box::pin(async move {
            let ev: EventNotification = serde_json::from_value(params)
                .map_err(|e| Error::invalid_params(format!("events.on: {e}")))?;
            if let Some(handler) = handlers.get(&ev.r#type) {
                handler(ctx, ev).await;
            }
            Ok(Value::Null)
        })
    })
}
