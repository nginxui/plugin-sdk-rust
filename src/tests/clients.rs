//! Minimal clients for the transports under test, written against the
//! specification and independent of the server code.

use std::time::Duration;

use bytes::{Buf, Bytes};
use http::Request;
use http_body_util::{BodyExt, Full};
use hyper_util::rt::{TokioExecutor, TokioIo};
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::grpc::RpcStatus;
use crate::pb::v1 as pb;
use crate::protocol::InitializeResult;

// gRPC.

/// The status of a failed gRPC call.
#[derive(Debug)]
pub struct GrpcError {
    /// The gRPC status code.
    pub code: i32,
    pub message: String,
    /// The `PluginError` detail, if the status carries one.
    pub detail: Option<pb::PluginError>,
}

pub const OK: i32 = 0;
pub const UNKNOWN: i32 = 2;
pub const INVALID_ARGUMENT: i32 = 3;
pub const UNIMPLEMENTED: i32 = 12;
pub const INTERNAL: i32 = 13;
pub const UNAUTHENTICATED: i32 = 16;

pub struct Grpc {
    sender: hyper::client::conn::http2::SendRequest<Full<Bytes>>,
    token: Option<String>,
}

impl Grpc {
    /// Connects to the endpoint the plugin advertised.
    pub async fn connect(init: &InitializeResult) -> Grpc {
        let token = (!init.rpc_token.is_empty()).then(|| init.rpc_token.clone());
        if init.rpc_port != 0 {
            let stream = tokio::net::TcpStream::connect((
                "127.0.0.1",
                u16::try_from(init.rpc_port).unwrap(),
            ))
            .await
            .expect("connect to the rpc port");
            return Grpc::over(TokioIo::new(stream), token).await;
        }
        #[cfg(windows)]
        if !init.rpc_pipe.is_empty() {
            let stream = connect_pipe(&init.rpc_pipe)
                .await
                .expect("connect to the rpc pipe");
            return Grpc::over(TokioIo::new(stream), token).await;
        }
        #[cfg(unix)]
        {
            let stream = tokio::net::UnixStream::connect(&init.rpc_socket)
                .await
                .expect("connect to the rpc socket");
            Grpc::over(TokioIo::new(stream), token).await
        }
        #[cfg(not(unix))]
        panic!("no rpc pipe or port")
    }

    async fn over<T>(io: T, token: Option<String>) -> Grpc
    where
        T: hyper::rt::Read + hyper::rt::Write + Unpin + Send + 'static,
    {
        let (sender, conn) = hyper::client::conn::http2::handshake(TokioExecutor::new(), io)
            .await
            .expect("http2 handshake");
        tokio::spawn(async move {
            let _ = conn.await;
        });
        Grpc { sender, token }
    }

    /// One unary call.
    pub async fn call(&self, path: &str, request: &[u8]) -> Result<Vec<u8>, GrpcError> {
        self.call_full(path, &[request.to_vec()], None, true, None)
            .await
    }

    /// A client stream of `messages`, answered once.
    pub async fn stream(&self, path: &str, messages: &[Vec<u8>]) -> Result<Vec<u8>, GrpcError> {
        self.call_full(path, messages, None, true, None).await
    }

    /// One call with a `grpc-timeout` header.
    pub async fn call_with_timeout(
        &self,
        path: &str,
        request: &[u8],
        timeout: &str,
    ) -> Result<Vec<u8>, GrpcError> {
        self.call_full(path, &[request.to_vec()], None, true, Some(timeout))
            .await
    }

    /// One call with an explicit authorization header.
    pub async fn call_with_authorization(
        &self,
        path: &str,
        request: &[u8],
        authorization: Option<&str>,
    ) -> Result<Vec<u8>, GrpcError> {
        self.call_full(path, &[request.to_vec()], authorization, false, None)
            .await
    }

