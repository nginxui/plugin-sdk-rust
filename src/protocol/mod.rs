//! The wire contract between nginx-ui and plugin processes.
//!
//! Every type maps one to one onto a JSON-RPC 2.0 message exchanged as NDJSON
//! over the stdin and stdout of the plugin. The types follow the proto
//! contract of the plugin-spec repository, which is the source of truth: the
//! JSON members carry the proto field names, an absent member reads as its
//! default and a member with a default value is left out when writing.
//! [`pb`](crate::pb) holds the generated bindings and a test fails when a type
//! here drifts from its message.
//!
//! This module mirrors `internal/plugin/protocol` of the nginx-ui repository
//! and the `protocol` package of the Go SDK. Only add fields, never rename
//! one.

mod blocklist;
mod deploy;
mod discovery;
mod dns01;
mod errors;
mod events;
mod handshake;
mod host;
mod http;
mod logsink;
mod manifest;
mod mcp;
mod names;
mod notify;
mod probe;
mod storage;

pub use blocklist::*;
pub use deploy::*;
pub use discovery::*;
pub use dns01::*;
pub use errors::*;
pub use events::*;
pub use handshake::*;
pub use host::*;
pub use http::*;
pub use logsink::*;
pub use manifest::*;
pub use mcp::*;
pub use names::*;
pub use notify::*;
pub use probe::*;
pub use storage::*;

/// Protocol major version implemented by this SDK.
pub const API_VERSION: i32 = 1;

/// A JSON object, the shape of settings and free-form option maps.
pub type Settings = serde_json::Map<String, serde_json::Value>;

/// String map of form values, keyed by field key.
pub type Config = std::collections::HashMap<String, String>;

pub(crate) fn is_zero_i32(n: &i32) -> bool {
    *n == 0
}

pub(crate) fn is_zero_f64(n: &f64) -> bool {
    *n == 0.0
}

pub(crate) fn is_false(b: &bool) -> bool {
    !*b
}
