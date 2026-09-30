//! A bidirectional JSON-RPC 2.0 peer over NDJSON, usable on its own.
//!
//! Messages are framed as one compact JSON object per line. The same
//! [`Conn`] carries host to plugin requests and notifications as well as the
//! `host.*` calls of the plugin. A connection needs a running tokio runtime:
//! [`Conn::new`] starts the task that writes the outgoing lines.
//!
//! ```no_run
//! use nginxui_plugin_sdk::jsonrpc::{handler, Conn};
//! use serde_json::json;
//!
//! # async fn demo() {
//! let (a, b) = tokio::io::duplex(64 * 1024);
//! let (ar, aw) = tokio::io::split(a);
//! let (br, bw) = tokio::io::split(b);
//! let left = Conn::new(ar, aw);
//! let right = Conn::new(br, bw);
//! right.handle("echo", handler(|params| async move { Ok(params) }));
//! tokio::spawn({ let l = left.clone(); async move { let _ = l.serve().await; } });
//! tokio::spawn({ let r = right.clone(); async move { let _ = r.serve().await; } });
//! let out = left.call("echo", json!({"n": 1})).await.unwrap();
//! assert_eq!(out, json!({"n": 1}));
//! # }
//! ```

mod conn;
mod frame;

pub use conn::{handler, BoxFuture, CallError, Conn, Handler, DRAIN_TIMEOUT};
pub use frame::{MAX_MESSAGE_BYTES, VERSION};

#[cfg(test)]
mod tests;
