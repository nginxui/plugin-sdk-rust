use async_trait::async_trait;
use serde_json::json;

use super::harness::start;
use crate::protocol::{
    capability, code, method, BlocklistEntry, BlocklistFetchParams, DiscoveryResolveParams,
    DiscoveryTarget,
};
use crate::{
    unknown_service, BlocklistHandler, BlocklistRequest, BlocklistResult, Context,
    DiscoveryHandler, DiscoveryRequest, DiscoveryResult, Error, Plugin,
};

/// A `BlocklistHandler` with a fixed list.
pub struct Feed;

#[async_trait]
impl BlocklistHandler for Feed {
    async fn fetch(&self, _ctx: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        match req.config.get("api_key").map(String::as_str) {
            None | Some("") => Err(Error::invalid_config("api_key", "api_key is required")),
            Some("empty") => Ok(BlocklistResult::default()),
            Some(_) => Ok(BlocklistResult {
                entries: vec![
                    BlocklistEntry::deny("198.51.100.23", "SSH brute force"),
                    BlocklistEntry::deny("2001:db8:bad::/48", ""),
                ],
                ttl_seconds: 900,
            }),
        }
    }
}

/// A `DiscoveryHandler` that knows one service.
pub struct Registry;

#[async_trait]
impl DiscoveryHandler for Registry {
    async fn resolve(
        &self,
        _ctx: &Context,
        req: DiscoveryRequest,
    ) -> Result<DiscoveryResult, Error> {
        if req.config.get("address").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("address", "address is required"));
        }
        match req.service.as_str() {
            "api" => Ok(DiscoveryResult {
                targets: vec![
                    DiscoveryTarget::new("10.0.1.12", 8080).with_tags(["zone-a"]),
                    DiscoveryTarget {
                        address: "10.0.2.7".into(),
                        port: 8080,
                        weight: 2,
                        tags: Vec::new(),
                    },
                ],
                ttl_seconds: 30,
            }),
            "idle" => Ok(DiscoveryResult::default()),
            other => Err(unknown_service(other)),
        }
    }
}

#[tokio::test]
async fn blocklist_fetch_reaches_the_handler() {
    let mut h = start(Plugin::new().blocklist(Feed));

    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::SECURITY_BLOCKLIST]);

    let mut params = BlocklistFetchParams {
        source: "threatfeed".into(),
        config: [("api_key".to_owned(), "k".to_owned())].into(),
    };
    let result = h.call(method::BLOCKLIST_FETCH, &params).await.unwrap();
    assert_eq!(
        result,
        json!({
            "entries": [
                {"cidr": "198.51.100.23", "reason": "SSH brute force"},
                {"cidr": "2001:db8:bad::/48"},
            ],
            "ttl_seconds": 900,
        })
    );

    // An empty list is sent as an array.
    params.config.insert("api_key".into(), "empty".into());
    let raw = h.call(method::BLOCKLIST_FETCH, &params).await.unwrap();
    assert_eq!(raw, json!({"entries": []}));

    let err = h
        .call(
            method::BLOCKLIST_FETCH,
            BlocklistFetchParams {
                source: "threatfeed".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("api_key"));

    // A malformed payload is invalid params, not a handler call.
    let err = h
        .call(method::BLOCKLIST_FETCH, "threatfeed")
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INVALID_PARAMS);
}

#[tokio::test]
async fn discovery_resolve_reaches_the_handler() {
    let mut h = start(Plugin::new().discovery(Registry));

    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::UPSTREAM_DISCOVERY]);

    let config: std::collections::HashMap<String, String> = [(
        "address".to_owned(),
        "https://registry.example.com".to_owned(),
    )]
    .into();
    let resolve =
        |service: &str, config: std::collections::HashMap<String, String>| DiscoveryResolveParams {
            provider: "registry".into(),
            config,
            service: service.into(),
        };

    let result = h
        .call(method::DISCOVERY_RESOLVE, resolve("api", config.clone()))
        .await
        .unwrap();
    assert_eq!(
        result,
        json!({
            "targets": [
                {"address": "10.0.1.12", "port": 8080, "weight": 1, "tags": ["zone-a"]},
                {"address": "10.0.2.7", "port": 8080, "weight": 2},
            ],
            "ttl_seconds": 30,
        })
    );

    let raw = h
        .call(method::DISCOVERY_RESOLVE, resolve("idle", config.clone()))
        .await
        .unwrap();
    assert_eq!(raw, json!({"targets": []}));

    let err = h
        .call(method::DISCOVERY_RESOLVE, resolve("billing", config))
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("service"));
    let err = h
        .call(
            method::DISCOVERY_RESOLVE,
            resolve("api", Default::default()),
        )
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("address"));
}
