use serde::{Deserialize, Serialize};

use super::Config;

/// The payload of `notify.send`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotifySendParams {
    /// Channel code declared in the manifest, without any host prefix.
    pub channel: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
    /// Already translated into the language of the notifier.
    pub title: String,
    /// Already translated and rendered.
    pub content: String,
    /// One of [`notify_severity`].
    pub severity: String,
}

/// The payload of `notify.validate`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotifyValidateParams {
    /// Channel code.
    pub channel: String,
    /// Field values the user filled in.
    pub config: Config,
}

/// Values of [`NotifySendParams::severity`].
pub mod notify_severity {
    /// Informational.
    pub const INFO: &str = "info";
    /// A success.
    pub const SUCCESS: &str = "success";
    /// A warning.
    pub const WARNING: &str = "warning";
    /// An error.
    pub const ERROR: &str = "error";
}
