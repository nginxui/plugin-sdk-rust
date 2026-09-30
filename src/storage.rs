//! The `storage` capability: keeping host files in a backend.

use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::{empty, Registry};
use crate::protocol::{
    method, ByteSize, Config, Error, StorageDeleteParams, StorageGetParams, StorageListParams,
    StorageListResult, StorageObject, StoragePutParams, StorageSizeResult, StorageValidateParams,
};
use crate::rfc3339;

/// The payload of `storage.put`.
pub type StoragePutRequest = StoragePutParams;

/// The payload of `storage.get`.
pub type StorageGetRequest = StorageGetParams;

/// The payload of `storage.list`.
pub type StorageListRequest = StorageListParams;

/// The payload of `storage.delete`.
pub type StorageDeleteRequest = StorageDeleteParams;

/// Keeps host files in the backends the manifest declares in its `storage`
/// block.
///
/// File contents never travel in a message: the host puts the file to store
/// at `req.source_path` and expects a fetched object at `req.target_path`,
/// both inside the exchange directory of the data directory. `req.backend` is
/// the backend code and `req.config` holds the values of its form. Return
/// [`Error::invalid_config`] for a bad field and any other error for a vendor
/// failure. The SDK answers a malformed key with -32602 before the handler
/// runs.
#[async_trait]
pub trait StorageHandler: Send + Sync + 'static {
    /// Stores the file at `req.source_path` under `req.key`, replacing an
    /// object stored there, and returns the number of bytes stored. It must
    /// only read the source file.
    async fn put(&self, ctx: &Context, req: StoragePutRequest) -> Result<u64, Error>;

    /// Writes the object stored under `req.key` to `req.target_path`, which
    /// does not exist yet, and returns the number of bytes written.
    async fn get(&self, ctx: &Context, req: StorageGetRequest) -> Result<u64, Error>;

    /// Returns every object whose key starts with `req.prefix`, a plain
    /// string prefix. [`StorageObject::stored`] builds an entry.
    async fn list(
        &self,
        ctx: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error>;

    /// Removes the object stored under `req.key`. A missing object is not an
    /// error.
    async fn delete(&self, ctx: &Context, req: StorageDeleteRequest) -> Result<(), Error>;

    /// Checks a backend configuration without storing anything. The default
    /// answers `-32002` (unsupported).
    async fn validate(&self, ctx: &Context, backend: &str, config: &Config) -> Result<(), Error> {
        let _ = (ctx, backend, config);
        Err(Error::unsupported(method::STORAGE_VALIDATE))
    }
}

impl StorageObject {
    /// Builds a `storage.list` entry. A missing modified time is left out.
    pub fn stored(key: impl Into<String>, size: u64, modified: Option<SystemTime>) -> Self {
        StorageObject {
            key: key.into(),
            size: ByteSize(size),
            modified_at: modified.map(rfc3339::format_utc).unwrap_or_default(),
        }
    }
}

/// Bounds a key, in bytes.
const MAX_STORAGE_KEY_LENGTH: usize = 1024;

/// Reports whether `key` has the form the host sends: a relative path of `/`
/// separated segments, none of them empty, `.` or `..`, without a backslash
/// or a control character, at most 1024 bytes long.
pub fn valid_storage_key(key: &str) -> bool {
    if key.is_empty() || key.len() > MAX_STORAGE_KEY_LENGTH {
        return false;
    }
    if key
        .chars()
        .any(|c| (c as u32) < 0x20 || c == '\u{7f}' || c == '\\')
    {
        return false;
    }
    key.split('/')
        .all(|segment| !(segment.is_empty() || segment == "." || segment == ".."))
}

/// Rejects a malformed key before it reaches the handler, so a handler can
/// build vendor paths from it safely.
fn check_key(key: &str) -> Result<(), Error> {
    if valid_storage_key(key) {
        Ok(())
    } else {
        Err(Error::invalid_params(format!(
            "malformed storage key: {key}"
        )))
    }
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: StorageHandler + ?Sized> StorageHandler for Arc<T> {
    async fn put(&self, ctx: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        (**self).put(ctx, req).await
    }

    async fn get(&self, ctx: &Context, req: StorageGetRequest) -> Result<u64, Error> {
        (**self).get(ctx, req).await
    }

    async fn list(
        &self,
        ctx: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error> {
        (**self).list(ctx, req).await
    }

    async fn delete(&self, ctx: &Context, req: StorageDeleteRequest) -> Result<(), Error> {
        (**self).delete(ctx, req).await
    }

    async fn validate(&self, ctx: &Context, backend: &str, config: &Config) -> Result<(), Error> {
        (**self).validate(ctx, backend, config).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn StorageHandler>) {
    let handler = h.clone();
    reg.tracked(method::STORAGE_PUT, move |ctx, req: StoragePutRequest| {
        let h = handler.clone();
        async move {
            check_key(&req.key)?;
            let size = h.put(&ctx, req).await?;
            Ok(StorageSizeResult {
                size: ByteSize(size),
            })
        }
    });
    let handler = h.clone();
    reg.tracked(method::STORAGE_GET, move |ctx, req: StorageGetRequest| {
        let h = handler.clone();
        async move {
            check_key(&req.key)?;
            let size = h.get(&ctx, req).await?;
            Ok(StorageSizeResult {
                size: ByteSize(size),
            })
        }
    });
    let handler = h.clone();
    reg.tracked(method::STORAGE_LIST, move |ctx, req: StorageListRequest| {
        let h = handler.clone();
        async move {
            let objects = h.list(&ctx, req).await?;
            Ok(StorageListResult { objects })
        }
    });
    let handler = h.clone();
    reg.tracked(
        method::STORAGE_DELETE,
        move |ctx, req: StorageDeleteRequest| {
            let h = handler.clone();
            async move {
                check_key(&req.key)?;
                h.delete(&ctx, req).await?;
                empty()
            }
        },
    );
    let handler = h;
    reg.tracked(
        method::STORAGE_VALIDATE,
        move |ctx, params: StorageValidateParams| {
            let h = handler.clone();
            async move {
                h.validate(&ctx, &params.backend, &params.config).await?;
                empty()
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_keys() {
        assert!(valid_storage_key("nginx-ui/daily_1790000000.zip.key"));
        for key in [
            "", "/abs", "a//b", "a/../b", "./a", "a/", "a\\b", "a\0b", "a\u{7f}b",
        ] {
            assert!(!valid_storage_key(key), "{key:?} must be rejected");
        }
        assert!(!valid_storage_key(&"k".repeat(1025)));
        assert!(valid_storage_key(&"k".repeat(1024)));
    }

    #[test]
    fn stored_object_formats_the_modified_time() {
        let modified = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_789_959_605);
        let object = StorageObject::stored("a/b", 7, Some(modified));
        assert_eq!(object.modified_at, "2026-09-21T03:00:05Z");
        assert_eq!(StorageObject::stored("a", 1, None).modified_at, "");
    }
}
