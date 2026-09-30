use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use serde_json::json;
use tokio::sync::mpsc;

use super::harness::start;
use crate::protocol::{
    capability, code, method, DeployCertificate, DeployPushParams, StorageDeleteParams,
    StorageGetParams, StorageListParams, StorageObject, StoragePutParams, StorageValidateParams,
};
use crate::{
    full_chain_pem, valid_storage_key, Context, DeployHandler, DeployRequest, Error, Plugin,
    StorageDeleteRequest, StorageGetRequest, StorageHandler, StorageListRequest, StoragePutRequest,
};

/// A `StorageHandler` that keeps objects in memory and reads and writes the
/// exchange files it is handed.
#[derive(Default)]
struct MemoryStorage {
    objects: Mutex<BTreeMap<String, Vec<u8>>>,
}

pub fn memory_storage() -> impl StorageHandler {
    MemoryStorage::default()
}

#[async_trait]
impl StorageHandler for MemoryStorage {
    async fn put(&self, _ctx: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        if req.config.get("url").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("url", "url is required"));
        }
        let data = std::fs::read(&req.source_path)?;
        let size = data.len() as u64;
        self.objects.lock().unwrap().insert(req.key, data);
        Ok(size)
    }

    async fn get(&self, _ctx: &Context, req: StorageGetRequest) -> Result<u64, Error> {
        let data = self.objects.lock().unwrap().get(&req.key).cloned();
        let Some(data) = data else {
            return Err(Error::internal(format!(
                "no object is stored under {}",
                req.key
            )));
        };
        std::fs::write(&req.target_path, &data)?;
        Ok(data.len() as u64)
    }

    async fn list(
        &self,
        _ctx: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error> {
        let modified = UNIX_EPOCH + Duration::from_secs(1_789_959_605);
        Ok(self
            .objects
            .lock()
            .unwrap()
            .iter()
            .filter(|(key, _)| key.starts_with(&req.prefix))
            .map(|(key, data)| {
                StorageObject::stored(key.clone(), data.len() as u64, Some(modified))
            })
            .collect())
    }

    async fn delete(&self, _ctx: &Context, req: StorageDeleteRequest) -> Result<(), Error> {
        self.objects.lock().unwrap().remove(&req.key);
        Ok(())
    }
}

/// Also implements the validator.
struct ValidatingStorage(MemoryStorage);

