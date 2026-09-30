//! The `log.sink` capability: the access log lines of the host as a stream.

use std::ops::Deref;
use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use prost::Message;

use crate::context::Context;
use crate::dispatch::{Registry, StreamHandler};
use crate::pb::v1 as pb;
use crate::protocol::{log_format, log_sink_limits, method, ByteSize, Error, LogEntry};
use crate::rfc3339;

/// Bounds the entries one [`LogSinkHandler::push`] call receives. The host
/// never sends more in one stream, a longer stream reaches `push` in chunks.
pub const MAX_LOG_SINK_BATCH: usize = log_sink_limits::MAX_BATCH_SIZE;

/// One access log line the host streamed.
///
/// It derefs to [`LogEntry`], so `entry.status` and `entry.request_uri` work
/// directly.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LogSinkEntry {
    /// Absolute path of the access log the line was read from.
    pub log_path: String,
    /// The line.
    pub entry: LogEntry,
}

impl Deref for LogSinkEntry {
    type Target = LogEntry;

    fn deref(&self) -> &LogEntry {
        &self.entry
    }
}

impl LogSinkEntry {
    /// Parses `timestamp`. It returns `None` when the host sent none or one
    /// that is not RFC 3339.
    pub fn time(&self) -> Option<SystemTime> {
        rfc3339::parse(&self.entry.timestamp)
    }

    /// Reports whether the host parsed the line into fields. Otherwise only
    /// `raw` and `timestamp` are set.
    pub fn parsed(&self) -> bool {
        self.entry.format == log_format::COMBINED
    }
}

/// Receives the nginx access log lines of the host while they are written.
///
/// The manifest must request the `log.read` permission and may tune the
/// batches in its `log_sink` block. The lines travel as a client stream on
/// the gRPC transport only, so a plugin with a `LogSinkHandler` always serves
/// gRPC.
///
/// Treat every field as untrusted input and as personal data: clients choose
/// the request URI, the referer and the user agent, and a query string may
/// carry a token.
#[async_trait]
pub trait LogSinkHandler: Send + Sync + 'static {
    /// Handles one batch of at most [`MAX_LOG_SINK_BATCH`] entries and returns
    /// how many it kept, the rest counts as rejected.
    ///
    /// Answer promptly: the host sends the next batch only after this one
    /// returned and drops lines meanwhile, so buffer the entries instead of
    /// waiting for a slow destination. An error means the whole batch is lost,
    /// the host does not send it again.
    async fn push(&self, ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error>;
}

/// Serves the `log.push` stream. It has no stdio handler, so a `log.push`
/// request on stdio answers method not found (spec WIRE-12).
/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: LogSinkHandler + ?Sized> LogSinkHandler for Arc<T> {
    async fn push(&self, ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error> {
        (**self).push(ctx, batch).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn LogSinkHandler>) {
    let inflight = reg.inflight.clone();
    reg.streams.insert(
        method::LOG_PUSH,
        Box::new(move || {
            Box::new(LogPushStream {
                handler: h.clone(),
                batch: Vec::new(),
                accepted: 0,
                total: 0,
                _guard: inflight.enter(),
            })
        }),
    );
}

/// Collects the entries of one `log.push` stream.
struct LogPushStream {
    handler: Arc<dyn LogSinkHandler>,
    batch: Vec<LogSinkEntry>,
    accepted: usize,
    total: usize,
    // An open stream counts for plugin.shutdown like any capability call.
    _guard: crate::dispatch::InflightGuard,
}

impl LogPushStream {
    async fn flush(&mut self, ctx: &Context) -> Result<(), Error> {
        if self.batch.is_empty() {
            return Ok(());
        }
        let batch = std::mem::take(&mut self.batch);
        let len = batch.len();
        let accepted = self.handler.push(ctx, batch).await?;
        self.accepted += accepted.min(len);
        self.total += len;
        Ok(())
    }
}

#[async_trait]
impl StreamHandler for LogPushStream {
    async fn add(&mut self, ctx: &Context, message: &[u8]) -> Result<(), Error> {
        let req = pb::LogSinkPushRequest::decode(message)
            .map_err(|e| Error::invalid_params(format!("decode log.push message: {e}")))?;
        self.batch.push(entry_from_proto(req));
        if self.batch.len() >= MAX_LOG_SINK_BATCH {
            return self.flush(ctx).await;
        }
        Ok(())
    }

    async fn finish(&mut self, ctx: &Context) -> Result<Vec<u8>, Error> {
        self.flush(ctx).await?;
        let response = pb::LogSinkPushResponse {
            accepted: u32::try_from(self.accepted).unwrap_or(u32::MAX),
            rejected: u32::try_from(self.total - self.accepted).unwrap_or(u32::MAX),
        };
        Ok(response.encode_to_vec())
    }
}

/// Converts one stream message.
fn entry_from_proto(req: pb::LogSinkPushRequest) -> LogSinkEntry {
    let e = req.entry.unwrap_or_default();
    let body = if e.body_bytes_sent.is_finite() && e.body_bytes_sent > 0.0 {
        // Saturating, the value is a double on the wire.
        e.body_bytes_sent as u64
    } else {
        0
    };
    LogSinkEntry {
        log_path: req.log_path,
        entry: LogEntry {
            timestamp: e.timestamp,
            remote_addr: e.remote_addr,
            request_method: e.request_method,
            request_uri: e.request_uri,
            protocol: e.protocol,
            status: e.status,
            body_bytes_sent: ByteSize(body),
            referer: e.referer,
            user_agent: e.user_agent,
            upstream_addr: e.upstream_addr,
            request_time: e.request_time,
            upstream_response_time: e.upstream_response_time,
            host: e.host,
            raw: e.raw,
            format: e.format,
        },
    }
}
