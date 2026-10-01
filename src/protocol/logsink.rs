use serde::{Deserialize, Serialize};

use super::{is_zero_f64, is_zero_i32, ByteSize};

/// One message of the `log.push` stream: one access log line.
///
/// `log.push` is a client stream on the gRPC transport only and has no
/// JSON-RPC form. The serde form documents the protobuf JSON
/// mapping of the message.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LogSinkPushParams {
    /// Absolute path of the access log the line was read from.
    pub log_path: String,
    /// The line.
    pub entry: LogEntry,
}

/// One access log line. Fields the host could not extract are empty.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LogEntry {
    /// Time of the request in RFC 3339 form.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub timestamp: String,
    /// Client address.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub remote_addr: String,
    /// Request method.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub request_method: String,
    /// Request URI, including the query string.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub request_uri: String,
    /// Protocol of the request.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    /// Response status.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub status: i32,
    /// `$body_bytes_sent`.
    #[serde(skip_serializing_if = "is_zero_size")]
    pub body_bytes_sent: ByteSize,
    /// Referer header.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub referer: String,
    /// User agent header.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub user_agent: String,
    /// Address of the upstream server.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub upstream_addr: String,
    /// `$request_time` in seconds.
    #[serde(skip_serializing_if = "is_zero_f64")]
    pub request_time: f64,
    /// `$upstream_response_time` in seconds.
    #[serde(skip_serializing_if = "is_zero_f64")]
    pub upstream_response_time: f64,
    /// Host header.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub host: String,
    /// The line as nginx wrote it.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub raw: String,
    /// [`log_format::COMBINED`] or [`log_format::RAW`].
    #[serde(skip_serializing_if = "String::is_empty")]
    pub format: String,
}

fn is_zero_size(n: &ByteSize) -> bool {
    n.0 == 0
}

/// The answer to one `log.push` stream.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LogSinkPushResult {
    /// Entries the plugin kept.
    pub accepted: i32,
    /// Entries the plugin received and discarded.
    pub rejected: i32,
}

/// Values of [`LogEntry::format`] and of [`ManifestLogSink::formats`].
///
/// [`ManifestLogSink::formats`]: super::ManifestLogSink::formats
pub mod log_format {
    /// A line the host parsed as the nginx combined format, optionally
    /// followed by `$request_time` and `$upstream_response_time`.
    pub const COMBINED: &str = "combined";
    /// Any other line. Only `raw` and `timestamp` are set.
    pub const RAW: &str = "raw";
}

/// Bounds of `log_sink` in the manifest.
pub mod log_sink_limits {
    /// Default `log_sink.batch_size`.
    pub const DEFAULT_BATCH_SIZE: usize = 256;
    /// Largest `log_sink.batch_size`.
    pub const MAX_BATCH_SIZE: usize = 4096;
    /// Default `log_sink.flush_interval_ms`.
    pub const DEFAULT_FLUSH_INTERVAL_MS: i32 = 500;
    /// Smallest `log_sink.flush_interval_ms`.
    pub const MIN_FLUSH_INTERVAL_MS: i32 = 50;
}
