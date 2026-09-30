use std::fmt;

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use super::Config;

/// The payload of `storage.validate`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageValidateParams {
    /// Backend code declared in the manifest, without any host prefix.
    pub backend: String,
    /// Field values the user filled in, keyed by field key.
    pub config: Config,
}

/// The payload of `storage.put`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StoragePutParams {
    /// Backend code.
    pub backend: String,
    /// Field values of the backend.
    pub config: Config,
    /// Relative path with `/` separated segments.
    pub key: String,
    /// Absolute path of the file to store, inside the exchange directory of
    /// the data directory of the plugin.
    pub source_path: String,
}

/// The payload of `storage.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageGetParams {
    /// Backend code.
    pub backend: String,
    /// Field values of the backend.
    pub config: Config,
    /// Key of the object.
    pub key: String,
    /// Absolute path the plugin writes the object to, inside the exchange
    /// directory of the data directory of the plugin.
    pub target_path: String,
}

/// The reply to `storage.put` and `storage.get`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageSizeResult {
    /// Number of bytes stored or written.
    pub size: ByteSize,
}

/// The payload of `storage.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageListParams {
    /// Backend code.
    pub backend: String,
    /// Field values of the backend.
    pub config: Config,
    /// Plain string prefix of the keys to list. Empty lists every object.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub prefix: String,
}

/// The reply to `storage.list`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageListResult {
    /// The objects.
    pub objects: Vec<StorageObject>,
}

/// One stored object.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageObject {
    /// Key of the object.
    pub key: String,
    /// Size in bytes.
    pub size: ByteSize,
    /// RFC 3339 timestamp, empty when unknown.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub modified_at: String,
}

/// The payload of `storage.delete`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageDeleteParams {
    /// Backend code.
    pub backend: String,
    /// Field values of the backend.
    pub config: Config,
    /// Key of the object.
    pub key: String,
}

/// A number of bytes.
///
/// The contract carries it as a double, so it stays a JSON number past
/// 4 GiB. It decodes from any JSON number that is a whole, non negative byte
/// count, with or without a fraction, such as `5242880` and `5242880.0`, and
/// encodes as an integer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSize(pub u64);

impl ByteSize {
    /// Largest byte count a double holds exactly.
    pub const MAX: u64 = 1 << 53;

    /// Returns the number of bytes.
    pub fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for ByteSize {
    fn from(n: u64) -> Self {
        ByteSize(n)
    }
}

impl From<ByteSize> for u64 {
    fn from(n: ByteSize) -> Self {
        n.0
    }
}

impl PartialEq<u64> for ByteSize {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for ByteSize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0)
    }
}

struct ByteSizeVisitor;

impl ByteSizeVisitor {
    fn from_f64<E: de::Error>(v: f64) -> Result<ByteSize, E> {
        if v < 0.0 || v.fract() != 0.0 || v > ByteSize::MAX as f64 {
            return Err(E::custom(format!("size {v} is not a byte count")));
        }
        Ok(ByteSize(v as u64))
    }
}

impl Visitor<'_> for ByteSizeVisitor {
    type Value = ByteSize;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a whole, non negative number of bytes")
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<ByteSize, E> {
        if v > ByteSize::MAX {
            return Err(E::custom(format!("size {v} is not a byte count")));
        }
        Ok(ByteSize(v))
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<ByteSize, E> {
        u64::try_from(v)
            .map_err(|_| E::custom(format!("size {v} is not a byte count")))
            .and_then(|v| self.visit_u64(v))
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<ByteSize, E> {
        Self::from_f64(v)
    }
}

impl<'de> Deserialize<'de> for ByteSize {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ByteSizeVisitor)
    }
}
