//! Replays every vector of `plugin-spec/vectors/v1` against the SDK.
//!
//! A `host_to_plugin` vector is sent to a plugin whose handlers answer as
//! the vector says, and the reply is compared by value: results through the
//! protobuf JSON mapping of the message (so a default value and an absent
//! member are the same), errors by `code`, `data` and, where a handler
//! produced the message, `message`. A `plugin_to_host` vector drives the
//! `Host` client against a scripted host and checks the request it emits and
//! the way it takes the reply.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::task::JoinHandle;

use super::harness::{short_temp_dir, Builder, WAIT};
use crate::pb::registry::{Rpc, RPCS};
use crate::protocol::{
    self as p, code, method, EmptyResult, Error, HostLogsListResult, McpCallParams, StorageObject,
};
use crate::{
    env, run_until, BlocklistHandler, BlocklistRequest, BlocklistResult, Context, DeployHandler,
    DeployRequest, DiscoveryHandler, DiscoveryRequest, DiscoveryResult, Dns01Handler, Dns01Request,
    LogSinkEntry, LogSinkHandler, McpHandler, McpRequest, McpResult, Network, NotifyHandler,
    NotifyRequest, Options, Plugin, ProbeHandler, ProbeRequest, ProbeResult, StorageDeleteRequest,
    StorageGetRequest, StorageHandler, StorageListRequest, StoragePutRequest,
};

fn spec_dir() -> PathBuf {
    std::env::var_os("SPEC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../plugin-spec"))
}

#[derive(Clone)]
struct Vector {
    file: String,
    description: String,
    method: Option<String>,
    direction: String,
    kind: String,
    request: Value,
    request_raw: Option<String>,
    response: Option<Value>,
}

fn load_vectors() -> Option<Vec<Vector>> {
    let dir = spec_dir().join("vectors/v1");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        assert!(
            std::env::var_os("REQUIRE_SPEC").is_none(),
            "REQUIRE_SPEC is set but {} is not there",
            dir.display()
        );
        eprintln!("skipped: {} is not there, set SPEC_DIR", dir.display());
        return None;
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    Some(
        files
            .into_iter()
            .map(|path| {
                let raw: Value =
                    serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
                Vector {
                    file: path.file_name().unwrap().to_string_lossy().into_owned(),
                    description: raw["description"].as_str().unwrap_or_default().to_owned(),
                    method: raw["method"].as_str().map(str::to_owned),
                    direction: raw["direction"].as_str().unwrap().to_owned(),
                    kind: raw["kind"].as_str().unwrap().to_owned(),
                    request: raw.get("request").cloned().unwrap_or(Value::Null),
                    request_raw: raw["request_raw"].as_str().map(str::to_owned),
                    response: raw.get("response").cloned(),
                }
            })
            .collect(),
    )
}

fn rpc_of(method: &str) -> &'static Rpc {
    RPCS.iter()
        .find(|r| r.rpc_name == method)
        .unwrap_or_else(|| panic!("{method} is not an rpc of the contract"))
}

/// Maps a message to protobuf and back: defaults and unknown members vanish
/// and numbers take the form of the mapping.
fn canonical(rpc: &Rpc, is_request: bool, value: &Value) -> Value {
    let (encode, decode) = if is_request {
        (rpc.json_to_request, rpc.request_to_json)
    } else {
        (rpc.json_to_response, rpc.response_to_json)
    };
    let bytes = encode(value).unwrap_or_else(|e| panic!("{}: {e}", rpc.rpc_name));
    prune(decode(&bytes).unwrap())
}

/// An empty object means the same as an absent member (WIRE-10).
fn prune(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(k, v)| (k, prune(v)))
                .filter(|(_, v)| v.as_object().is_none_or(|o| !o.is_empty()))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(prune).collect()),
        other => other,
    }
}

// The scripted plugin.

struct Inner {
    /// The canonical params the handler has to receive.
    expected: Value,
    /// What the handler answers.
    reply: Result<Value, Error>,
    problems: Mutex<Vec<String>>,
}

#[derive(Clone)]
struct Scripted(Arc<Inner>);

