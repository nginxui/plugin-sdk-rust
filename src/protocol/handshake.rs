use serde::{Deserialize, Serialize};

use super::{is_zero_i32, Settings};

/// Describes the host to the plugin during the handshake.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostInfo {
    /// Host version, for example `2.7.0`.
    pub version: String,
    /// Operating system in GOOS notation.
    pub os: String,
    /// CPU architecture in GOARCH notation.
    pub arch: String,
    /// Current UI locale.
    pub locale: String,
}

/// The payload of `plugin.initialize`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InitializeParams {
    /// The host.
    pub host: HostInfo,
    /// Current settings values.
    pub settings: Settings,
    /// Permissions the person granted, possibly a subset of the manifest's.
    pub permissions: Vec<String>,
}

/// The reply to `plugin.initialize`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InitializeResult {
    /// Protocol major version the plugin implements.
    pub api_version: i32,
    /// Capabilities the plugin implements.
    pub capabilities: Vec<String>,
    /// Transports the plugin can serve, for example `["stdio", "grpc"]`.
    /// Empty means stdio only.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transports: Vec<String>,
    /// Loopback port of the `http` capability, reported when the plugin
    /// cannot listen on a Unix socket.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub http_port: i32,
    /// Loopback port of the gRPC transport, reported when the plugin cannot
    /// listen on a Unix socket.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub rpc_port: i32,
    /// Token the host presents on the loopback gRPC transport.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rpc_token: String,
    /// Absolute path of the Unix socket the gRPC transport listens on. Empty
    /// means `<NGINX_UI_PLUGIN_DATA_DIR>/rpc.sock`.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rpc_socket: String,
}

/// The payload of `plugin.configure`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigureParams {
    /// Latest settings values.
    pub settings: Settings,
}

/// The reply to methods that have nothing to return, the JSON object `{}`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyResult {}
