use serde::{Deserialize, Serialize};

use super::{is_false, Config};

/// The payload of `deploy.validate`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeployValidateParams {
    /// Target kind code declared in the manifest, without any host prefix.
    pub kind: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
}

/// The payload of `deploy.push`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeployPushParams {
    /// Target kind code.
    pub kind: String,
    /// Field values of the target.
    pub config: Config,
    /// The certificate to push.
    pub certificate: DeployCertificate,
    /// Asks the plugin to check the target without changing anything.
    #[serde(skip_serializing_if = "is_false")]
    pub dry_run: bool,
}

/// The certificate `deploy.push` carries.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeployCertificate {
    /// Name of the certificate.
    pub name: String,
    /// Domains and IP addresses the certificate covers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<String>,
    /// The leaf certificate.
    pub certificate_pem: String,
    /// The private key of the leaf certificate.
    pub private_key_pem: String,
    /// The intermediates, starting with the issuer of the leaf.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub chain_pem: String,
    /// Expiry of the leaf as an RFC 3339 timestamp.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub not_after: String,
}

/// The reply to `deploy.push`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeployPushResult {
    /// Summarizes what was done, or what a dry run would do.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}
