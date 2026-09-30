use serde::{Deserialize, Serialize};

use super::{is_zero_i32, Config};

/// The payload of `probe.check`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProbeCheckParams {
    /// Probe kind code declared in the manifest, without any host prefix.
    pub kind: String,
    /// What to probe, usually an absolute URL.
    pub target: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
    /// How long the host waits for the answer.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub timeout_seconds: i32,
}

/// The reply to `probe.check`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProbeCheckResult {
    /// One of [`probe_status`].
    pub status: String,
    /// Time the check took, in milliseconds.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub latency_ms: i32,
    /// Human readable detail, expected when the status is not `up`.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// Values of [`ProbeCheckResult::status`].
pub mod probe_status {
    /// Healthy.
    pub const UP: &str = "up";
    /// Unhealthy or unreachable.
    pub const DOWN: &str = "down";
    /// Answers, but not as well as it should.
    pub const DEGRADED: &str = "degraded";
}
