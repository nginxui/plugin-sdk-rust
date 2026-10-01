use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Config, Settings};

/// The payload of `host.log`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostLogParams {
    /// `debug`, `info`, `warn` or `error`.
    pub level: String,
    /// The line.
    pub message: String,
    /// Structured fields of the line.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub fields: Settings,
}

/// The payload of `host.kv.get` and `host.kv.delete`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostKvGetParams {
    /// Key in the private store of the plugin.
    pub key: String,
}

/// The reply to `host.kv.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostKvGetResult {
    /// The stored value, null when the key is absent.
    pub value: Value,
    /// Whether the key exists.
    pub found: bool,
}

/// The payload of `host.kv.set`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostKvSetParams {
    /// Key in the private store of the plugin.
    pub key: String,
    /// Any JSON, up to 64 KiB encoded.
    pub value: Value,
}

/// The payload of `host.kv.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostKvListParams {
    /// Lists the keys that start with it.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,
}

/// The reply to `host.kv.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostKvListResult {
    /// The keys.
    pub keys: Vec<String>,
}

/// The reply to `host.settings.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostSettingsGetResult {
    /// The settings values.
    pub settings: Settings,
}

/// The reply to `host.i18n.locale`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostI18nLocaleResult {
    /// The UI locale of the host.
    pub locale: String,
}

/// The payload of `host.credentials.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCredentialsGetParams {
    /// Credential kind, for example `dns`.
    pub kind: String,
    /// Credential id.
    pub id: String,
}

/// The reply to `host.credentials.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCredentialsGetResult {
    /// Credential id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Provider code of the credential.
    pub provider_code: String,
    /// Field values of the credential.
    pub config: Config,
}

/// The payload of `host.cron.register`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCronRegisterParams {
    /// Id of the entry.
    pub id: String,
    /// Five field cron expression or `@every 10m`.
    pub schedule: String,
    /// Method of the plugin the host calls.
    pub method: String,
}

/// The payload of `host.cron.unregister`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCronUnregisterParams {
    /// Id of the entry.
    pub id: String,
}

/// The payload of `host.notify`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNotifyParams {
    /// `info`, `success`, `warning` or `error`.
    pub level: String,
    /// Title of the notification.
    pub title: String,
    /// Body of the notification.
    pub content: String,
    /// Any JSON detail.
    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

/// The reply to `host.metrics.snapshot`. The shape mirrors the analytics
/// snapshot of the host and is opaque to the protocol.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostMetricsSnapshotResult {
    /// The snapshot.
    pub snapshot: Value,
}

/// The reply to `host.logs.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostLogsListResult {
    /// The log files.
    pub logs: Vec<HostLogFile>,
}

/// One log file of [`HostLogsListResult`]. Rotated files are not listed, the
/// plugin finds them next to `path`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostLogFile {
    /// Absolute path of the file.
    pub path: String,
    /// [`log_type::ACCESS`] or [`log_type::ERROR`].
    pub r#type: String,
    /// [`log_source::CONFIG`] or [`log_source::DEFAULT`].
    pub source: String,
    /// The nginx config file that declares the log, empty when the source is
    /// the default.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub config_file: String,
}

/// The payload of `host.activity.set`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostActivitySetParams {
    /// Key of the indicator entry.
    pub key: String,
    /// English source string the host translates.
    pub label: String,
    /// Shows the entry when true and clears it when false.
    pub active: bool,
}

/// Values of [`HostLogFile::type`].
pub mod log_type {
    /// An access log.
    pub const ACCESS: &str = "access";
    /// An error log.
    pub const ERROR: &str = "error";
}

/// Values of [`HostLogFile::source`].
pub mod log_source {
    /// Declared in an nginx config file.
    pub const CONFIG: &str = "config";
    /// The default log of nginx.
    pub const DEFAULT: &str = "default";
}

/// The params of `host.nginx.snippet.put`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippetPutParams {
    /// Name of the snippet inside the plugin, 1 to 64 characters of
    /// `[a-z0-9_-]` starting with a letter or digit.
    pub name: String,
    /// nginx configuration text, at most 256 KiB.
    pub content: String,
}

/// The reply to `host.nginx.snippet.put`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippetPutResult {
    /// False when the snippet already had this content.
    pub changed: bool,
    /// The directive that includes the snippet.
    pub include: String,
}

/// The params of `host.nginx.snippet.delete`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippetDeleteParams {
    /// Name of the snippet.
    pub name: String,
}

/// The reply to `host.nginx.snippet.delete`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippetDeleteResult {
    /// False when there was no such snippet.
    pub removed: bool,
}

/// The reply to `host.nginx.snippet.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippetListResult {
    /// The snippets, sorted by name.
    pub snippets: Vec<HostNginxSnippet>,
}

/// One snippet of [`HostNginxSnippetListResult`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxSnippet {
    /// Name of the snippet.
    pub name: String,
    /// The directive that includes the snippet.
    pub include: String,
}

/// The reply to `host.nginx.config.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxConfigListResult {
    /// Paths relative to the nginx configuration directory, sorted.
    pub files: Vec<String>,
}

/// The params of `host.nginx.config.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxConfigGetParams {
    /// One of the paths `host.nginx.config.list` returns.
    pub path: String,
}

/// The reply to `host.nginx.config.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostNginxConfigGetResult {
    /// The content of the file.
    pub content: String,
}

/// The reply to `host.sites.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostSitesListResult {
    /// The sites, sorted by name.
    pub sites: Vec<HostSite>,
}

/// One site of [`HostSitesListResult`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostSite {
    /// Name of the site, the name of its configuration file.
    pub name: String,
    /// `enabled`, `disabled` or `maintenance`.
    pub status: String,
    /// The addresses the site answers on.
    pub urls: Vec<String>,
    /// Configuration file relative to the nginx configuration directory.
    pub config_file: String,
}

/// The reply to `host.certs.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCertsListResult {
    /// The certificates, sorted by name.
    pub certs: Vec<HostCert>,
}

/// One certificate of [`HostCertsListResult`]. It never carries a private
/// key.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostCert {
    /// Identifier of the certificate in the host, in decimal.
    pub id: String,
    /// Name of the certificate.
    pub name: String,
    /// Domains the certificate was requested for.
    pub domains: Vec<String>,
    /// True when the host renews the certificate itself.
    pub auto_renew: bool,
    /// `http01`, `dns01` or empty.
    pub challenge_method: String,
    /// Key type, such as `2048` or `P256`.
    pub key_type: String,
    /// Start of the validity as an RFC 3339 timestamp, empty when unknown.
    pub not_before: String,
    /// End of the validity as an RFC 3339 timestamp, empty when unknown.
    pub not_after: String,
    /// Organization or name of the issuing CA, empty when unknown.
    pub issuer: String,
}
