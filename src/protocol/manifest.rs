use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{is_false, is_zero_i32, Settings};

/// The parsed `plugin.json`. Validation lives in the host.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Manifest {
    /// Plugin id in reverse domain form.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Plugin version.
    pub version: String,
    /// Short description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Homepage of the plugin.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub homepage_url: String,
    /// Icon inside the package.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon_path: String,
    /// Protocol major version the plugin targets.
    pub api_version: i32,
    /// Oldest host version the plugin runs on.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub min_nginx_ui_version: String,

    /// Translates `name` and `description`, keyed by host locale code. The
    /// top level fields stay the fallback.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub i18n: HashMap<String, ManifestI18n>,

    /// How to start the plugin process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ManifestServer>,
    /// The browser bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webapp: Option<ManifestWebapp>,
    /// Process-less contributions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ManifestContent>,

    /// Capabilities the plugin implements.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    /// Permissions the plugin requests.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
    /// Hard dependencies on other plugins.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<ManifestRequirement>,
    /// Capabilities other plugins must provide.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires_capabilities: Vec<String>,
    /// Plugins that must not run together with this one.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<String>,
    /// Events the plugin subscribes to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    /// Host scheduled tasks.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cron: Vec<ManifestCron>,
    /// Hosts the plugin connects to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub network_hosts: Vec<String>,

    /// Metadata of the `dns01` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns01: Option<ManifestDns01>,
    /// Metadata of the `http` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<ManifestHttp>,
    /// Drives the auto rendered settings form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_schema: Option<SettingsSchema>,
    /// Metadata of the `notify` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<ManifestNotify>,
    /// Metadata of the `probe` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probe: Option<ManifestProbe>,
    /// Metadata of the `mcp` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp: Option<ManifestMcp>,
    /// Metadata of the `storage` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ManifestStorage>,
    /// Metadata of the `cert.deploy` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<ManifestDeploy>,
    /// Metadata of the `security.blocklist` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocklist: Option<ManifestBlocklist>,
    /// Metadata of the `upstream.discovery` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<ManifestDiscovery>,
    /// Tunes the `log.sink` capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sink: Option<ManifestLogSink>,
}

/// Translates the display fields of a manifest into one language. An empty
/// string means no translation for that field.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestI18n {
    /// Translated name.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Translated description.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// Describes how to start the plugin process.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestServer {
    /// Maps `<goos>-<goarch>` to a path relative to the plugin directory.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub executables: HashMap<String, String>,
    /// Fallback argv for interpreted plugins.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    /// [`lifecycle::RESIDENT`](super::lifecycle::RESIDENT) (default) or
    /// [`lifecycle::ON_DEMAND`](super::lifecycle::ON_DEMAND).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lifecycle: String,
    /// Idle time after which an on demand plugin is stopped.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub idle_timeout_seconds: i32,
    /// Hints for a host that confines plugin processes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ManifestResources>,
}

/// The resources a plugin process needs at most. 0 means no hint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestResources {
    /// Memory in MiB.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub memory_mb: i32,
    /// CPU time in percent of one core, 100 being one core.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub cpu_percent: i32,
    /// Memory in MiB the machine should have for the plugin to work well.
    /// Advice for people choosing plugins, not a limit.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub recommended_memory_mb: i32,
}

/// Describes the optional browser bundle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestWebapp {
    /// The bundle script.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub bundle_path: String,
    /// The bundle stylesheet.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub style_path: String,
    /// Maps a shared runtime library to the semver range the bundle was built
    /// against.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub shared: HashMap<String, String>,
    /// Zero build iframe pages.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<ManifestPage>,
    /// Maps a chunk name to a package relative `.js` file the bundle loads on
    /// demand.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub chunks: HashMap<String, String>,
}

/// A zero build iframe page.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestPage {
    /// Route path.
    pub path: String,
    /// Title by locale.
    pub title: HashMap<String, String>,
    /// Menu icon.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    /// The HTML file inside the package.
    pub file: String,
}

/// Declares process-less contributions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestContent {
    /// Directory holding `conf/` and `block/` config templates.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub templates: String,
    /// Directory holding one `<lang>.po` file per language.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub locales: String,
}

