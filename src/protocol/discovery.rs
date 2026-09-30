use serde::{Deserialize, Serialize};

use super::{is_zero_i32, Config};

/// The payload of `discovery.resolve`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscoveryResolveParams {
    /// Provider code declared in the manifest, without any host prefix.
    pub provider: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
    /// The service to resolve, in the terms of the provider.
    pub service: String,
}

/// The reply to `discovery.resolve`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscoveryResolveResult {
    /// Every server that backs the service right now.
    pub targets: Vec<DiscoveryTarget>,
    /// How long the answer stays fresh. 0 leaves it to the host.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub ttl_seconds: i32,
}

/// One server of a service.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscoveryTarget {
    /// An IPv4 or IPv6 address or a host name, without a port.
    pub address: String,
    /// The TCP port, 1 to 65535.
    pub port: i32,
    /// Relative weight of the server. 0 means 1.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub weight: i32,
    /// Labels the provider attaches to the server, for display.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
