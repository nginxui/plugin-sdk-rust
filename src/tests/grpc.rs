use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;
use prost::Message;
use serde_json::{json, Value};
use tokio::sync::mpsc;

use super::clients::{self, Grpc};
use super::harness::{short_temp_dir, Builder, WAIT};
use crate::grpc::{MAX_GRPC_MESSAGE_BYTES, RPC_SOCKET_NAME};
use crate::pb::v1 as pb;
use crate::pb::wkt;
use crate::protocol::{
    code, log_format, method, transport, BlocklistEntry, DiscoveryTarget, Error, EventNotification,
    InitializeResult,
};
use crate::{
    env, BlocklistHandler, BlocklistRequest, BlocklistResult, Context, DiscoveryHandler,
    DiscoveryRequest, DiscoveryResult, Dns01Handler, Dns01Request, McpResult, McpTools, Network,
    Plugin, StorageDeleteRequest, StorageGetRequest, StorageHandler, StorageListRequest,
    StoragePutRequest,
};

/// Implements every dns01 method for the gRPC tests.
pub struct GrpcHandler;

#[async_trait]
impl Dns01Handler for GrpcHandler {
    async fn present(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }

    async fn clean_up(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }

    async fn validate(
        &self,
        _: &Context,
        _: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        if config.get("TOKEN").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("TOKEN", "TOKEN is required"));
        }
        Ok(())
    }

    async fn options(
        &self,
        _: &Context,
        p: crate::protocol::Dns01OptionsParams,
    ) -> Result<crate::protocol::Dns01OptionsResult, Error> {
        if p.provider == "panic" {
            panic!("boom");
        }
        Ok(crate::protocol::Dns01OptionsResult {
            propagation_timeout_seconds: 120,
            polling_interval_seconds: 2,
            ..Default::default()
        })
    }
}

/// Starts a plugin with the gRPC transport over `network` and completes the
/// handshake.
pub async fn grpc_harness(
    plugin: Plugin,
    dir: &tempfile::TempDir,
    network: Network,
) -> super::harness::Harness {
    let mut h = Builder::new(plugin)
        .data_dir(dir.path())
        .options(|o| o.with_grpc_network(network))
        .start();
    h.initialize().await;
    h
}

fn detail(err: &clients::GrpcError) -> &pb::PluginError {
    err.detail
        .as_ref()
        .unwrap_or_else(|| panic!("status {err:?} has no PluginError detail"))
}

const DNS01: &str = "/nginxui.plugin.v1.DNS01";

