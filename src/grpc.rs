//! The optional gRPC transport (spec WIRE-11).
//!
//! Every gRPC call is resolved through the [`registry`](crate::pb::registry)
//! of the contract to its JSON-RPC method name and runs the exact handler the
//! stdio dispatcher runs, so both transports answer identically. A client
//! streaming rpc (WIRE-12) has no stdio form: it is read until the end of the
//! stream and handed to its stream handler.

use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::task::Poll;
use std::time::Duration;

use bytes::{Buf, BufMut, Bytes};
use http::{HeaderMap, Request, Response};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo, TokioTimer};
use prost::Message;
use serde_json::Value;
use tonic::body::Body;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::server::Grpc;
use tonic::{Code, Status, Streaming};

use crate::context::Context;
use crate::dispatch::RtHandler;
use crate::host::with_task_host;
use crate::jsonrpc::BoxFuture;
use crate::panic::{catch_unwind, CatchUnwind};
use crate::pb::registry::{Rpc, RPCS};
use crate::pb::v1 as pb;
use crate::pb::wkt;
use crate::protocol::{code, method, Error, InitializeResult};
use crate::runtime::Runtime;
use crate::transport::{wait_flag, Listener, Network, Server, Stream};
use crate::util::{
    constant_time_eq, create_private_dir, max_socket_path_len, random_hex, remove_stale_socket,
    restrict_permissions,
};

/// File name of the gRPC socket inside the data directory.
pub const RPC_SOCKET_NAME: &str = "rpc.sock";

/// Bounds one gRPC message in either direction. gRPC exists for payloads the
/// 4 MiB stdio frame cannot carry, so it is larger.
pub const MAX_GRPC_MESSAGE_BYTES: usize = 64 << 20;

/// Bounds the graceful stop of the gRPC server on exit.
const STOP_TIMEOUT: Duration = Duration::from_secs(2);

/// Type URL of the `PluginError` status detail.
const PLUGIN_ERROR_TYPE_URL: &str = "type.googleapis.com/nginxui.plugin.v1.PluginError";

/// The handshake and the stop sequence always travel on stdio, a gRPC call
/// for one of them is refused.
fn is_stdio_only(name: &str) -> bool {
    matches!(
        name,
        method::INITIALIZE | method::INITIALIZED | method::SHUTDOWN | method::EXIT
    )
}

/// Maps every gRPC full method of the contract to its rpc.
fn rpc_index() -> &'static HashMap<&'static str, &'static Rpc> {
    static INDEX: OnceLock<HashMap<&'static str, &'static Rpc>> = OnceLock::new();
    INDEX.get_or_init(|| RPCS.iter().map(|rpc| (rpc.full_method, rpc)).collect())
}

/// Reports whether `name` is the `rpc_name` of a streaming rpc, which must
/// never be registered as a stdio handler.
pub(crate) fn is_streaming_rpc(name: &str) -> bool {
    RPCS.iter().any(|rpc| rpc.rpc_name == name && rpc.streaming)
}

/// Hands the undecoded message bytes to the handler. The bytes are protobuf,
/// so the wire keeps the standard `application/grpc` content type.
#[derive(Debug, Clone, Copy, Default)]
struct RawCodec;

#[derive(Debug, Clone, Copy, Default)]
struct RawEncoder;

#[derive(Debug, Clone, Copy, Default)]
struct RawDecoder;

impl Codec for RawCodec {
    type Encode = Bytes;
    type Decode = Bytes;
    type Encoder = RawEncoder;
    type Decoder = RawDecoder;

    fn encoder(&mut self) -> RawEncoder {
        RawEncoder
    }

    fn decoder(&mut self) -> RawDecoder {
        RawDecoder
    }
}

impl Encoder for RawEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(&mut self, item: Bytes, dst: &mut EncodeBuf<'_>) -> Result<(), Status> {
        dst.put(item);
        Ok(())
    }
}

impl Decoder for RawDecoder {
    type Item = Bytes;
    type Error = Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Bytes>, Status> {
        Ok(Some(src.copy_to_bytes(src.remaining())))
    }
}

/// The running gRPC listener of one plugin process.
pub(crate) struct GrpcTransport {
    server: Server,
    /// The Unix socket path, `None` on TCP.
    socket: Option<PathBuf>,
    /// The fallback directory holding `socket`, removed on stop.
    tmp_dir: Option<PathBuf>,
    port: u16,
    /// Presented on TCP as `authorization: Bearer <token>`.
    token: String,
}