#[async_trait]
impl StorageHandler for ValidatingStorage {
    async fn put(&self, ctx: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        self.0.put(ctx, req).await
    }
    async fn get(&self, ctx: &Context, req: StorageGetRequest) -> Result<u64, Error> {
        self.0.get(ctx, req).await
    }
    async fn list(
        &self,
        ctx: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error> {
        self.0.list(ctx, req).await
    }
    async fn delete(&self, ctx: &Context, req: StorageDeleteRequest) -> Result<(), Error> {
        self.0.delete(ctx, req).await
    }

    async fn validate(
        &self,
        _ctx: &Context,
        backend: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        if backend != "webdav" {
            return Err(Error::invalid_config("backend", "unknown backend"));
        }
        if !config.get("url").is_some_and(|u| u.starts_with("https://")) {
            return Err(Error::invalid_config("url", "url must be an https URL"));
        }
        Ok(())
    }
}

#[tokio::test]
async fn storage_moves_files_through_the_exchange_directory() {
    let mut h = start(Plugin::new().storage(MemoryStorage::default()));
    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::STORAGE]);

    let exchange = tempfile::tempdir().unwrap();
    let source = exchange.path().join("daily_1.zip");
    std::fs::write(&source, b"archive").unwrap();
    let config = HashMap::from([("url".to_owned(), "https://dav.example".to_owned())]);

    let put = h
        .call(
            method::STORAGE_PUT,
            StoragePutParams {
                backend: "webdav".into(),
                config: config.clone(),
                key: "nginx-ui/daily_1.zip".into(),
                source_path: source.to_string_lossy().into_owned(),
            },
        )
        .await
        .unwrap();
    assert_eq!(put["size"], 7);

    let list = h
        .call(
            method::STORAGE_LIST,
            StorageListParams {
                backend: "webdav".into(),
                config: config.clone(),
                prefix: "nginx-ui/daily_".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        list,
        json!({"objects": [{"key": "nginx-ui/daily_1.zip", "size": 7, "modified_at": "2026-09-21T03:00:05Z"}]})
    );

    let target = exchange.path().join("fetched.zip");
    let got = h
        .call(
            method::STORAGE_GET,
            StorageGetParams {
                backend: "webdav".into(),
                config: config.clone(),
                key: "nginx-ui/daily_1.zip".into(),
                target_path: target.to_string_lossy().into_owned(),
            },
        )
        .await
        .unwrap();
    assert_eq!(got["size"], 7);
    assert_eq!(std::fs::read(&target).unwrap(), b"archive");

    for _ in 0..2 {
        // Deleting twice succeeds, a missing object is not an error.
        h.call(
            method::STORAGE_DELETE,
            StorageDeleteParams {
                backend: "webdav".into(),
                config: config.clone(),
                key: "nginx-ui/daily_1.zip".into(),
            },
        )
        .await
        .unwrap();
    }

    let raw = h
        .call(
            method::STORAGE_LIST,
            StorageListParams {
                backend: "webdav".into(),
                config: config.clone(),
                prefix: String::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        raw,
        json!({"objects": []}),
        "an empty objects list stays an array"
    );

    let err = h
        .call(
            method::STORAGE_PUT,
            StoragePutParams {
                backend: "webdav".into(),
                key: "nginx-ui/daily_2.zip".into(),
                source_path: source.to_string_lossy().into_owned(),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("url"));
}

#[tokio::test]
async fn storage_rejects_malformed_keys() {
    let mut h = start(Plugin::new().storage(MemoryStorage::default()));
    h.initialize().await;

    for key in ["", "/abs", "a//b", "a/../b", "./a", "a/", "a\\b", "a\0b"] {
        let err = h
            .call(
                method::STORAGE_DELETE,
                StorageDeleteParams {
                    backend: "webdav".into(),
                    key: key.into(),
                    ..Default::default()
                },
            )
            .await
            .unwrap_err();
        assert_eq!(err.code, code::INVALID_PARAMS, "key {key:?}");
    }
    assert!(valid_storage_key("nginx-ui/daily_1790000000.zip.key"));
}

#[tokio::test]
async fn storage_validate() {
    let mut h = start(Plugin::new().storage(MemoryStorage::default()));
    h.initialize().await;
    let err = h
        .call(
            method::STORAGE_VALIDATE,
            StorageValidateParams {
                backend: "webdav".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, code::UNSUPPORTED);

    let mut h = start(Plugin::new().storage(ValidatingStorage(MemoryStorage::default())));
    h.initialize().await;
    let err = h
        .call(
            method::STORAGE_VALIDATE,
            StorageValidateParams {
                backend: "webdav".into(),
                config: HashMap::from([("url".to_owned(), "ftp://dav.example".to_owned())]),
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("url"));
    h.call(
        method::STORAGE_VALIDATE,
        StorageValidateParams {
            backend: "webdav".into(),
            config: HashMap::from([("url".to_owned(), "https://dav.example".to_owned())]),
        },
    )
    .await
    .unwrap();
}

#[test]
fn storage_object_without_a_time_leaves_it_out() {
    let object = StorageObject::stored("a", 1, None);
    assert_eq!(
        serde_json::to_value(&object).unwrap(),
        json!({"key": "a", "size": 1})
    );
    let time: SystemTime = UNIX_EPOCH;
    assert_eq!(
        StorageObject::stored("a", 1, Some(time)).modified_at,
        "1970-01-01T00:00:00Z"
    );
}

/// A `DeployHandler` that remembers what it was asked to push.
struct Deployer {
    pushed: mpsc::UnboundedSender<DeployRequest>,
}

pub fn deployer() -> (impl DeployHandler, mpsc::UnboundedReceiver<DeployRequest>) {
    let (pushed, rx) = mpsc::unbounded_channel();
    (Deployer { pushed }, rx)
}

#[async_trait]
impl DeployHandler for Deployer {
    async fn push(&self, _ctx: &Context, req: DeployRequest) -> Result<String, Error> {
        let zone = req.config.get("zone_id").cloned().unwrap_or_default();
        if zone.is_empty() {
            return Err(Error::invalid_config("zone_id", "zone_id is required"));
        }
        let dry_run = req.dry_run;
        let _ = self.pushed.send(req);
        Ok(if dry_run {
            format!("zone {zone} is reachable")
        } else {
            format!("uploaded to zone {zone}")
        })
    }
}

/// Also implements the validator.
struct ValidatingDeployer(Deployer);

#[async_trait]
impl DeployHandler for ValidatingDeployer {
    async fn push(&self, ctx: &Context, req: DeployRequest) -> Result<String, Error> {
        self.0.push(ctx, req).await
    }

    async fn validate(
        &self,
        _: &Context,
        _kind: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        if config.get("zone_id").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("zone_id", "zone_id is required"));
        }
        Ok(())
    }
}

#[tokio::test]
async fn deploy_push_reaches_the_handler() {
    let (d, mut pushed) = deployer();
    let mut h = start(Plugin::new().deploy(d));
    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::CERT_DEPLOY]);

    let mut params = DeployPushParams {
        kind: "mycdn".into(),
        config: HashMap::from([("zone_id".to_owned(), "zone_123".to_owned())]),
        certificate: DeployCertificate {
            name: "example.com".into(),
            domains: vec!["example.com".into()],
            certificate_pem: "LEAF".into(),
            private_key_pem: "KEY".into(),
            chain_pem: "ISSUER\n".into(),
            not_after: "2026-12-20T08:15:00Z".into(),
        },
        dry_run: false,
    };
    let result = h.call(method::DEPLOY_PUSH, &params).await.unwrap();
    assert_eq!(result["message"], "uploaded to zone zone_123");
    let got = pushed.recv().await.unwrap();
    assert_eq!(got.certificate.private_key_pem, "KEY");
    assert!(!got.dry_run);
    assert_eq!(full_chain_pem(&got.certificate), "LEAF\nISSUER\n");

    params.dry_run = true;
    let result = h.call(method::DEPLOY_PUSH, &params).await.unwrap();
    assert_eq!(result["message"], "zone zone_123 is reachable");
    assert!(pushed.recv().await.unwrap().dry_run);

    params.config = HashMap::new();
    let err = h.call(method::DEPLOY_PUSH, &params).await.unwrap_err();
    assert_eq!(err.field(), Some("zone_id"));

    let err = h
        .call(method::DEPLOY_VALIDATE, json!({"kind": "mycdn"}))
        .await
        .unwrap_err();
    assert_eq!(err.code, code::UNSUPPORTED);
}

#[tokio::test]
async fn deploy_validate_uses_the_validator() {
    let (pushed, _rx) = mpsc::unbounded_channel();
    let mut h = start(Plugin::new().deploy(ValidatingDeployer(Deployer { pushed })));
    h.initialize().await;

    let err = h
        .call(method::DEPLOY_VALIDATE, json!({"kind": "mycdn"}))
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("zone_id"));
    h.call(
        method::DEPLOY_VALIDATE,
        json!({"kind": "mycdn", "config": {"zone_id": "zone_123"}}),
    )
    .await
    .unwrap();
}

#[test]
fn shared_state_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Arc<Mutex<Vec<u8>>>>();
}
