use serde::{Deserialize, Serialize};

use super::{is_zero_i32, Config};

/// The payload of `blocklist.fetch`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlocklistFetchParams {
    /// Source kind code declared in the manifest, without any host prefix.
    pub source: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
}

/// The reply to `blocklist.fetch`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlocklistFetchResult {
    /// The complete current list of the source. Empty denies nothing.
    pub entries: Vec<BlocklistEntry>,
    /// How long the list stays fresh. 0 leaves it to the host.
    #[serde(skip_serializing_if = "is_zero_i32")]
    pub ttl_seconds: i32,
}

/// One address or network to deny.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BlocklistEntry {
    /// An IPv4 or IPv6 address, or a network in CIDR notation.
    pub cidr: String,
    /// Says why the source lists the entry, for display.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}
