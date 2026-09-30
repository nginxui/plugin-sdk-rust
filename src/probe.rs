//! The `probe` capability: health checks of a target.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::Registry;
use crate::protocol::{method, probe_status, Error, ProbeCheckParams, ProbeCheckResult};

/// The payload of `probe.check`.
pub type ProbeRequest = ProbeCheckParams;

/// The reply to `probe.check`.
pub type ProbeResult = ProbeCheckResult;

/// Checks the health of a target with the probe kinds the manifest declares
/// in its `probe` block.
#[async_trait]
pub trait ProbeHandler: Send + Sync + 'static {
    /// Probes `req.target` once with the kind `req.kind`.
    ///
    /// An unhealthy or unreachable target is a result with status `down`, not
    /// an error: return an error only when the check could not run,
    /// [`Error::invalid_config`] for a bad field. The context expires after
    /// `req.timeout_seconds`, and the call is dropped at that moment.
    async fn check(&self, ctx: &Context, req: ProbeRequest) -> Result<ProbeResult, Error>;
}

impl ProbeCheckResult {
    /// Reports a healthy target.
    pub fn up(latency: Duration) -> Self {
        ProbeCheckResult {
            status: probe_status::UP.to_owned(),
            latency_ms: millis(latency),
            message: String::new(),
        }
    }

    /// Reports a target that is unhealthy or did not answer.
    pub fn down(latency: Duration, message: impl Into<String>) -> Self {
        ProbeCheckResult {
            status: probe_status::DOWN.to_owned(),
            latency_ms: millis(latency),
            message: message.into(),
        }
    }

    /// Reports a target that answers, but not as well as it should.
    pub fn degraded(latency: Duration, message: impl Into<String>) -> Self {
        ProbeCheckResult {
            status: probe_status::DEGRADED.to_owned(),
            latency_ms: millis(latency),
            message: message.into(),
        }
    }
}

fn millis(latency: Duration) -> i32 {
    i32::try_from(latency.as_millis()).unwrap_or(i32::MAX)
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: ProbeHandler + ?Sized> ProbeHandler for Arc<T> {
    async fn check(&self, ctx: &Context, req: ProbeRequest) -> Result<ProbeResult, Error> {
        (**self).check(ctx, req).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn ProbeHandler>) {
    reg.tracked(method::PROBE_CHECK, move |ctx, req: ProbeRequest| {
        let h = h.clone();
        async move {
            let timeout = u64::try_from(req.timeout_seconds).ok().filter(|s| *s > 0);
            let Some(seconds) = timeout else {
                return h.check(&ctx, req).await;
            };
            let ctx = ctx.with_timeout(Duration::from_secs(seconds));
            match tokio::time::timeout(Duration::from_secs(seconds), h.check(&ctx, req)).await {
                Ok(result) => result,
                Err(_) => Err(Error::internal("context deadline exceeded")),
            }
        }
    });
}