/// A hard dependency on another plugin.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestRequirement {
    /// Plugin id.
    pub id: String,
    /// Semver range.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub version: String,
}

/// A host scheduled task.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestCron {
    /// Id of the entry.
    pub id: String,
    /// Five field cron expression or `@every 10m`.
    pub schedule: String,
    /// Method of the plugin the host calls.
    pub method: String,
}

/// The metadata block of the `dns01` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestDns01 {
    /// The vendors the plugin solves DNS-01 for.
    pub providers: Vec<Dns01Provider>,
}

/// One vendor a plugin can solve DNS-01 for.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01Provider {
    /// Display name.
    pub name: String,
    /// Vendor code.
    pub code: String,
    /// Links to vendor documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Dns01ProviderLinks>,
    /// Default propagation timeout.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub propagation_timeout_seconds: i32,
    /// Default polling interval.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub polling_interval_seconds: i32,
    /// Every value the provider accepts.
    pub form: Dns01ProviderForm,
}

/// Points at vendor documentation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ProviderLinks {
    /// API documentation.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api: String,
}

/// Field groups of a dns01 form.
pub mod dns01_field_group {
    /// A credential.
    pub const CREDENTIAL: &str = "credential";
    /// A setting.
    pub const SETTING: &str = "setting";
}

/// Marks a dns01 form value counted in seconds.
pub const DNS01_FIELD_UNIT_SECONDS: &str = "seconds";

/// Describes how a host lays out the credential form.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ProviderForm {
    /// Fields in display order. Use an empty list for a provider that takes
    /// no values.
    pub fields: Vec<Dns01ProviderField>,
    /// Set only when there is more than one way to sign in.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<Dns01ProviderMethod>,
}

/// One input of the credential form. `label`, `help` and the method names
/// are English gettext msgids.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ProviderField {
    /// Config key.
    pub key: String,
    /// Label.
    pub label: String,
    /// Help text.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub help: String,
    /// One of [`dns01_field_group`].
    pub group: String,
    /// The field may stay empty.
    #[serde(skip_serializing_if = "is_false")]
    pub optional: bool,
    /// The field is a secret, masked and never logged.
    #[serde(skip_serializing_if = "is_false")]
    pub secret: bool,
    /// Default value.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub default: String,
    /// [`DNS01_FIELD_UNIT_SECONDS`] or empty.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub unit: String,
    /// Link to the page where the value is created.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub link: String,
}

/// One way to sign in to a dns01 provider. `fields` lists the credential keys
/// it uses and may be empty, but is always written, as an empty list. Values
/// are fixed config entries the host stores while the method is chosen.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ProviderMethod {
    /// Method name.
    pub name: String,
    /// The method is the recommended one.
    #[serde(skip_serializing_if = "is_false")]
    pub recommended: bool,
    /// Credential keys the method uses.
    pub fields: Vec<String>,
    /// Fixed config entries.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub values: HashMap<String, String>,
}

/// The metadata block of the `http` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestHttp {
    /// `unix` (reverse proxy to a socket) or `rpc` (`http.handle` fallback).
    pub listen: String,
}

/// The metadata block of the `notify` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestNotify {
    /// The channels the plugin delivers through.
    pub channels: Vec<NotifyChannel>,
}

/// One vendor channel a plugin delivers notifications through.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotifyChannel {
    /// Shared across every installed notify plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
}

/// The metadata block of the `probe` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestProbe {
    /// The probe kinds.
    pub kinds: Vec<ProbeKind>,
}

/// One way a plugin can check the health of a target.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProbeKind {
    /// Shared across every installed probe plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
}

/// The metadata block of the `storage` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestStorage {
    /// The backends.
    pub backends: Vec<StorageBackend>,
}

/// One place a plugin can keep host files.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageBackend {
    /// Shared across every installed storage plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
}

/// The metadata block of the `cert.deploy` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestDeploy {
    /// The target kinds.
    pub targets: Vec<DeployTarget>,
}

/// One kind of external target a plugin can push certificates to.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeployTarget {
    /// Shared across every installed cert.deploy plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
}