    async fn call_full(
        &self,
        path: &str,
        messages: &[Vec<u8>],
        authorization: Option<&str>,
        use_token: bool,
        timeout: Option<&str>,
    ) -> Result<Vec<u8>, GrpcError> {
        let mut body = Vec::new();
        for message in messages {
            body.push(0u8);
            body.extend_from_slice(&u32::try_from(message.len()).unwrap().to_be_bytes());
            body.extend_from_slice(message);
        }
        let mut builder = Request::builder()
            .method("POST")
            .uri(format!("http://plugin{path}"))
            .header("content-type", "application/grpc")
            .header("te", "trailers");
        if let Some(value) = authorization {
            builder = builder.header("authorization", value);
        } else if use_token {
            if let Some(token) = &self.token {
                builder = builder.header("authorization", format!("Bearer {token}"));
            }
        }
        if let Some(timeout) = timeout {
            builder = builder.header("grpc-timeout", timeout);
        }
        let request = builder.body(Full::new(Bytes::from(body))).unwrap();

        let mut sender = self.sender.clone();
        let response = tokio::time::timeout(Duration::from_secs(10), sender.send_request(request))
            .await
            .expect("the call timed out")
            .expect("the call failed at the transport");
        let (parts, body) = response.into_parts();
        let collected = tokio::time::timeout(Duration::from_secs(10), body.collect())
            .await
            .expect("reading the reply timed out")
            .expect("reading the reply failed");
        let trailers = collected.trailers().cloned();
        let mut data = collected.to_bytes();

        // A trailers only reply carries the status in the headers.
        let headers = trailers.as_ref().unwrap_or(&parts.headers);
        let code: i32 = headers
            .get("grpc-status")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(UNKNOWN);
        if code != OK {
            let message = percent_decode(
                headers
                    .get("grpc-message")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or_default(),
            );
            let detail = headers
                .get("grpc-status-details-bin")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD_NO_PAD
                        .decode(v.trim_end_matches('='))
                        .ok()
                })
                .and_then(|raw| RpcStatus::decode(raw.as_slice()).ok())
                .and_then(|status| status.details.into_iter().next())
                .and_then(|any| pb::PluginError::decode(any.value.as_slice()).ok());
            return Err(GrpcError {
                code,
                message,
                detail,
            });
        }

        assert!(data.remaining() >= 5, "the reply has no message");
        let _flag = data.get_u8();
        let len = data.get_u32() as usize;
        assert_eq!(data.remaining(), len, "the reply holds exactly one message");
        Ok(data.to_vec())
    }
}

impl GrpcError {
    pub fn plugin_code(&self) -> Option<i32> {
        self.detail.as_ref().map(|d| d.code)
    }
}

/// Connects to a named pipe, retrying while every instance is busy.
#[cfg(windows)]
pub async fn connect_pipe(
    name: &str,
) -> std::io::Result<tokio::net::windows::named_pipe::NamedPipeClient> {
    use tokio::net::windows::named_pipe::ClientOptions;
    // ERROR_PIPE_BUSY: the server is between two instances.
    const PIPE_BUSY: i32 = 231;
    loop {
        match ClientOptions::new().open(name) {
            Err(e) if e.raw_os_error() == Some(PIPE_BUSY) => {
                tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            }
            other => return other,
        }
    }
}

// HTTP over a Unix socket, a named pipe or a loopback port.

pub enum Target<'a> {
    #[cfg(unix)]
    Unix(&'a std::path::Path),
    #[cfg(windows)]
    Pipe(&'a str),
    Tcp(u16),
    #[allow(dead_code)]
    Never(std::marker::PhantomData<&'a ()>),
}

/// A `Connection: close` request. It returns the status, the headers as
/// lower case pairs and the body.
pub async fn http_request(
    target: &Target<'_>,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
) -> std::io::Result<(u16, Vec<(String, String)>, String)> {
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: plugin\r\nConnection: close\r\n");
    for (k, v) in headers {
        request.push_str(&format!("{k}: {v}\r\n"));
    }
    request.push_str("\r\n");

    let mut raw = Vec::new();
    match target {
        #[cfg(unix)]
        Target::Unix(path) => {
            let mut s = tokio::net::UnixStream::connect(path).await?;
            s.write_all(request.as_bytes()).await?;
            s.read_to_end(&mut raw).await?;
        }
        #[cfg(windows)]
        Target::Pipe(name) => {
            let mut s = connect_pipe(name).await?;
            s.write_all(request.as_bytes()).await?;
            s.read_to_end(&mut raw).await?;
        }
        Target::Tcp(port) => {
            let mut s = tokio::net::TcpStream::connect(("127.0.0.1", *port)).await?;
            s.write_all(request.as_bytes()).await?;
            s.read_to_end(&mut raw).await?;
        }
        Target::Never(_) => unreachable!(),
    }

    let text = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    let mut lines = head.lines();
    let status_line = lines.next().unwrap_or_default();
    let status: u16 = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let headers = lines
        .filter_map(|l| l.split_once(':'))
        .map(|(k, v)| (k.trim().to_ascii_lowercase(), v.trim().to_owned()))
        .collect();
    Ok((status, headers, body.to_owned()))
}

/// Decodes the percent encoding of `grpc-message`.
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