/// Opens the listener and starts serving in the background.
pub(crate) fn start(
    rt: Arc<Runtime>,
    data_dir: &Path,
    network: Option<Network>,
    fallback_dirs: Option<&[PathBuf]>,
) -> Result<GrpcTransport, String> {
    let network = network.unwrap_or_else(Network::platform_default);

    let mut socket = None;
    let mut tmp_dir = None;
    let mut port = 0;
    let mut token = String::new();
    let listener = match network {
        Network::Unix => {
            let (listener, path, dir) = listen_unix(data_dir, fallback_dirs)?;
            socket = Some(path);
            tmp_dir = dir;
            listener
        }
        Network::Tcp => {
            token = random_hex(32).map_err(|e| e.to_string())?;
            let (listener, p) = listen_tcp()?;
            port = p;
            listener
        }
    };

    let token_arc: Option<Arc<str>> = (!token.is_empty()).then(|| Arc::from(token.as_str()));
    let server = Server::spawn(listener, move |stream, stop| {
        Box::pin(serve_connection(
            stream,
            stop,
            rt.clone(),
            token_arc.clone(),
        ))
    });
    Ok(GrpcTransport {
        server,
        socket,
        tmp_dir,
        port,
        token,
    })
}

#[cfg(unix)]
fn listen_unix(
    data_dir: &Path,
    fallback_dirs: Option<&[PathBuf]>,
) -> Result<(Listener, PathBuf, Option<PathBuf>), String> {
    use std::os::unix::fs::DirBuilderExt;

    let limit = max_socket_path_len();

    if !data_dir.as_os_str().is_empty() {
        if let Ok(dir) = std::path::absolute(data_dir) {
            let path = dir.join(RPC_SOCKET_NAME);
            if path.as_os_str().len() <= limit
                && std::fs::DirBuilder::new()
                    .recursive(true)
                    .mode(0o700)
                    .create(&dir)
                    .is_ok()
            {
                if let Ok(listener) = listen_on(&path) {
                    return Ok((listener, path, None));
                }
            }
        }
    }

    // The data directory is unusable or its path too long: use a private
    // directory under the temp dir and report the path in rpc_socket.
    let mut last_error = String::from("no usable socket directory");
    let default_dirs = [std::env::temp_dir(), PathBuf::from("/tmp")];
    for base in fallback_dirs.unwrap_or(&default_dirs) {
        let dir = match create_private_dir(base, "nuip-") {
            Ok(dir) => dir,
            Err(e) => {
                last_error = e.to_string();
                continue;
            }
        };
        let path = dir.join(RPC_SOCKET_NAME);
        if path.as_os_str().len() > limit {
            let _ = std::fs::remove_dir_all(&dir);
            last_error = format!(
                "socket path {} is longer than {limit} bytes",
                path.display()
            );
            continue;
        }
        match listen_on(&path) {
            Ok(listener) => return Ok((listener, path, Some(dir))),
            Err(e) => {
                let _ = std::fs::remove_dir_all(&dir);
                last_error = e.to_string();
            }
        }
    }
    Err(last_error)
}

#[cfg(not(unix))]
fn listen_unix(
    _data_dir: &Path,
    _fallback_dirs: Option<&[PathBuf]>,
) -> Result<(Listener, PathBuf, Option<PathBuf>), String> {
    let _ = (
        max_socket_path_len(),
        create_private_dir,
        remove_stale_socket,
        restrict_permissions,
    );
    Err("unix sockets are not supported on this platform".to_owned())
}

#[cfg(unix)]
fn listen_on(path: &Path) -> std::io::Result<Listener> {
    // A crashed predecessor may have left its socket behind.
    let _ = remove_stale_socket(path);
    let listener = tokio::net::UnixListener::bind(path)?;
    let _ = restrict_permissions(path);
    Ok(Listener::Unix(listener))
}

fn listen_tcp() -> Result<(Listener, u16), String> {
    let std_listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    std_listener
        .set_nonblocking(true)
        .map_err(|e| e.to_string())?;
    let port = std_listener.local_addr().map_err(|e| e.to_string())?.port();
    let listener = tokio::net::TcpListener::from_std(std_listener).map_err(|e| e.to_string())?;
    Ok((Listener::Tcp(listener), port))
}