/// The metadata block of the `security.blocklist` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestBlocklist {
    /// The source kinds.
    pub sources: Vec<BlocklistSource>,
}

/// One kind of source a plugin can fetch a list of addresses to deny from.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlocklistSource {
    /// Shared across every installed security.blocklist plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
    /// Default refresh interval of a source of this kind. 0 means
    /// [`blocklist_refresh::DEFAULT_SECONDS`].
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub refresh_seconds: i32,
}

/// Refresh intervals of a blocklist source kind, in seconds.
pub mod blocklist_refresh {
    /// Used when a kind declares none.
    pub const DEFAULT_SECONDS: i32 = 3600;
    /// The shortest interval.
    pub const MIN_SECONDS: i32 = 60;
}

/// The metadata block of the `upstream.discovery` capability.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestDiscovery {
    /// The providers.
    pub providers: Vec<DiscoveryProvider>,
}

/// One place a plugin can resolve services from.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscoveryProvider {
    /// Shared across every installed upstream.discovery plugin.
    pub code: String,
    /// Display name.
    pub name: String,
    /// The form of the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationSchema>,
}

/// Tunes the `log.sink` capability. Every field is optional.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestLogSink {
    /// The most entries of one `log.push` stream. 0 means
    /// [`log_sink_limits::DEFAULT_BATCH_SIZE`](super::log_sink_limits::DEFAULT_BATCH_SIZE).
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub batch_size: i32,
    /// The longest time a stream stays open once its first entry was sent. 0
    /// means [`log_sink_limits::DEFAULT_FLUSH_INTERVAL_MS`](super::log_sink_limits::DEFAULT_FLUSH_INTERVAL_MS).
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub flush_interval_ms: i32,
    /// The [`log_format`](super::log_format) values the plugin wants. Empty
    /// means every line.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub formats: Vec<String>,
}

/// Drives the form of a notify channel, a probe kind, a storage backend, a
/// deploy target, a blocklist source or a discovery provider. The values
/// travel as a map of strings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigurationSchema {
    /// The fields in display order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<ConfigurationField>,
}

/// One entry of [`ConfigurationSchema`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigurationField {
    /// Config key.
    pub key: String,
    /// One of [`configuration_field`], empty means text.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    /// Label.
    pub display_name: String,
    /// Help text.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub help_text: String,
    /// The field must be filled in.
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
    /// The field is a credential: masked in forms and never logged.
    #[serde(skip_serializing_if = "is_false")]
    pub secret: bool,
}

/// Values of [`ConfigurationField::type`].
pub mod configuration_field {
    /// A single line of text.
    pub const TEXT: &str = "text";
    /// Several lines of text.
    pub const TEXTAREA: &str = "textarea";
    /// A number.
    pub const NUMBER: &str = "number";
    /// A switch.
    pub const BOOL: &str = "bool";
}

/// The metadata block of the `mcp` capability.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ManifestMcp {
    /// The tools.
    pub tools: Vec<McpTool>,
}

/// One Model Context Protocol tool a plugin serves.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct McpTool {
    /// Unique within the plugin. The host publishes it with a prefix derived
    /// from the plugin id.
    pub name: String,
    /// What the tool does, for the AI assistant.
    pub description: String,
    /// JSON Schema of the arguments object.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub input_schema: Settings,
}

/// Drives the auto rendered settings form.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsSchema {
    /// Text above the form.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub header: String,
    /// Text below the form.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub footer: String,
    /// The fields.
    pub settings: Vec<SettingsField>,
}

/// One entry of [`SettingsSchema`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsField {
    /// Settings key.
    pub key: String,
    /// `text`, `bool`, `number`, `select`, `secret`, `textarea` or `list`.
    pub r#type: String,
    /// Label.
    pub display_name: String,
    /// Help text.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub help_text: String,
    /// Default value.
    #[serde(skip_serializing_if = "Value::is_null")]
    pub default: Value,
    /// Choices of a select field.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<SettingsOption>,
    /// The field must be filled in.
    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,
}

/// A choice of a select field.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SettingsOption {
    /// Stored value.
    pub value: String,
    /// Label.
    pub label: String,
}
