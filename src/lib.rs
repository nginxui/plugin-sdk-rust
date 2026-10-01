#![doc = include_str!("../README.md")]
// Unsafe code is limited to the Win32 calls that secure named pipes.
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod blocklist;
mod context;
mod deploy;
mod discovery;
mod dispatch;
mod dns01;
mod events;
mod grpc;
mod host;
pub mod http;
pub mod jsonrpc;
pub mod logger;
mod logsink;
mod mcp;
mod notify;
mod options;
mod panic;
pub mod pb;
mod pipe;
mod plugin;
mod probe;
pub mod protocol;
mod redact;
mod rfc3339;
mod runtime;
mod serve;
mod storage;
mod transport;
mod util;

pub use blocklist::{BlocklistHandler, BlocklistRequest, BlocklistResult};
pub use context::Context;
pub use deploy::{full_chain_pem, DeployHandler, DeployRequest};
pub use discovery::{unknown_service, DiscoveryHandler, DiscoveryRequest, DiscoveryResult};
pub use dns01::{Dns01Handler, Dns01Request};
pub use grpc::{MAX_GRPC_MESSAGE_BYTES, RPC_SOCKET_NAME};
pub use host::{current_host, Activity, Host, HostError, Info};
pub use http::{HttpHandler, HTTP_SOCKET_NAME};
pub use logger::Level;
pub use logsink::{LogSinkEntry, LogSinkHandler, MAX_LOG_SINK_BATCH};
pub use mcp::{unknown_tool, McpHandler, McpRequest, McpResult, McpTools};
pub use notify::{NotifyHandler, NotifyRequest};
pub use options::{env, Network, Options};
pub use plugin::Plugin;
pub use probe::{ProbeHandler, ProbeRequest, ProbeResult};
pub use protocol::Error;
pub use redact::redact;
pub use runtime::{run, run_until, run_with, SHUTDOWN_DRAIN};
pub use serve::{serve, serve_blocking, serve_blocking_with, serve_with};
pub use storage::{
    valid_storage_key, StorageDeleteRequest, StorageGetRequest, StorageHandler, StorageListRequest,
    StoragePutRequest,
};

/// The result type of a handler.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests;