impl Scripted {
    fn respond<Req: Serialize, Res: DeserializeOwned>(
        &self,
        name: &str,
        req: &Req,
    ) -> Result<Res, Error> {
        let rpc = rpc_of(name);
        let got = canonical(rpc, true, &serde_json::to_value(req).unwrap());
        if got != self.0.expected {
            self.0.problems.lock().unwrap().push(format!(
                "{name}: the handler received {got}, the vector sends {}",
                self.0.expected
            ));
        }
        match &self.0.reply {
            Ok(v) => serde_json::from_value(v.clone())
                .map_err(|e| Error::internal(format!("the vector result does not decode: {e}"))),
            Err(e) => Err(e.clone()),
        }
    }
}

#[async_trait]
impl Dns01Handler for Scripted {
    async fn present(&self, _: &Context, req: Dns01Request) -> Result<(), Error> {
        self.respond::<_, EmptyResult>(method::DNS01_PRESENT, &req)
            .map(|_| ())
    }
    async fn clean_up(&self, _: &Context, req: Dns01Request) -> Result<(), Error> {
        self.respond::<_, EmptyResult>(method::DNS01_CLEANUP, &req)
            .map(|_| ())
    }
    async fn validate(&self, _: &Context, provider: &str, config: &p::Config) -> Result<(), Error> {
        let params = p::Dns01ValidateParams {
            provider: provider.into(),
            config: config.clone(),
        };
        self.respond::<_, EmptyResult>(method::DNS01_VALIDATE, &params)
            .map(|_| ())
    }
    async fn options(
        &self,
        _: &Context,
        params: p::Dns01OptionsParams,
    ) -> Result<p::Dns01OptionsResult, Error> {
        self.respond(method::DNS01_OPTIONS, &params)
    }
    async fn check(
        &self,
        _: &Context,
        params: p::Dns01CheckParams,
    ) -> Result<p::Dns01CheckResult, Error> {
        self.respond(method::DNS01_CHECK, &params)
    }
}

/// A dns01 handler with the required methods only.
struct Bare;