impl GrpcTransport {
    /// Adds the transport to the `plugin.initialize` reply.
    pub(crate) fn advertise(&self, res: &mut InitializeResult) {
        res.transports
            .push(crate::protocol::transport::GRPC.to_owned());
        match &self.socket {
            Some(path) => res.rpc_socket = path.to_string_lossy().into_owned(),
            None => {
                res.rpc_port = i32::from(self.port);
                res.rpc_token.clone_from(&self.token);
            }
        }
    }

    /// Ends the server, waiting a moment for calls still running, and removes
    /// the socket.
    pub(crate) async fn stop(&self) {
        self.server.begin_stop();
        self.server.wait(STOP_TIMEOUT).await;
        self.cleanup();
    }

    fn cleanup(&self) {
        if let Some(path) = &self.socket {
            let _ = std::fs::remove_file(path);
        }
        if let Some(dir) = &self.tmp_dir {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

impl Drop for GrpcTransport {
    fn drop(&mut self) {
        self.server.abort();
        self.cleanup();
    }
}

async fn serve_connection(
    stream: Stream,
    mut stop: tokio::sync::watch::Receiver<bool>,
    rt: Arc<Runtime>,
    token: Option<Arc<str>>,
) {
    let service = service_fn(move |req: Request<Incoming>| {
        let rt = rt.clone();
        let token = token.clone();
        async move { Ok::<_, Infallible>(handle(rt, token, req).await) }
    });

    let conn = hyper::server::conn::http2::Builder::new(TokioExecutor::new())
        .timer(TokioTimer::new())
        .adaptive_window(true)
        .max_frame_size(Some(1 << 20))
        .serve_connection(TokioIo::new(stream), service);
    tokio::pin!(conn);

    tokio::select! {
        result = conn.as_mut() => {
            if let Err(e) = result {
                crate::logger::debug(format!("gRPC connection ended: {e}"));
            }
        }
        () = wait_flag(&mut stop) => {
            conn.as_mut().graceful_shutdown();
            let _ = conn.await;
        }
    }
}

/// Serves one gRPC request. It is the only entry point, no service is
/// registered, so a method outside the contract is answered like on stdio.
async fn handle(
    rt: Arc<Runtime>,
    token: Option<Arc<str>>,
    req: Request<Incoming>,
) -> Response<Body> {
    if let Some(token) = &token {
        if !valid_token(req.headers(), token) {
            let err = Error::new(
                code::PERMISSION_DENIED,
                "missing or invalid rpc token",
                None,
            );
            return grpc_status(&err, Some(Code::Unauthenticated)).into_http();
        }
    }

    let path = req.uri().path().to_owned();
    let timeout = parse_timeout(req.headers());
    let rpc = rpc_index().get(path.as_str()).copied();
    let ctx = match timeout {
        Some(d) => rt.new_context().with_timeout(d),
        None => rt.new_context(),
    };

    let call = async move {
        let grpc = Grpc::new(RawCodec)
            .max_decoding_message_size(MAX_GRPC_MESSAGE_BYTES)
            .max_encoding_message_size(MAX_GRPC_MESSAGE_BYTES);
        let mut grpc = grpc;
        match rpc {
            Some(rpc) if rpc.streaming => {
                grpc.client_streaming(StreamSvc { rt, rpc, ctx }, req).await
            }
            _ => grpc.unary(UnarySvc { rt, rpc, path, ctx }, req).await,
        }
    };

    match timeout {
        // The deadline bounds the whole call, a stream included.
        Some(d) => match tokio::time::timeout(d, call).await {
            Ok(response) => response,
            Err(_) => Status::deadline_exceeded("deadline exceeded").into_http(),
        },
        None => call.await,
    }
}

/// Answers one unary rpc.
struct UnarySvc {
    rt: Arc<Runtime>,
    rpc: Option<&'static Rpc>,
    path: String,
    ctx: Context,
}

impl tower_service::Service<tonic::Request<Bytes>> for UnarySvc {
    type Response = tonic::Response<Bytes>;
    type Error = Status;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Status>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Status>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: tonic::Request<Bytes>) -> Self::Future {
        let rt = self.rt.clone();
        let rpc = self.rpc;
        let path = self.path.clone();
        let ctx = self.ctx.clone();
        Box::pin(async move {
            let out = serve_unary(&rt, rpc, &path, ctx, request.into_inner()).await?;
            Ok(tonic::Response::new(Bytes::from(out)))
        })
    }
}

/// Resolves one gRPC call to its JSON-RPC handler and runs it. The returned
/// error is always a gRPC status.
async fn serve_unary(
    rt: &Runtime,
    rpc: Option<&'static Rpc>,
    full_method: &str,
    ctx: Context,
    input: Bytes,
) -> Result<Vec<u8>, Status> {
    let Some(rpc) = rpc else {
        return Err(grpc_status(
            &Error::new(
                code::METHOD_NOT_FOUND,
                format!("unknown method: {full_method}"),
                None,
            ),
            None,
        ));
    };
    if is_stdio_only(rpc.rpc_name) {
        return Err(grpc_status(
            &Error::new(
                code::METHOD_NOT_FOUND,
                format!("{} is only served on stdio", rpc.rpc_name),
                None,
            ),
            None,
        ));
    }

    let Some(handler) = rt.handler(rpc.rpc_name) else {
        return Err(grpc_status(&Error::method_not_found(rpc.rpc_name), None));
    };

    let params =
        (rpc.request_to_json)(&input).map_err(|e| grpc_status(&Error::invalid_params(e), None))?;

    if rpc.notification {
        // A notification is answered at once, its handler outlives the call.
        let detached = ctx.detached();
        let name = rpc.rpc_name;
        tokio::spawn(async move {
            let _ = invoke(name, &handler, detached, params).await;
        });
        return (rpc.json_to_response)(&Value::Null)
            .map_err(|e| grpc_status(&Error::internal(format!("marshal result: {e}")), None));
    }

    let result = invoke(rpc.rpc_name, &handler, ctx, params)
        .await
        .map_err(|e| grpc_status(&e, None))?;
    (rpc.json_to_response)(&result)
        .map_err(|e| grpc_status(&Error::internal(format!("marshal result: {e}")), None))
}

/// Runs a handler and turns a panic into an internal error, like the stdio
/// dispatcher does.
async fn invoke(
    name: &str,
    handler: &RtHandler,
    ctx: Context,
    params: Value,
) -> Result<Value, Error> {
    match catch_unwind(handler(ctx, params)).await {
        Ok(outcome) => outcome,
        Err(message) => Err(Error::internal(format!("panic in {name}: {message}"))),
    }
}

/// Answers one client streaming rpc.
struct StreamSvc {
    rt: Arc<Runtime>,
    rpc: &'static Rpc,
    ctx: Context,
}

impl tower_service::Service<tonic::Request<Streaming<Bytes>>> for StreamSvc {
    type Response = tonic::Response<Bytes>;
    type Error = Status;
    type Future = BoxFuture<Result<Self::Response, Status>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Status>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: tonic::Request<Streaming<Bytes>>) -> Self::Future {
        let rt = self.rt.clone();
        let rpc = self.rpc;
        let ctx = self.ctx.clone();
        Box::pin(async move {
            let out = serve_stream(&rt, rpc, ctx, request.into_inner()).await?;
            Ok(tonic::Response::new(Bytes::from(out)))
        })
    }
}

/// Reads a client stream until its end, hands every message to the stream
/// handler of the rpc and answers once.
async fn serve_stream(
    rt: &Runtime,
    rpc: &'static Rpc,
    ctx: Context,
    mut stream: Streaming<Bytes>,
) -> Result<Vec<u8>, Status> {
    let Some(open) = rt.stream_factory(rpc.rpc_name) else {
        return Err(grpc_status(&Error::method_not_found(rpc.rpc_name), None));
    };
    let mut handler = open();

    let outcome = CatchUnwind(Box::pin(with_task_host(rt.host(), async {
        while let Some(message) = stream.message().await? {
            handler
                .add(&ctx, &message)
                .await
                .map_err(|e| grpc_status(&e, None))?;
        }
        handler
            .finish(&ctx)
            .await
            .map_err(|e| grpc_status(&e, None))
    })))
    .await;

    match outcome {
        Ok(result) => result,
        Err(panic) => Err(grpc_status(
            &Error::internal(format!(
                "panic in {}: {}",
                rpc.rpc_name,
                crate::panic::panic_message(&*panic)
            )),
            None,
        )),
    }
}

/// Checks the metadata `authorization: Bearer <token>` in constant time.
fn valid_token(headers: &HeaderMap, token: &str) -> bool {
    let values: Vec<_> = headers
        .get_all(http::header::AUTHORIZATION)
        .iter()
        .collect();
    if values.len() != 1 {
        return false;
    }
    let want = format!("Bearer {token}");
    constant_time_eq(want.as_bytes(), values[0].as_bytes())
}

/// Parses the `grpc-timeout` header, `<digits><unit>` with the units `H`,
/// `M`, `S`, `m`, `u` and `n`.
fn parse_timeout(headers: &HeaderMap) -> Option<Duration> {
    let text = headers.get("grpc-timeout")?.to_str().ok()?;
    if text.len() < 2 || !text.is_char_boundary(text.len() - 1) {
        return None;
    }
    let (digits, unit) = text.split_at(text.len() - 1);
    let n: u64 = digits.parse().ok()?;
    Some(match unit {
        "H" => Duration::from_secs(n.checked_mul(3600)?),
        "M" => Duration::from_secs(n.checked_mul(60)?),
        "S" => Duration::from_secs(n),
        "m" => Duration::from_millis(n),
        "u" => Duration::from_micros(n),
        "n" => Duration::from_nanos(n),
        _ => return None,
    })
}

/// Maps a JSON-RPC error code onto a gRPC status code (spec WIRE-11).
pub(crate) fn grpc_code_for(rpc_code: i32) -> Code {
    match rpc_code {
        code::PARSE_ERROR | code::INVALID_REQUEST | code::INVALID_PARAMS | code::INVALID_CONFIG => {
            Code::InvalidArgument
        }
        code::METHOD_NOT_FOUND | code::UNSUPPORTED => Code::Unimplemented,
        code::INTERNAL_ERROR => Code::Internal,
        code::PERMISSION_DENIED => Code::PermissionDenied,
        _ => Code::Unknown,
    }
}

/// The `google.rpc.Status` message `grpc-status-details-bin` carries.
#[derive(Clone, PartialEq, Message)]
pub(crate) struct RpcStatus {
    #[prost(int32, tag = "1")]
    pub(crate) code: i32,
    #[prost(string, tag = "2")]
    pub(crate) message: String,
    #[prost(message, repeated, tag = "3")]
    pub(crate) details: Vec<Any>,
}

/// `google.protobuf.Any`.
#[derive(Clone, PartialEq, Message)]
pub(crate) struct Any {
    #[prost(string, tag = "1")]
    pub(crate) type_url: String,
    #[prost(bytes = "vec", tag = "2")]
    pub(crate) value: Vec<u8>,
}

/// Turns a handler error into a gRPC status carrying the JSON-RPC error as a
/// `PluginError` detail. `override_code` replaces the mapped status code.
pub(crate) fn grpc_status(err: &Error, override_code: Option<Code>) -> Status {
    let status_code = override_code.unwrap_or_else(|| grpc_code_for(err.code));
    let detail = pb::PluginError {
        code: err.code,
        message: err.message.clone(),
        data: error_data(err.data.as_ref()),
    };
    let status = RpcStatus {
        code: status_code as i32,
        message: err.message.clone(),
        details: vec![Any {
            type_url: PLUGIN_ERROR_TYPE_URL.to_owned(),
            value: detail.encode_to_vec(),
        }],
    };
    Status::with_details(
        status_code,
        err.message.clone(),
        Bytes::from(status.encode_to_vec()),
    )
}

/// Converts error data to a `Struct`. A value that is not a JSON object is
/// wrapped as `{"value": data}` (spec WIRE-5).
fn error_data(data: Option<&Value>) -> Option<wkt::Struct> {
    match data? {
        Value::Null => None,
        Value::Object(map) => Some(wkt::Struct::from(map.clone())),
        other => {
            let mut map = serde_json::Map::new();
            map.insert("value".to_owned(), other.clone());
            Some(wkt::Struct::from(map))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeouts_parse() {
        let parse = |v: &str| {
            let mut h = HeaderMap::new();
            h.insert("grpc-timeout", v.parse().unwrap());
            parse_timeout(&h)
        };
        assert_eq!(parse("5S"), Some(Duration::from_secs(5)));
        assert_eq!(parse("100m"), Some(Duration::from_millis(100)));
        assert_eq!(parse("2H"), Some(Duration::from_secs(7200)));
        assert_eq!(parse("15000000u"), Some(Duration::from_secs(15)));
        assert_eq!(parse("S"), None);
        assert_eq!(parse("5x"), None);
        assert_eq!(parse_timeout(&HeaderMap::new()), None);
    }

    #[test]
    fn rpc_index_covers_the_contract() {
        let index = rpc_index();
        for (full, want) in [
            ("/nginxui.plugin.v1.DNS01/Present", method::DNS01_PRESENT),
            ("/nginxui.plugin.v1.HTTP/Handle", method::HTTP_HANDLE),
            ("/nginxui.plugin.v1.Notify/Send", method::NOTIFY_SEND),
            ("/nginxui.plugin.v1.Probe/Check", method::PROBE_CHECK),
            ("/nginxui.plugin.v1.MCP/Call", method::MCP_CALL),
            ("/nginxui.plugin.v1.Storage/List", method::STORAGE_LIST),
            ("/nginxui.plugin.v1.Deploy/Push", method::DEPLOY_PUSH),
            ("/nginxui.plugin.v1.Plugin/Ping", method::PING),
            ("/nginxui.plugin.v1.Events/On", method::EVENTS_ON),
            ("/nginxui.plugin.v1.Host/KVGet", method::HOST_KV_GET),
            (
                "/nginxui.plugin.v1.Blocklist/Fetch",
                method::BLOCKLIST_FETCH,
            ),
            (
                "/nginxui.plugin.v1.Discovery/Resolve",
                method::DISCOVERY_RESOLVE,
            ),
        ] {
            let rpc = index
                .get(full)
                .unwrap_or_else(|| panic!("{full} is missing"));
            assert_eq!(rpc.rpc_name, want, "{full}");
        }
        assert!(index["/nginxui.plugin.v1.Events/On"].notification);
        let push = index["/nginxui.plugin.v1.LogSink/Push"];
        assert!(push.streaming && push.rpc_name == method::LOG_PUSH);
        assert!(is_streaming_rpc(method::LOG_PUSH));
        assert!(!is_streaming_rpc(method::DNS01_PRESENT));
    }

    #[test]
    fn a_struct_field_survives_the_json_round_trip() {
        let request = pb::Dns01OptionsRequest {
            provider: "p".into(),
            options: Some(wkt::Struct::from(
                serde_json::json!({"credential_id": "7", "n": 3})
                    .as_object()
                    .unwrap()
                    .clone(),
            )),
            ..Default::default()
        };
        let rpc = rpc_index()["/nginxui.plugin.v1.DNS01/Options"];
        let json = (rpc.request_to_json)(&request.encode_to_vec()).unwrap();
        let params: crate::protocol::Dns01OptionsParams = serde_json::from_value(json).unwrap();
        assert_eq!(params.provider, "p");
        assert_eq!(params.options["credential_id"], "7");
        assert_eq!(params.options["n"].as_i64(), Some(3));
    }

    #[test]
    fn errors_map_to_statuses() {
        let cases: [(Error, Code, i32, Option<Value>); 7] = [
            (
                Error::invalid_config("TOKEN", "bad"),
                Code::InvalidArgument,
                code::INVALID_CONFIG,
                Some(serde_json::json!({"field": "TOKEN"})),
            ),
            (
                Error::invalid_params("bad"),
                Code::InvalidArgument,
                code::INVALID_PARAMS,
                None,
            ),
            (
                Error::unsupported("dns01.check"),
                Code::Unimplemented,
                code::UNSUPPORTED,
                None,
            ),
            (
                Error::new(code::METHOD_NOT_FOUND, "x", None),
                Code::Unimplemented,
                code::METHOD_NOT_FOUND,
                None,
            ),
            (
                Error::new(code::PERMISSION_DENIED, "x", None),
                Code::PermissionDenied,
                code::PERMISSION_DENIED,
                None,
            ),
            (
                Error::internal("vendor down"),
                Code::Internal,
                code::INTERNAL_ERROR,
                None,
            ),
            (
                Error::new(-31000, "x", Some(Value::from("scalar"))),
                Code::Unknown,
                -31000,
                Some(serde_json::json!({"value": "scalar"})),
            ),
        ];
        for (err, want_code, want_rpc, want_data) in cases {
            let status = grpc_status(&err, None);
            assert_eq!(status.code(), want_code, "{err:?}");
            assert_eq!(status.message(), err.message);
            let details = RpcStatus::decode(status.details()).unwrap();
            assert_eq!(details.code, want_code as i32);
            let any = &details.details[0];
            assert_eq!(any.type_url, PLUGIN_ERROR_TYPE_URL);
            let detail = pb::PluginError::decode(any.value.as_slice()).unwrap();
            assert_eq!(detail.code, want_rpc);
            assert_eq!(detail.message, err.message);
            let data = detail.data.map(|s| Value::Object(s.into()));
            assert_eq!(data, want_data, "{err:?}");
        }
        let overridden = grpc_status(&Error::internal("x"), Some(Code::Unauthenticated));
        assert_eq!(overridden.code(), Code::Unauthenticated);
    }
}