#[tokio::test]
async fn grpc_serves_the_stdio_handlers() {
    let dir = short_temp_dir();
    let (events_tx, mut events) = mpsc::unbounded_channel::<String>();
    let plugin = Plugin::new()
        .dns01(GrpcHandler)
        .method(method::EVENTS_ON, move |_ctx, params| {
            let tx = events_tx.clone();
            async move {
                let ev: EventNotification = serde_json::from_value(params)?;
                let _ = tx.send(ev.r#type);
                Ok(Value::Null)
            }
        });
    let mut h = grpc_harness(plugin, &dir, Network::Unix).await;

    assert_eq!(h.init.transports, [transport::STDIO, transport::GRPC]);
    let want_socket = std::path::absolute(dir.path().join(RPC_SOCKET_NAME)).unwrap();
    assert_eq!(h.init.rpc_socket, want_socket.to_string_lossy());
    assert_eq!(
        (h.init.rpc_port, h.init.rpc_token.as_str()),
        (0, ""),
        "a Unix socket reports no port or token"
    );

    let grpc = Grpc::connect(&h.init).await;

    // A result travels as the response message.
    let out = grpc
        .call(
            &format!("{DNS01}/Options"),
            &pb::Dns01OptionsRequest {
                provider: "demo".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap();
    let options = pb::Dns01OptionsResponse::decode(out.as_slice()).unwrap();
    assert_eq!(
        (
            options.propagation_timeout_seconds,
            options.polling_interval_seconds
        ),
        (120, 2)
    );

    // A JSON-RPC error travels as a status with the PluginError detail.
    let err = grpc
        .call(
            &format!("{DNS01}/Validate"),
            &pb::Dns01ValidateRequest {
                provider: "demo".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, clients::INVALID_ARGUMENT);
    assert_eq!(err.message, "TOKEN is required");
    assert_eq!(detail(&err).code, code::INVALID_CONFIG);
    assert_eq!(
        Value::Object(detail(&err).data.clone().unwrap().into())["field"],
        "TOKEN"
    );

    // A panic is an internal error, as on stdio.
    let err = grpc
        .call(
            &format!("{DNS01}/Options"),
            &pb::Dns01OptionsRequest {
                provider: "panic".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::INTERNAL, code::INTERNAL_ERROR)
    );

    // plugin.ping is served so the host can probe the channel.
    grpc.call("/nginxui.plugin.v1.Plugin/Ping", &[])
        .await
        .unwrap();

    // The handshake and the stop sequence stay on stdio.
    let err = grpc
        .call("/nginxui.plugin.v1.Plugin/Exit", &[])
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::UNIMPLEMENTED, code::METHOD_NOT_FOUND)
    );

    // A method outside the contract is unknown.
    let err = grpc
        .call(&format!("{DNS01}/DoesNotExist"), &[])
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::UNIMPLEMENTED, code::METHOD_NOT_FOUND)
    );

    // A contract rpc without a handler answers like the stdio dispatcher.
    let err = grpc
        .call("/nginxui.plugin.v1.HTTP/Handle", &[])
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::UNIMPLEMENTED, code::METHOD_NOT_FOUND)
    );
    assert_eq!(detail(&err).message, "unknown method: http.handle");

    // Bytes that are no valid request message are invalid params.
    let err = grpc
        .call(&format!("{DNS01}/Options"), &[0xff, 0xff, 0xff])
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::INVALID_ARGUMENT, code::INVALID_PARAMS)
    );

    // A notification rpc returns at once and still reaches its handler.
    grpc.call(
        "/nginxui.plugin.v1.Events/On",
        &pb::EventsOnRequest {
            r#type: "cert.renewed".into(),
            ..Default::default()
        }
        .encode_to_vec(),
    )
    .await
    .unwrap();
    let got = tokio::time::timeout(WAIT, events.recv())
        .await
        .expect("events.on did not run");
    assert_eq!(got.as_deref(), Some("cert.renewed"));

    let socket = PathBuf::from(&h.init.rpc_socket);
    h.stop().await;
    assert!(!socket.exists(), "the socket still exists after stop");
}

#[tokio::test]
async fn grpc_serves_mcp_calls() {
    let dir = short_temp_dir();
    let tools = McpTools::new().tool("echo", |_ctx, args| async move {
        let paths = args
            .get("paths")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);
        let zone = args
            .get("zone")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        Ok(McpResult::text(format!("{zone}:{paths}")))
    });
    let mut h = grpc_harness(Plugin::new().mcp(tools), &dir, Network::Unix).await;
    let grpc = Grpc::connect(&h.init).await;

    // The Struct arguments reach the tool as a JSON object and the content
    // list comes back as repeated messages.
    let args = json!({"zone": "example.com", "paths": ["/a", "/b"]});
    let request = pb::McpCallRequest {
        tool: "echo".into(),
        arguments: Some(wkt::Struct::from(args.as_object().unwrap().clone())),
    };
    let out = grpc
        .call("/nginxui.plugin.v1.MCP/Call", &request.encode_to_vec())
        .await
        .unwrap();
    let result = pb::McpCallResponse::decode(out.as_slice()).unwrap();
    assert_eq!(result.content.len(), 1);
    assert_eq!(result.content[0].text, "example.com:2");
    assert!(!result.is_error);

    // An unknown tool is invalid params on gRPC as on stdio.
    let err = grpc
        .call(
            "/nginxui.plugin.v1.MCP/Call",
            &pb::McpCallRequest {
                tool: "missing".into(),
                arguments: None,
            }
            .encode_to_vec(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::INVALID_ARGUMENT, code::INVALID_PARAMS)
    );
    h.stop().await;
}

/// Lists one object past 4 GiB and stores nothing.
struct BigStorage;

