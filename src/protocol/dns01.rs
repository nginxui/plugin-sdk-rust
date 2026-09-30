use serde::{Deserialize, Serialize};

use super::{is_false, is_zero_i32, Config, Settings};

/// The payload of `dns01.present` and `dns01.cleanup`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ChallengeParams {
    /// Vendor code declared in the manifest, without the plugin id prefix.
    pub provider: String,
    /// Merges the credential and additional fields the user filled in.
    pub config: Config,
    /// Per certificate DNS-01 configuration, opaque to the host.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub options: Settings,
    /// Certificate identifier with any leading wildcard removed.
    pub domain: String,
    /// The `_acme-challenge` name, without CNAME following.
    pub fqdn: String,
    /// The CNAME target when known, otherwise equal to `fqdn`.
    pub effective_fqdn: String,
    /// TXT record value derived from `key_auth`.
    pub value: String,
    /// Raw ACME challenge token.
    pub token: String,
    /// ACME key authorization.
    pub key_auth: String,
    /// Asks the plugin to validate shapes without touching the vendor API.
    #[serde(skip_serializing_if = "is_false")]
    pub dry_run: bool,
}

/// The payload of `dns01.options`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01OptionsParams {
    /// Vendor code.
    pub provider: String,
    /// Credential and additional field values.
    pub config: Config,
    /// Per certificate DNS-01 configuration.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub options: Settings,
}

/// The reply to `dns01.options`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01OptionsResult {
    /// How long the host waits for the record to propagate.
    pub propagation_timeout_seconds: i32,
    /// How often the host polls for the record.
    pub polling_interval_seconds: i32,
    /// Non zero when the provider requires challenges to be solved one at a
    /// time.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub sequential_interval_seconds: i32,
}

/// The payload of `dns01.check`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01CheckParams {
    /// Vendor code.
    pub provider: String,
    /// Credential and additional field values.
    pub config: Config,
    /// Per certificate DNS-01 configuration.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub options: Settings,
    /// Certificate identifier.
    pub domain: String,
    /// The `_acme-challenge` name.
    pub fqdn: String,
    /// Expected TXT record value.
    pub value: String,
    /// ACME key authorization.
    pub key_auth: String,
}

/// The reply to `dns01.check`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01CheckResult {
    /// Whether the record is visible.
    pub ready: bool,
    /// The name that was checked when it differs from the request.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub effective_fqdn: String,
    /// Human readable detail.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub detail: String,
}

/// The payload of `dns01.validate`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dns01ValidateParams {
    /// Vendor code.
    pub provider: String,
    /// Credential and additional field values.
    pub config: Config,
}

/// Keys of the per certificate options map understood by the official dns01
/// plugin.
pub mod dns01_option {
    /// Id of the stored credential to use.
    pub const CREDENTIAL_ID: &str = "credential_id";
    /// Do not follow CNAME records.
    pub const DISABLE_CNAME: &str = "disable_cname";
    /// Skip the authoritative nameserver propagation check.
    pub const DISABLE_AUTHORITATIVE_NS_PROPAGATION: &str = "disable_authoritative_ns_propagation";
    /// Skip the recursive nameserver propagation check.
    pub const DISABLE_RECURSIVE_NS_PROPAGATION: &str = "disable_recursive_ns_propagation";
}