#[async_trait]
impl Dns01Handler for Bare {
    async fn present(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }
    async fn clean_up(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl NotifyHandler for Scripted {
    async fn send(&self, _: &Context, req: NotifyRequest) -> Result<(), Error> {
        self.respond::<_, EmptyResult>(method::NOTIFY_SEND, &req)
            .map(|_| ())
    }
    async fn validate(&self, _: &Context, channel: &str, config: &p::Config) -> Result<(), Error> {
        let params = p::NotifyValidateParams {
            channel: channel.into(),
            config: config.clone(),
        };
        self.respond::<_, EmptyResult>(method::NOTIFY_VALIDATE, &params)
            .map(|_| ())
    }
}

#[async_trait]
impl ProbeHandler for Scripted {
    async fn check(&self, _: &Context, req: ProbeRequest) -> Result<ProbeResult, Error> {
        self.respond(method::PROBE_CHECK, &req)
    }
}

#[async_trait]
impl McpHandler for Scripted {
    async fn call(&self, _: &Context, req: McpRequest) -> Result<McpResult, Error> {
        let _: &McpCallParams = &req;
        self.respond(method::MCP_CALL, &req)
    }
}

#[async_trait]
impl StorageHandler for Scripted {
    async fn put(&self, _: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        Ok(self
            .respond::<_, p::StorageSizeResult>(method::STORAGE_PUT, &req)?
            .size
            .get())
    }
    async fn get(&self, _: &Context, req: StorageGetRequest) -> Result<u64, Error> {
        Ok(self
            .respond::<_, p::StorageSizeResult>(method::STORAGE_GET, &req)?
            .size
            .get())
    }
    async fn list(
        &self,
        _: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error> {
        Ok(self
            .respond::<_, p::StorageListResult>(method::STORAGE_LIST, &req)?
            .objects)
    }
    async fn delete(&self, _: &Context, req: StorageDeleteRequest) -> Result<(), Error> {
        self.respond::<_, EmptyResult>(method::STORAGE_DELETE, &req)
            .map(|_| ())
    }
    async fn validate(&self, _: &Context, backend: &str, config: &p::Config) -> Result<(), Error> {
        let params = p::StorageValidateParams {
            backend: backend.into(),
            config: config.clone(),
        };
        self.respond::<_, EmptyResult>(method::STORAGE_VALIDATE, &params)
            .map(|_| ())
    }
}

#[async_trait]
impl DeployHandler for Scripted {
    async fn push(&self, _: &Context, req: DeployRequest) -> Result<String, Error> {
        Ok(self
            .respond::<_, p::DeployPushResult>(method::DEPLOY_PUSH, &req)?
            .message)
    }
    async fn validate(&self, _: &Context, kind: &str, config: &p::Config) -> Result<(), Error> {
        let params = p::DeployValidateParams {
            kind: kind.into(),
            config: config.clone(),
        };
        self.respond::<_, EmptyResult>(method::DEPLOY_VALIDATE, &params)
            .map(|_| ())
    }
}

#[async_trait]
impl BlocklistHandler for Scripted {
    async fn fetch(&self, _: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        self.respond(method::BLOCKLIST_FETCH, &req)
    }
}

#[async_trait]
impl DiscoveryHandler for Scripted {
    async fn resolve(&self, _: &Context, req: DiscoveryRequest) -> Result<DiscoveryResult, Error> {
        self.respond(method::DISCOVERY_RESOLVE, &req)
    }
}

#[async_trait]
impl LogSinkHandler for Scripted {
    async fn push(&self, _: &Context, _: Vec<LogSinkEntry>) -> Result<usize, Error> {
        Ok(0)
    }
}

// The raw wire to the plugin.

struct Wire {
    host_w: tokio::io::DuplexStream,
    lines: tokio::io::Lines<BufReader<tokio::io::DuplexStream>>,
    run: Option<JoinHandle<std::io::Result<()>>>,
}

impl Wire {
    fn start(plugin: Plugin, options: Options) -> Wire {
        let (host_w, plugin_in) = tokio::io::duplex(1 << 20);
        let (plugin_out, host_r) = tokio::io::duplex(1 << 20);
        let run = tokio::spawn(run_until(
            plugin,
            plugin_in,
            plugin_out,
            options,
            std::future::pending(),
        ));
        Wire {
            host_w,
            lines: BufReader::new(host_r).lines(),
            run: Some(run),
        }
    }

    async fn send(&mut self, line: &str) {
        self.host_w.write_all(line.as_bytes()).await.unwrap();
        self.host_w.write_all(b"\n").await.unwrap();
    }

    async fn recv(&mut self) -> Value {
        let line = tokio::time::timeout(WAIT, self.lines.next_line())
            .await
            .expect("no frame arrived")
            .unwrap()
            .expect("the plugin closed its output");
        serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("stdout carries a frame that is no JSON ({e}): {line}"))
    }

    async fn handshake(&mut self, permissions: &[&str]) -> Value {
        self.send(
            &json!({
                "jsonrpc": "2.0", "id": 900, "method": "plugin.initialize",
                "params": {"host": {"version": "2.7.0", "os": "linux", "arch": "amd64", "locale": "en"},
                           "settings": {}, "permissions": permissions},
            })
            .to_string(),
        )
        .await;
        let reply = self.recv().await;
        assert_eq!(reply["id"], 900, "{reply}");
        self.send(r#"{"jsonrpc":"2.0","method":"plugin.initialized"}"#)
            .await;
        reply
    }
}

/// Codes whose message the SDK writes itself, so the message of the vector
/// is the wording of another implementation.
fn sdk_wording(error_code: i64) -> bool {
    [
        code::PARSE_ERROR,
        code::INVALID_REQUEST,
        code::METHOD_NOT_FOUND,
        code::INVALID_PARAMS,
        code::UNSUPPORTED,
    ]
    .iter()
    .any(|c| i64::from(*c) == error_code)
}

struct Fixture {
    plugin: Plugin,
    options: Options,
    dir: tempfile::TempDir,
    scripted: Scripted,
    configured: Arc<Mutex<Vec<p::Settings>>>,
    events: Arc<Mutex<Vec<p::EventNotification>>>,
}

fn fixture(v: &Vector) -> Fixture {
    let params = v.request.get("params").cloned().unwrap_or(Value::Null);
    let rpc = v
        .method
        .as_deref()
        .and_then(|m| RPCS.iter().find(|r| r.rpc_name == m));
    let expected = match rpc {
        Some(rpc) if !matches!(rpc.rpc_name, "plugin.initialize" | "plugin.configure") => {
            // A vector that expects -32602 may not decode into the message.
            if (rpc.json_to_request)(&params).is_err() {
                Value::Null
            } else {
                canonical(rpc, true, &params)
            }
        }
        _ => Value::Null,
    };
    let reply = match &v.response {
        Some(r) if r.get("error").is_some() => {
            Err(serde_json::from_value::<Error>(r["error"].clone()).unwrap())
        }
        Some(r) => Ok(r.get("result").cloned().unwrap_or(json!({}))),
        None => Ok(json!({})),
    };
    let scripted = Scripted(Arc::new(Inner {
        expected,
        reply,
        problems: Mutex::new(Vec::new()),
    }));

    let configured: Arc<Mutex<Vec<p::Settings>>> = Arc::default();
    let events: Arc<Mutex<Vec<p::EventNotification>>> = Arc::default();
    let (c, e) = (configured.clone(), events.clone());

    let dir = short_temp_dir();
    let expects_unsupported = v
        .response
        .as_ref()
        .is_some_and(|r| r["error"]["code"] == code::UNSUPPORTED);
    let mut plugin = Plugin::new();
    if v.method.as_deref() == Some("plugin.initialize") {
        // The plugin serves what the vector says it does.
        let expected_caps = v.response.as_ref().unwrap()["result"]["capabilities"].clone();
        if expected_caps == json!(["http"]) {
            plugin = plugin.http(|_req: http::Request<hyper::body::Incoming>| async {
                crate::http::text_response(http::StatusCode::OK, "ok")
            });
        } else {
            plugin = plugin.dns01(scripted.clone());
        }
    } else {
        plugin = if expects_unsupported {
            plugin.dns01(Bare)
        } else {
            plugin.dns01(scripted.clone())
        };
        plugin = plugin
            .notify(scripted.clone())
            .probe(scripted.clone())
            .mcp(scripted.clone())
            .storage(scripted.clone())
            .deploy(scripted.clone())
            .blocklist(scripted.clone())
            .discovery(scripted.clone())
            .log_sink(scripted.clone());
    }
    plugin = plugin
        .configure(move |_ctx, settings| {
            let c = c.clone();
            async move {
                c.lock().unwrap().push(settings);
                Ok(())
            }
        })
        .event(p::event::LOG_PATHS_CHANGED, move |_ctx, ev| {
            let e = e.clone();
            async move {
                e.lock().unwrap().push(ev);
            }
        });

    // The initialize vectors show the transports the plugin serves.
    let with_grpc = v.method.as_deref() == Some("plugin.initialize")
        && v.response.as_ref().is_some_and(|r| {
            r["result"]["transports"]
                .as_array()
                .is_some_and(|t| t.len() > 1)
        });
    let mut options = Options::new()
        .with_env([
            (
                env::PLUGIN_DATA_DIR.to_owned(),
                dir.path().to_string_lossy().into_owned(),
            ),
            (
                env::PLUGIN_HTTP_SECRET.to_owned(),
                "vector-secret".to_owned(),
            ),
        ])
        .with_http_network(Network::Tcp);
    if !with_grpc {
        options = options.without_grpc();
    }
    Fixture {
        plugin,
        options,
        dir,
        scripted,
        configured,
        events,
    }
}

async fn replay_host_to_plugin(v: &Vector) {
    let Fixture {
        plugin,
        options,
        dir,
        scripted,
        configured,
        events,
    } = fixture(v);
    let mut wire = Wire::start(plugin, options);
    let is_initialize = v.method.as_deref() == Some("plugin.initialize");
    if !is_initialize {
        wire.handshake(&[]).await;
    }

    let line = match &v.request_raw {
        Some(raw) => raw.clone(),
        None => v.request.to_string(),
    };
    wire.send(&line).await;

    if v.kind == "notification" {
        // Nothing may come back, and the connection stays usable.
        if v.method.as_deref() == Some("plugin.exit") {
            let run = wire.run.take().unwrap();
            tokio::time::timeout(WAIT, run)
                .await
                .expect("plugin.exit did not end the run")
                .unwrap()
                .unwrap();
            return;
        }
        wire.send(r#"{"jsonrpc":"2.0","id":901,"method":"plugin.ping"}"#)
            .await;
        let next = wire.recv().await;
        assert_eq!(next["id"], 901, "a notification was answered: {next}");
        if v.method.as_deref() == Some("events.on") {
            tokio::time::sleep(Duration::from_millis(50)).await;
            let got = events.lock().unwrap().clone();
            assert_eq!(got.len(), 1, "the event was not delivered");
            assert_eq!(got[0].r#type, v.request["params"]["type"]);
            assert_eq!(json!(got[0].ts), v.request["params"]["ts"]);
        }
        return;
    }

    let want = v
        .response
        .as_ref()
        .expect("a request vector has a response");
    let reply = wire.recv().await;
    assert_eq!(reply["jsonrpc"], "2.0", "{reply}");
    assert_eq!(reply["id"], want["id"], "the id is echoed: {reply}");

    if let Some(want_error) = want.get("error") {
        let got = reply
            .get("error")
            .unwrap_or_else(|| panic!("want an error, got {reply}"));
        assert_eq!(got["code"], want_error["code"], "error code: {reply}");
        if let Some(data) = want_error.get("data").filter(|d| !d.is_null()) {
            assert_eq!(got.get("data"), Some(data), "error data");
        }
        if !sdk_wording(want_error["code"].as_i64().unwrap()) {
            assert_eq!(got["message"], want_error["message"], "error message");
        }
        assert!(
            !got["message"].as_str().unwrap_or_default().is_empty(),
            "an error has a message"
        );
    } else {
        assert!(reply.get("error").is_none(), "want a result, got {reply}");
        let mut got = reply["result"].clone();
        let mut expected = want["result"].clone();
        let name = v.method.as_deref().unwrap();
        let rpc = rpc_of(name);

        if is_initialize {
            // Values that depend on the machine.
            for key in ["rpc_socket", "http_port"] {
                if expected.get(key).is_some() {
                    let value = &got[key];
                    assert!(
                        value.as_str().is_some_and(|s| s.ends_with("rpc.sock"))
                            || value.as_i64().is_some_and(|n| n > 0),
                        "{key} = {value}"
                    );
                    expected.as_object_mut().unwrap().remove(key);
                    got.as_object_mut().unwrap().remove(key);
                }
            }
            // Stdio is always served, the SDK says so.
            if expected.get("transports").is_none() && got["transports"] == json!(["stdio"]) {
                got.as_object_mut().unwrap().remove("transports");
            }
            if got.get("rpc_socket").is_some() {
                let socket = got["rpc_socket"].as_str().unwrap();
                assert!(
                    socket.starts_with(dir.path().to_string_lossy().as_ref()),
                    "{socket}"
                );
            }
        }
        assert_eq!(
            canonical(rpc, false, &got),
            canonical(rpc, false, &expected),
            "result of {name}"
        );
    }

    if let Some(method_name) = &v.method {
        if method_name == "plugin.configure" {
            let expected = &v.request["params"]["settings"];
            let got = configured.lock().unwrap().clone();
            assert_eq!(got.len(), 1);
            assert_eq!(&Value::Object(got[0].clone()), expected);
        }
    }
    let problems = scripted.0.problems.lock().unwrap().clone();
    assert!(problems.is_empty(), "{problems:?}");
}

async fn replay_plugin_to_host(v: &Vector) {
    let name = v.method.clone().unwrap();
    let params = v.request.get("params").cloned().unwrap_or(Value::Null);
    let want = v.response.clone().unwrap();
    let rpc = rpc_of(&name);

    let outcome: Arc<Mutex<Option<Result<Value, crate::HostError>>>> = Arc::default();
    let slot = outcome.clone();
    let call_params = params.clone();
    let call_name = name.clone();
    let plugin = Plugin::new().method("test.call", move |ctx: Context, _p| {
        let slot = slot.clone();
        let params = call_params.clone();
        let name = call_name.clone();
        async move {
            let host = ctx.host();
            let out: Result<Value, crate::HostError> = match name.as_str() {
                "host.kv.set" => host
                    .kv_set(params["key"].as_str().unwrap(), params["value"].clone())
                    .await
                    .map(|()| json!({})),
                "host.kv.get" => host
                    .kv_get_value(params["key"].as_str().unwrap())
                    .await
                    .map(|v| json!({"value": v, "found": true})),
                "host.logs.list" => host
                    .logs_list()
                    .await
                    .map(|logs| serde_json::to_value(HostLogsListResult { logs }).unwrap()),
                "host.activity.set" => host
                    .activity_set(
                        params["key"].as_str().unwrap(),
                        params["label"].as_str().unwrap(),
                        params["active"].as_bool().unwrap(),
                    )
                    .await
                    .map(|()| json!({})),
                other => panic!("{other}: add the call of this vector to the table"),
            };
            *slot.lock().unwrap() = Some(out);
            Ok(json!({}))
        }
    });
    let mut h = Builder::new(plugin).options(Options::without_grpc).start();

    let received: Arc<Mutex<Option<Value>>> = Arc::default();
    let seen = received.clone();
    let reply = want.clone();
    h.host.handle(
        name.clone(),
        crate::jsonrpc::handler(move |params| {
            let seen = seen.clone();
            let reply = reply.clone();
            async move {
                *seen.lock().unwrap() = Some(params);
                match reply.get("error") {
                    Some(e) => Err(serde_json::from_value::<Error>(e.clone()).unwrap()),
                    None => Ok(reply.get("result").cloned().unwrap_or(json!({}))),
                }
            }
        }),
    );
    h.initialize().await;
    h.call("test.call", ()).await.unwrap();

    // The request the Host client emitted.
    let got = received.lock().unwrap().clone().unwrap_or(Value::Null);
    let request_params = |p: &Value| {
        if p.is_null() {
            json!({})
        } else {
            canonical(rpc, true, p)
        }
    };
    assert_eq!(
        request_params(&got),
        request_params(&params),
        "params of {name}"
    );

    // The way it took the reply.
    let result = outcome
        .lock()
        .unwrap()
        .take()
        .expect("the call did not finish");
    match want.get("error") {
        Some(want_error) => {
            let err = result.expect_err("want an error");
            let rpc_error = err
                .rpc_error()
                .unwrap_or_else(|| panic!("want an rpc error, got {err}"));
            assert_eq!(json!(rpc_error.code), want_error["code"]);
            assert_eq!(json!(rpc_error.message), want_error["message"]);
        }
        None => {
            let value = result.unwrap_or_else(|e| panic!("{name}: {e}"));
            if name == "host.logs.list" {
                assert_eq!(
                    canonical(rpc, false, &value),
                    canonical(rpc, false, &want["result"])
                );
            }
        }
    }
}

#[tokio::test]
async fn every_vector_replays() {
    let Some(vectors) = load_vectors() else {
        return;
    };
    assert!(!vectors.is_empty());

    let mut failures = Vec::new();
    for v in &vectors {
        let owned = v.clone();
        let run = async move {
            match owned.direction.as_str() {
                "host_to_plugin" => replay_host_to_plugin(&owned).await,
                "plugin_to_host" => replay_plugin_to_host(&owned).await,
                other => panic!("unknown direction {other}"),
            }
        };
        let result = tokio::spawn(run).await;
        if let Err(e) = result {
            let message = e.into_panic();
            let text = message
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| message.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_default();
            failures.push(format!("{} ({}): {text}", v.file, v.description));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} vectors failed:\n{}",
        failures.len(),
        vectors.len(),
        failures.join("\n")
    );
    eprintln!("replayed {} vectors", vectors.len());
}

/// `v.kind` and the shape are known for every vector.
#[test]
fn vectors_have_a_known_shape() {
    let Some(vectors) = load_vectors() else {
        return;
    };
    let mut by_kind: HashMap<String, usize> = HashMap::new();
    for v in &vectors {
        assert!(
            ["request", "notification"].contains(&v.kind.as_str()),
            "{}",
            v.file
        );
        *by_kind
            .entry(format!("{}/{}", v.direction, v.kind))
            .or_default() += 1;
    }
    assert!(by_kind.contains_key("host_to_plugin/request"));
}