#[async_trait]
impl StorageHandler for BigStorage {
    async fn put(&self, _: &Context, _: StoragePutRequest) -> Result<u64, Error> {
        Ok(6 << 30)
    }
    async fn get(&self, _: &Context, _: StorageGetRequest) -> Result<u64, Error> {
        Err(Error::internal("empty"))
    }
    async fn delete(&self, _: &Context, _: StorageDeleteRequest) -> Result<(), Error> {
        Ok(())
    }
    async fn list(
        &self,
        _: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<crate::protocol::StorageObject>, Error> {
        Ok(vec![crate::protocol::StorageObject::stored(
            format!("{}big.zip", req.prefix),
            5 << 30,
            None,
        )])
    }
}

#[tokio::test]
async fn grpc_carries_storage_sizes_as_doubles() {
    let dir = short_temp_dir();
    let mut h = grpc_harness(Plugin::new().storage(BigStorage), &dir, Network::Unix).await;
    let grpc = Grpc::connect(&h.init).await;

    let out = grpc
        .call(
            "/nginxui.plugin.v1.Storage/List",
            &pb::StorageListRequest {
                backend: "webdav".into(),
                prefix: "b/".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap();
    let list = pb::StorageListResponse::decode(out.as_slice()).unwrap();
    assert_eq!(list.objects.len(), 1);
    assert_eq!(list.objects[0].key, "b/big.zip");
    assert!((list.objects[0].size - (5u64 << 30) as f64).abs() < 0.5);

    let out = grpc
        .call(
            "/nginxui.plugin.v1.Storage/Put",
            &pb::StoragePutRequest {
                backend: "webdav".into(),
                key: "b/big.zip".into(),
                source_path: "/x".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap();
    assert!(
        (pb::StoragePutResponse::decode(out.as_slice()).unwrap().size - (6u64 << 30) as f64).abs()
            < 0.5
    );

    // A malformed key is invalid params on gRPC as on stdio.
    let err = grpc
        .call(
            "/nginxui.plugin.v1.Storage/Delete",
            &pb::StorageDeleteRequest {
                backend: "webdav".into(),
                key: "../x".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        (err.code, detail(&err).code),
        (clients::INVALID_ARGUMENT, code::INVALID_PARAMS)
    );
    h.stop().await;
}

struct ListFeed;

#[async_trait]
impl BlocklistHandler for ListFeed {
    async fn fetch(&self, _: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        if req.config.get("api_key").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("api_key", "api_key is required"));
        }
        Ok(BlocklistResult {
            entries: vec![BlocklistEntry::deny("203.0.113.0/24", "botnet")],
            ttl_seconds: 60,
        })
    }
}

struct OneService;

#[async_trait]
impl DiscoveryHandler for OneService {
    async fn resolve(&self, _: &Context, _: DiscoveryRequest) -> Result<DiscoveryResult, Error> {
        Ok(DiscoveryResult {
            targets: vec![DiscoveryTarget::new("10.0.0.5", 8080)],
            ttl_seconds: 0,
        })
    }
}

#[tokio::test]
async fn grpc_serves_blocklist_and_discovery_calls() {
    let dir = short_temp_dir();
    let plugin = Plugin::new().blocklist(ListFeed).discovery(OneService);
    let mut h = grpc_harness(plugin, &dir, Network::Unix).await;
    let grpc = Grpc::connect(&h.init).await;

    let out = grpc
        .call(
            "/nginxui.plugin.v1.Blocklist/Fetch",
            &pb::BlocklistFetchRequest {
                source: "threatfeed".into(),
                config: [("api_key".to_owned(), "k".to_owned())].into(),
            }
            .encode_to_vec(),
        )
        .await
        .unwrap();
    let fetched = pb::BlocklistFetchResponse::decode(out.as_slice()).unwrap();
    assert_eq!(fetched.entries.len(), 1);
    assert_eq!(fetched.entries[0].cidr, "203.0.113.0/24");
    assert_eq!(fetched.ttl_seconds, 60);

    let err = grpc
        .call(
            "/nginxui.plugin.v1.Blocklist/Fetch",
            &pb::BlocklistFetchRequest {
                source: "threatfeed".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap_err();
    assert_eq!(detail(&err).code, code::INVALID_CONFIG);

    let out = grpc
        .call(
            "/nginxui.plugin.v1.Discovery/Resolve",
            &pb::DiscoveryResolveRequest {
                provider: "registry".into(),
                service: "api".into(),
                ..Default::default()
            }
            .encode_to_vec(),
        )
        .await
        .unwrap();
    let resolved = pb::DiscoveryResolveResponse::decode(out.as_slice()).unwrap();
    assert_eq!(
        (resolved.targets[0].port, resolved.targets[0].weight),
        (8080, 1)
    );
    h.stop().await;
}

#[cfg(unix)]
#[tokio::test]
async fn grpc_socket_falls_back_for_a_long_data_dir() {
    let base = short_temp_dir();
    let data_dir = base.path().join("d".repeat(120));
    let mut h = Builder::new(Plugin::new().dns01(GrpcHandler))
        .var(env::PLUGIN_DATA_DIR, data_dir.to_string_lossy())
        .options(|o| o.with_grpc_network(Network::Unix))
        .start();
    let init = h.initialize().await;

    assert!(!init.rpc_socket.is_empty());
    assert!(
        !init
            .rpc_socket
            .starts_with(data_dir.to_string_lossy().as_ref()),
        "{}",
        init.rpc_socket
    );
    assert!(
        init.rpc_socket.len() <= crate::util::max_socket_path_len(),
        "{}",
        init.rpc_socket
    );

    let grpc = Grpc::connect(&init).await;
    grpc.call("/nginxui.plugin.v1.Plugin/Ping", &[])
        .await
        .unwrap();

    h.stop().await;
    let fallback_dir = PathBuf::from(&init.rpc_socket).parent().unwrap().to_owned();
    assert!(
        !fallback_dir.exists(),
        "the fallback directory still exists after stop"
    );
}

#[tokio::test]
async fn grpc_over_tcp_requires_the_token() {
    let dir = short_temp_dir();
    let mut h = grpc_harness(Plugin::new().dns01(GrpcHandler), &dir, Network::Tcp).await;
    let init: &InitializeResult = &h.init;
    assert!(
        init.rpc_port != 0 && init.rpc_token.len() >= 32 && init.rpc_socket.is_empty(),
        "{init:?}"
    );

    let grpc = Grpc::connect(&h.init).await;
    let ping = "/nginxui.plugin.v1.Plugin/Ping";

    for (name, authorization) in [
        ("no token", None),
        ("wrong token", Some("Bearer nope".to_owned())),
        ("no bearer", Some(h.init.rpc_token.clone())),
    ] {
        let err = grpc
            .call_with_authorization(ping, &[], authorization.as_deref())
            .await
            .unwrap_err();
        assert_eq!(err.code, clients::UNAUTHENTICATED, "{name}");
        assert_eq!(detail(&err).code, code::PERMISSION_DENIED, "{name}");
    }

    grpc.call(ping, &[]).await.unwrap();
    h.stop().await;
}

#[cfg(windows)]
#[tokio::test]
async fn grpc_over_a_pipe_requires_the_token() {
    let dir = short_temp_dir();
    let mut h = grpc_harness(Plugin::new().dns01(GrpcHandler), &dir, Network::Pipe).await;
    let init: &InitializeResult = &h.init;
    assert!(
        init.rpc_pipe.starts_with(r"\\.\pipe\")
            && init.rpc_token.len() >= 32
            && init.rpc_port == 0
            && init.rpc_socket.is_empty(),
        "{init:?}"
    );
    let pipe = init.rpc_pipe.clone();

    let grpc = Grpc::connect(&h.init).await;
    let ping = "/nginxui.plugin.v1.Plugin/Ping";
    let err = grpc
        .call_with_authorization(ping, &[], None)
        .await
        .unwrap_err();
    assert_eq!(err.code, clients::UNAUTHENTICATED);
    grpc.call(ping, &[]).await.unwrap();

    // Two clients at once each get an instance of their own.
    let second = Grpc::connect(&h.init).await;
    second.call(ping, &[]).await.unwrap();

    h.stop().await;
    drop((grpc, second));
    let gone = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        clients::connect_pipe(&pipe),
    )
    .await;
    assert!(
        matches!(gone, Ok(Err(_))),
        "the pipe still accepts connections after stop"
    );
}

fn assert_stdio_only(init: &InitializeResult, dir: &tempfile::TempDir) {
    assert_eq!(init.transports, [transport::STDIO]);
    assert!(
        init.rpc_socket.is_empty() && init.rpc_port == 0 && init.rpc_token.is_empty(),
        "{init:?}"
    );
    assert!(
        !dir.path().join(RPC_SOCKET_NAME).exists(),
        "a socket exists although gRPC is disabled"
    );
}

#[tokio::test]
async fn grpc_can_be_disabled() {
    // By option.
    let dir = short_temp_dir();
    let mut h = Builder::new(Plugin::new().dns01(GrpcHandler))
        .data_dir(dir.path())
        .options(crate::Options::without_grpc)
        .start();
    let init = h.initialize().await;
    assert_stdio_only(&init, &dir);

    // By environment.
    let dir = short_temp_dir();
    let mut h = Builder::new(Plugin::new().dns01(GrpcHandler))
        .data_dir(dir.path())
        .var(env::DISABLE_GRPC, "1")
        .start();
    let init = h.initialize().await;
    assert_stdio_only(&init, &dir);
}

#[cfg(unix)]
#[tokio::test]
async fn a_failing_grpc_listener_falls_back_to_stdio() {
    // The data directory is below a file, and the fallback directory does not
    // exist: no socket can be opened.
    let dir = short_temp_dir();
    let file = dir.path().join("file");
    std::fs::write(&file, b"x").unwrap();
    let mut h = Builder::new(Plugin::new().dns01(GrpcHandler))
        .data_dir(&file.join("data"))
        .options(|o| {
            o.with_grpc_network(Network::Unix)
                .with_grpc_fallback_dirs(vec![dir.path().join("missing")])
        })
        .start();
    let init = h.initialize().await;

    assert_eq!(init.transports, [transport::STDIO]);
    assert!(
        init.rpc_socket.is_empty() && init.rpc_port == 0 && init.rpc_token.is_empty(),
        "{init:?}"
    );
    // The plugin still serves stdio.
    h.call(method::PING, ()).await.unwrap();
    h.stop().await;
}

#[test]
fn message_limit_is_larger_than_a_stdio_frame() {
    const { assert!(MAX_GRPC_MESSAGE_BYTES > crate::jsonrpc::MAX_MESSAGE_BYTES) };
    assert_eq!(log_format::COMBINED, "combined");
}

#[tokio::test]
async fn grpc_answers_a_deadline() {
    struct Sleepy;

    #[async_trait]
    impl Dns01Handler for Sleepy {
        async fn present(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            Ok(())
        }
        async fn clean_up(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
            Ok(())
        }
    }

    let dir = short_temp_dir();
    let mut h = grpc_harness(Plugin::new().dns01(Sleepy), &dir, Network::Unix).await;
    let grpc = Grpc::connect(&h.init).await;
    let started = std::time::Instant::now();
    let err = grpc
        .call_with_timeout(
            "/nginxui.plugin.v1.DNS01/Present",
            &pb::Dns01PresentRequest::default().encode_to_vec(),
            "200m",
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, 4, "DEADLINE_EXCEEDED");
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
    h.stop().await;
}

#[tokio::test]
async fn grpc_carries_messages_past_the_stdio_limit() {
    let dir = short_temp_dir();
    let tools = McpTools::new().tool("big", |_ctx, args| async move {
        let echoed = args.get("blob").and_then(Value::as_str).map_or(0, str::len);
        Ok(McpResult::text(format!("{echoed}:{}", "y".repeat(6 << 20))))
    });
    let mut h = grpc_harness(Plugin::new().mcp(tools), &dir, Network::Unix).await;
    let grpc = Grpc::connect(&h.init).await;

    // 6 MiB in and 6 MiB out, more than a stdio frame holds.
    let args = json!({"blob": "x".repeat(6 << 20)});
    let request = pb::McpCallRequest {
        tool: "big".into(),
        arguments: Some(wkt::Struct::from(args.as_object().unwrap().clone())),
    };
    let out = grpc
        .call("/nginxui.plugin.v1.MCP/Call", &request.encode_to_vec())
        .await
        .unwrap();
    let result = pb::McpCallResponse::decode(out.as_slice()).unwrap();
    assert!(result.content[0].text.starts_with("6291456:"));
    assert_eq!(result.content[0].text.len(), "6291456:".len() + (6 << 20));

    // The same result cannot leave on stdio, the caller gets an error and not
    // silence.
    let err = h
        .call(method::MCP_CALL, json!({"tool": "big", "arguments": {}}))
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INTERNAL_ERROR);
    h.stop().await;
}
