use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};

use async_trait::async_trait;
use prost::Message;
use serde_json::{json, Value};

use super::clients::{self, Grpc};
use super::grpc::{grpc_harness, GrpcHandler};
use super::harness::{short_temp_dir, Builder};
use crate::pb::v1 as pb;
use crate::protocol::{capability, code, log_format, method, transport, LogSinkPushParams};
use crate::{
    env, Context, Error, LogSinkEntry, LogSinkHandler, Network, Options, Plugin, MAX_LOG_SINK_BATCH,
};

const PUSH: &str = "/nginxui.plugin.v1.LogSink/Push";

/// Keeps every batch and accepts all but the entries with status 500.
#[derive(Default)]
struct LogRecorder {
    batches: Mutex<Vec<Vec<LogSinkEntry>>>,
    fail: Mutex<Option<String>>,
}

#[async_trait]
impl LogSinkHandler for LogRecorder {
    async fn push(&self, _ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error> {
        if let Some(message) = self.fail.lock().unwrap().clone() {
            return Err(Error::from(message));
        }
        let accepted = batch.iter().filter(|e| e.status != 500).count();
        self.batches.lock().unwrap().push(batch);
        Ok(accepted)
    }
}

impl LogRecorder {
    fn sizes(&self) -> Vec<usize> {
        self.batches.lock().unwrap().iter().map(Vec::len).collect()
    }
}

fn log_request(status: i32) -> Vec<u8> {
    pb::LogSinkPushRequest {
        log_path: "/var/log/nginx/access.log".into(),
        entry: Some(pb::LogEntry {
            timestamp: "2026-09-23T08:15:02Z".into(),
            remote_addr: "203.0.113.7".into(),
            request_method: "GET".into(),
            request_uri: "/index.html?lang=en".into(),
            protocol: "HTTP/1.1".into(),
            status,
            body_bytes_sent: (6u64 << 30) as f64,
            user_agent: "curl/8.9.1".into(),
            request_time: 0.004,
            upstream_response_time: 0.003,
            raw: "203.0.113.7 - - [...]".into(),
            format: log_format::COMBINED.into(),
            ..Default::default()
        }),
    }
    .encode_to_vec()
}

fn response(out: &[u8]) -> pb::LogSinkPushResponse {
    pb::LogSinkPushResponse::decode(out).unwrap()
}

#[tokio::test]
async fn log_sink_streams_over_grpc() {
    let sink = Arc::new(LogRecorder::default());
    let dir = short_temp_dir();
    let mut h = grpc_harness(
        Plugin::new().log_sink(sink.clone()),
        &dir,
        Network::platform_default(),
    )
    .await;
    assert_eq!(h.init.capabilities, [capability::LOG_SINK]);

    let grpc = Grpc::connect(&h.init).await;
    let res = response(
        &grpc
            .stream(
                PUSH,
                &[log_request(200), log_request(500), log_request(404)],
            )
            .await
            .unwrap(),
    );
    assert_eq!((res.accepted, res.rejected), (2, 1));

    assert_eq!(sink.sizes(), [3]);
    let entry = sink.batches.lock().unwrap()[0][0].clone();
    assert_eq!(entry.log_path, "/var/log/nginx/access.log");
    assert_eq!(entry.request_uri, "/index.html?lang=en");
    assert_eq!(entry.status, 200);
    assert_eq!(entry.body_bytes_sent.get(), 6 << 30);
    assert!((entry.upstream_response_time - 0.003).abs() < 1e-9);
    assert!(entry.parsed());
    // 2026-09-23T08:15:02Z
    assert_eq!(
        entry.time(),
        Some(UNIX_EPOCH + Duration::from_secs(1_790_151_302))
    );

    // An empty stream is valid and answers with zero counts.
    let res = response(&grpc.stream(PUSH, &[]).await.unwrap());
    assert_eq!((res.accepted, res.rejected), (0, 0));

    // Undecodable bytes are invalid params.
    let err = grpc
        .stream(PUSH, &[vec![0xff, 0xff, 0xff]])
        .await
        .unwrap_err();
    assert_eq!(err.code, clients::INVALID_ARGUMENT);
    assert_eq!(err.plugin_code(), Some(code::INVALID_PARAMS));

    // A failing handler loses the batch and says so.
    *sink.fail.lock().unwrap() = Some("destination down".into());
    let err = grpc.stream(PUSH, &[log_request(200)]).await.unwrap_err();
    assert_eq!(
        (err.code, err.plugin_code()),
        (clients::INTERNAL, Some(code::INTERNAL_ERROR))
    );

    h.stop().await;
}

#[tokio::test]
async fn log_sink_splits_long_streams() {
    let sink = Arc::new(LogRecorder::default());
    let dir = short_temp_dir();
    let h = grpc_harness(
        Plugin::new().log_sink(sink.clone()),
        &dir,
        Network::platform_default(),
    )
    .await;
    let grpc = Grpc::connect(&h.init).await;

    let requests = vec![log_request(200); MAX_LOG_SINK_BATCH + 4];
    let res = response(&grpc.stream(PUSH, &requests).await.unwrap());
    assert_eq!(res.accepted as usize, requests.len());
    assert_eq!(sink.sizes(), [MAX_LOG_SINK_BATCH, 4]);
}

#[tokio::test]
async fn log_sink_is_never_served_on_stdio() {
    let sink = Arc::new(LogRecorder::default());
    let called = Arc::new(Mutex::new(false));
    let flag = called.clone();
    let dir = short_temp_dir();
    // A stdio handler for a streaming rpc is ignored.
    let plugin = Plugin::new()
        .log_sink(sink)
        .method(method::LOG_PUSH, move |_ctx, _params| {
            let flag = flag.clone();
            async move {
                *flag.lock().unwrap() = true;
                Ok(json!({}))
            }
        });
    let h = grpc_harness(plugin, &dir, Network::platform_default()).await;

    let err = h
        .call(
            method::LOG_PUSH,
            LogSinkPushParams {
                log_path: "/var/log/nginx/access.log".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, code::METHOD_NOT_FOUND);
    assert!(
        !*called.lock().unwrap(),
        "the stdio handler of log.push ran"
    );
}

#[tokio::test]
async fn log_sink_keeps_grpc_on() {
    // Option.
    let dir = short_temp_dir();
    let mut h = Builder::new(Plugin::new().log_sink(Arc::new(LogRecorder::default())))
        .data_dir(dir.path())
        .options(|o: Options| {
            o.without_grpc()
                .with_grpc_network(Network::platform_default())
        })
        .start();
    let init = h.initialize().await;
    assert_eq!(init.transports, [transport::STDIO, transport::GRPC]);
    assert!(!init.rpc_socket.is_empty() || !init.rpc_pipe.is_empty());

    // Environment.
    let dir = short_temp_dir();
    let mut h = Builder::new(Plugin::new().log_sink(Arc::new(LogRecorder::default())))
        .data_dir(dir.path())
        .var(env::DISABLE_GRPC, "1")
        .options(|o| o.with_grpc_network(Network::platform_default()))
        .start();
    let init = h.initialize().await;
    assert_eq!(init.transports, [transport::STDIO, transport::GRPC]);
}

#[tokio::test]
async fn log_push_without_a_handler_is_unknown() {
    let dir = short_temp_dir();
    let h = grpc_harness(
        Plugin::new().dns01(GrpcHandler),
        &dir,
        Network::platform_default(),
    )
    .await;
    let grpc = Grpc::connect(&h.init).await;

    let err = grpc.stream(PUSH, &[log_request(200)]).await.unwrap_err();
    assert_eq!(
        (err.code, err.plugin_code()),
        (clients::UNIMPLEMENTED, Some(code::METHOD_NOT_FOUND))
    );
    let _: Value = json!(null);
}

#[tokio::test]
async fn an_open_stream_counts_for_shutdown() {
    struct Gate(tokio::sync::mpsc::UnboundedSender<()>);

    #[async_trait]
    impl LogSinkHandler for Gate {
        async fn push(&self, _ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error> {
            let _ = self.0.send(());
            tokio::time::sleep(Duration::from_millis(400)).await;
            Ok(batch.len())
        }
    }

    let (tx, mut started) = tokio::sync::mpsc::unbounded_channel();
    let dir = short_temp_dir();
    let h = grpc_harness(
        Plugin::new().log_sink(Gate(tx)),
        &dir,
        Network::platform_default(),
    )
    .await;
    let grpc = Grpc::connect(&h.init).await;

    let stream = tokio::spawn(async move { grpc.stream(PUSH, &[log_request(200)]).await });
    tokio::time::timeout(Duration::from_secs(5), started.recv())
        .await
        .unwrap();

    let begin = std::time::Instant::now();
    h.call(method::SHUTDOWN, ()).await.unwrap();
    assert!(
        begin.elapsed() >= Duration::from_millis(150),
        "shutdown did not wait for the stream"
    );
    assert_eq!(response(&stream.await.unwrap().unwrap()).accepted, 1);
}
