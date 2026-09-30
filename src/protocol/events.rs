use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The payload of the `events.on` notification.
///
/// A fired cron entry reuses it as the params of a request to the method of
/// the entry, with `type` set to the cron entry id and no `data`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EventNotification {
    /// The event type, one of [`event`].
    pub r#type: String,
    /// Any JSON payload.
    #[serde(skip_serializing_if = "Value::is_null")]
    pub data: Value,
    /// Unix seconds.
    pub ts: i64,
}

/// Event types a plugin may subscribe to through the manifest `events`.
pub mod event {
    /// A certificate was issued.
    pub const CERT_ISSUED: &str = "cert.issued";
    /// A certificate was renewed.
    pub const CERT_RENEWED: &str = "cert.renewed";
    /// A certificate is about to expire.
    pub const CERT_EXPIRING: &str = "cert.expiring";
    /// A site config was saved.
    pub const SITE_SAVED: &str = "site.saved";
    /// A site was enabled.
    pub const SITE_ENABLED: &str = "site.enabled";
    /// A site was disabled.
    pub const SITE_DISABLED: &str = "site.disabled";
    /// nginx was reloaded.
    pub const NGINX_RELOADED: &str = "nginx.reloaded";
    /// A reload of nginx failed.
    pub const NGINX_RELOAD_FAILED: &str = "nginx.reload_failed";
    /// The status of a node changed.
    pub const NODE_STATUS_CHANGED: &str = "node.status_changed";
    /// A node joined.
    pub const NODE_JOINED: &str = "node.joined";
    /// A backup completed.
    pub const BACKUP_COMPLETED: &str = "backup.completed";
    /// A login failed.
    pub const AUTH_LOGIN_FAILED: &str = "auth.login_failed";
    /// A plugin changed.
    pub const PLUGIN_CHANGED: &str = "plugin.changed";
    /// The log files changed. Sent only to plugins holding `log.files`, which
    /// call `host.logs.list` again.
    pub const LOG_PATHS_CHANGED: &str = "log.paths_changed";
}
