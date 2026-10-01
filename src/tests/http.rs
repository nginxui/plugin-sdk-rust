#![cfg_attr(not(unix), allow(unused_imports, dead_code))]

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use http::{HeaderMap, Request, Response, StatusCode};
use hyper::body::Incoming;
use serde_json::json;
use tokio::sync::{mpsc, Notify};

use super::clients::{http_request, Target};
use super::grpc::GrpcHandler;
use super::harness::{short_temp_dir, Builder, Harness, WAIT};
use crate::http::{
    text_response, user_from_request, HttpBody, HEADER_PLUGIN_SECRET, HEADER_USER, HEADER_USER_ID,
    HTTP_SOCKET_NAME,
};
use crate::protocol::{capability, code, method};
use crate::{env, Network, Options, Plugin};

const SECRET: &str = "test-secret-value";

/// Answers with the user the host proxied the request for.
async fn echo(req: Request<Incoming>) -> Response<HttpBody> {
    match req.uri().path() {
        "/whoami" => {
            let user = user_from_request(&req);
            text_response(StatusCode::OK, format!("{}:{}", user.id, user.name))
        }
        _ => text_response(StatusCode::NOT_FOUND, "not found"),
    }
}

/// A plugin with the secret the host would hand it.
fn http_builder(plugin: Plugin, dir: &tempfile::TempDir) -> Builder {
    Builder::new(plugin)
        .data_dir(dir.path())
        .var(env::PLUGIN_HTTP_SECRET, SECRET)
}

fn with_secret<'a>(extra: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
    let mut headers = vec![(HEADER_PLUGIN_SECRET, SECRET)];
    headers.extend_from_slice(extra);
    headers
}

fn unix_options(o: Options) -> Options {
    o.without_grpc().with_http_network(Network::Unix)
}

async fn get_ok(target: &Target<'_>, path: &str, headers: &[(&str, &str)]) -> String {
    let (status, _, body) = http_request(target, "GET", path, headers).await.unwrap();
    assert_eq!(status, 200, "GET {path}: {body}");
    body
}

#[cfg(unix)]
mod unix {
    use super::*;

    #[tokio::test]
    async fn http_serves_on_the_data_dir_socket() {
        let dir = short_temp_dir();
        let mut h = http_builder(Plugin::new().http(echo), &dir)
            .options(unix_options)
            .start();
        let init = h.initialize().await;

        assert_eq!(init.capabilities, [capability::HTTP]);
        assert_eq!(init.http_port, 0, "no port on a Unix socket");

        let socket = dir.path().join(HTTP_SOCKET_NAME);
        let meta = std::fs::symlink_metadata(&socket).expect("socket");
        {
            use std::os::unix::fs::{FileTypeExt, PermissionsExt};
            assert!(meta.file_type().is_socket());
            assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        }

        let headers = with_secret(&[(HEADER_USER_ID, "7"), (HEADER_USER, "alice")]);
        assert_eq!(
            get_ok(&Target::Unix(&socket), "/whoami", &headers).await,
            "7:alice"
        );

        h.stop().await;
        assert!(!socket.exists(), "the socket still exists after stop");
    }

    #[tokio::test]
    async fn http_replaces_a_stale_socket() {
        let dir = short_temp_dir();
        let socket = dir.path().join(HTTP_SOCKET_NAME);

        // A listener that keeps its file on drop is what a crashed process
        // leaves behind.
        drop(std::os::unix::net::UnixListener::bind(&socket).unwrap());
        assert!(socket.exists(), "the stale socket was not left behind");

        let _h = {
            let mut h = http_builder(Plugin::new().http(echo), &dir)
                .options(unix_options)
                .start();
            h.initialize().await;
            h
        };
        assert_eq!(
            get_ok(&Target::Unix(&socket), "/whoami", &with_secret(&[])).await,
            ":"
        );
    }

    #[tokio::test]
    async fn http_serves_next_to_grpc() {
        let dir = short_temp_dir();
        let mut h = http_builder(Plugin::new().dns01(GrpcHandler).http(echo), &dir)
            .options(|o| {
                o.with_grpc_network(Network::Unix)
                    .with_http_network(Network::Unix)
            })
            .start();
        let init = h.initialize().await;

        assert!(!init.rpc_socket.is_empty(), "{init:?}");
        let socket = dir.path().join(HTTP_SOCKET_NAME);
        assert_eq!(
            get_ok(&Target::Unix(&socket), "/whoami", &with_secret(&[])).await,
            ":"
        );
        assert_eq!(init.capabilities.join(","), "dns01,http");
    }

    async fn handshake_error(plugin: Plugin, vars: Vec<(&str, String)>) -> crate::protocol::Error {
        let mut builder = Builder::new(plugin).options(unix_options);
        for (k, v) in vars {
            builder = builder.var(k, v);
        }
        let h = builder.start();
        let err = h
            .host
            .call_typed::<crate::protocol::InitializeResult>(
                method::INITIALIZE,
                crate::protocol::InitializeParams::default(),
            )
            .await
            .expect_err("the handshake must fail");
        match err {
            crate::jsonrpc::CallError::Rpc(e) => e,
            other => panic!("{other}"),
        }
    }

    #[tokio::test]
    async fn http_listen_failure_fails_the_handshake() {
        let base = short_temp_dir();
        let cases = [
            ("no data dir", String::new()),
            (
                "path too long",
                base.path()
                    .join("d".repeat(120))
                    .to_string_lossy()
                    .into_owned(),
            ),
            ("unwritable dir", "/dev/null/data".to_owned()),
        ];
        for (name, data_dir) in cases {
            let err = handshake_error(
                Plugin::new().http(echo),
                vec![
                    (env::PLUGIN_DATA_DIR, data_dir),
                    (env::PLUGIN_HTTP_SECRET, SECRET.into()),
                ],
            )
            .await;
            assert_eq!(err.code, code::INTERNAL_ERROR, "{name}");
            assert!(
                err.message.contains("http listener"),
                "{name}: {}",
                err.message
            );
        }
    }

    #[tokio::test]
    async fn http_needs_the_secret_to_start() {
        let dir = short_temp_dir();
        let err = handshake_error(
            Plugin::new().http(echo),
            vec![
                (
                    env::PLUGIN_DATA_DIR,
                    dir.path().to_string_lossy().into_owned(),
                ),
                (env::PLUGIN_HTTP_SECRET, String::new()),
            ],
        )
        .await;
        assert!(
            err.message.contains(env::PLUGIN_HTTP_SECRET),
            "{}",
            err.message
        );
    }

    async fn slow_plugin(
        dir: &tempfile::TempDir,
        started: Arc<Notify>,
        release: Arc<Notify>,
        shutdown_called: Option<mpsc::UnboundedSender<()>>,
    ) -> Harness {
        let handler = move |req: Request<Incoming>| {
            let started = started.clone();
            let release = release.clone();
            async move {
                match req.uri().path() {
                    "/slow" => {
                        started.notify_one();
                        release.notified().await;
                        text_response(StatusCode::OK, "done")
                    }
                    "/hang" => {
                        started.notify_one();
                        std::future::pending::<()>().await;
                        unreachable!()
                    }
                    _ => text_response(StatusCode::NOT_FOUND, "not found"),
                }
            }
        };
        let mut plugin = Plugin::new().http(handler);
        if let Some(tx) = shutdown_called {
            plugin = plugin.shutdown(move |_| {
                let tx = tx.clone();
                async move {
                    let _ = tx.send(());
                    Ok(())
                }
            });
        }
        let mut h = http_builder(plugin, dir).options(unix_options).start();
        h.initialize().await;
        h
    }

    #[tokio::test]
    async fn http_shutdown_lets_a_running_request_finish() {
        let dir = short_temp_dir();
        let (started, release) = (Arc::new(Notify::new()), Arc::new(Notify::new()));
        let (tx, mut shutdown_called) = mpsc::unbounded_channel();
        let h = slow_plugin(&dir, started.clone(), release.clone(), Some(tx)).await;
        let socket = dir.path().join(HTTP_SOCKET_NAME);

        let request = {
            let socket = socket.clone();
            tokio::spawn(async move {
                http_request(&Target::Unix(&socket), "GET", "/slow", &with_secret(&[])).await
            })
        };
        tokio::time::timeout(WAIT, started.notified())
            .await
            .unwrap();

        let host = h.host.clone();
        let mut shutdown = tokio::spawn(async move { host.call(method::SHUTDOWN, ()).await });

        // The plugin hook ran, the reply waits for the request.
        tokio::time::timeout(WAIT, shutdown_called.recv())
            .await
            .expect("Plugin::shutdown was not called");
        tokio::select! {
            out = &mut shutdown => panic!("shutdown answered before the request finished: {out:?}"),
            () = tokio::time::sleep(Duration::from_millis(200)) => {}
        }

        // New connections are refused while the running request drains.
        assert!(
            tokio::net::UnixStream::connect(&socket).await.is_err(),
            "the socket accepts new connections during shutdown"
        );

        release.notify_one();
        let (status, _, body) = request.await.unwrap().unwrap();
        assert_eq!((status, body.as_str()), (200, "done"));
        tokio::time::timeout(WAIT, shutdown)
            .await
            .expect("shutdown did not answer")
            .unwrap()
            .unwrap();
        assert!(!socket.exists(), "the socket still exists after shutdown");
    }

    #[tokio::test]
    async fn http_shutdown_closes_a_request_that_outlives_the_grace() {
        let dir = short_temp_dir();
        let (started, release) = (Arc::new(Notify::new()), Arc::new(Notify::new()));
        let h = slow_plugin(&dir, started.clone(), release, None).await;
        let socket = dir.path().join(HTTP_SOCKET_NAME);

        let hanging = {
            let socket = socket.clone();
            tokio::spawn(async move {
                let _ =
                    http_request(&Target::Unix(&socket), "GET", "/hang", &with_secret(&[])).await;
            })
        };
        tokio::time::timeout(WAIT, started.notified())
            .await
            .unwrap();

        let begin = Instant::now();
        h.host.call(method::SHUTDOWN, ()).await.unwrap();
        let took = begin.elapsed();
        assert!(
            took < crate::http::SHUTDOWN_GRACE + Duration::from_secs(2),
            "shutdown took {took:?}"
        );
        assert!(
            took >= crate::http::SHUTDOWN_GRACE - Duration::from_millis(500),
            "shutdown took {took:?}"
        );
        tokio::time::timeout(WAIT, hanging)
            .await
            .expect("the hanging request was not closed")
            .unwrap();
    }

    #[tokio::test]
    async fn http_rejects_requests_without_the_secret() {
        let dir = short_temp_dir();
        let seen: Arc<Mutex<Vec<HeaderMap>>> = Arc::default();
        let recorded = seen.clone();
        let handler = move |req: Request<Incoming>| {
            let recorded = recorded.clone();
            async move {
                recorded.lock().unwrap().push(req.headers().clone());
                text_response(StatusCode::OK, "ok")
            }
        };
        let mut h = http_builder(Plugin::new().http(handler), &dir)
            .options(unix_options)
            .start();
        h.initialize().await;
        let socket = dir.path().join(HTTP_SOCKET_NAME);
        let target = Target::Unix(&socket);

        let too_short = &SECRET[..5];
        let too_long = format!("{SECRET}x");
        let cases: Vec<(&str, Vec<(&str, &str)>)> = vec![
            ("missing", vec![]),
            ("empty", vec![(HEADER_PLUGIN_SECRET, "")]),
            ("wrong", vec![(HEADER_PLUGIN_SECRET, "nope")]),
            ("prefix", vec![(HEADER_PLUGIN_SECRET, too_short)]),
            ("longer", vec![(HEADER_PLUGIN_SECRET, too_long.as_str())]),
            (
                "correct+wrong",
                vec![
                    (HEADER_PLUGIN_SECRET, SECRET),
                    (HEADER_PLUGIN_SECRET, "nope"),
                ],
            ),
            ("in the wrong header", vec![("X-Other", SECRET)]),
        ];
        for (name, headers) in cases {
            let (status, _, body) = http_request(&target, "GET", "/x", &headers).await.unwrap();
            assert_eq!(status, 401, "{name}");
            assert!(
                !body.contains(SECRET),
                "{name}: the response leaks the secret"
            );
        }
        assert!(
            seen.lock().unwrap().is_empty(),
            "a rejected request reached the handler"
        );

        // The right value gets through, and the handler never sees the header.
        let (status, _, body) =
            http_request(&target, "GET", "/x", &[(HEADER_PLUGIN_SECRET, SECRET)])
                .await
                .unwrap();
        assert_eq!((status, body.as_str()), (200, "ok"));
        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert!(
            seen[0].get(HEADER_PLUGIN_SECRET).is_none(),
            "the handler saw the secret header"
        );
    }

    #[tokio::test]
    async fn http_rejects_websocket_upgrades_without_the_secret() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        let dir = short_temp_dir();
        let mut h = http_builder(Plugin::new().http(echo), &dir)
            .options(unix_options)
            .start();
        h.initialize().await;

        let stream = tokio::net::UnixStream::connect(dir.path().join(HTTP_SOCKET_NAME))
            .await
            .unwrap();
        let (read, mut write) = stream.into_split();
        write
            .write_all(
                b"GET /whoami HTTP/1.1\r\nHost: plugin\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\
                  Sec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\r\n",
            )
            .await
            .unwrap();
        let mut line = String::new();
        BufReader::new(read).read_line(&mut line).await.unwrap();
        assert!(line.contains("401"), "{line}");
    }

    #[tokio::test]
    async fn http_upgrades_pass_through_with_the_secret() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let dir = short_temp_dir();
        let handler = |mut req: Request<Incoming>| async move {
            let upgrade = hyper::upgrade::on(&mut req);
            tokio::spawn(async move {
                if let Ok(upgraded) = upgrade.await {
                    let mut io = hyper_util::rt::TokioIo::new(upgraded);
                    let mut buf = [0u8; 4];
                    if io.read_exact(&mut buf).await.is_ok() {
                        let _ = io.write_all(&buf).await;
                    }
                }
            });
            let mut response = Response::new(crate::http::full_body(""));
            *response.status_mut() = StatusCode::SWITCHING_PROTOCOLS;
            response
                .headers_mut()
                .insert("connection", "upgrade".parse().unwrap());
            response
                .headers_mut()
                .insert("upgrade", "echo".parse().unwrap());
            response
        };
        let mut h = http_builder(Plugin::new().http(handler), &dir)
            .options(unix_options)
            .start();
        h.initialize().await;

        let mut stream = tokio::net::UnixStream::connect(dir.path().join(HTTP_SOCKET_NAME))
            .await
            .unwrap();
        stream
            .write_all(
                format!(
                    "GET /ws HTTP/1.1\r\nHost: plugin\r\nConnection: Upgrade\r\nUpgrade: echo\r\n{HEADER_PLUGIN_SECRET}: {SECRET}\r\n\r\n"
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        let mut head = vec![0u8; 256];
        let n = tokio::time::timeout(WAIT, stream.read(&mut head))
            .await
            .unwrap()
            .unwrap();
        assert!(String::from_utf8_lossy(&head[..n]).starts_with("HTTP/1.1 101"));
        stream.write_all(b"ping").await.unwrap();
        let mut echoed = [0u8; 4];
        tokio::time::timeout(WAIT, stream.read_exact(&mut echoed))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(&echoed, b"ping");
    }

    #[allow(dead_code)]
    fn _path(_: &Path) {}
}

#[tokio::test]
async fn http_reports_a_loopback_port() {
    let dir = short_temp_dir();
    let mut h = http_builder(Plugin::new().http(echo), &dir)
        .options(|o| o.without_grpc().with_http_network(Network::Tcp))
        .start();
    let init = h.initialize().await;

    assert!(init.http_port != 0, "http_port is not set: {init:?}");
    assert!(
        !dir.path().join(HTTP_SOCKET_NAME).exists(),
        "a socket exists next to a loopback port"
    );

    let port = u16::try_from(init.http_port).unwrap();
    let target = Target::Tcp(port);
    let headers = with_secret(&[(HEADER_USER_ID, "1"), (HEADER_USER, "bob")]);
    assert_eq!(get_ok(&target, "/whoami", &headers).await, "1:bob");

    // The loopback port needs the secret as well.
    let (status, _, _) = http_request(&target, "GET", "/whoami", &[]).await.unwrap();
    assert_eq!(status, 401);

    h.stop().await;
    let refused = tokio::time::timeout(
        Duration::from_secs(1),
        tokio::net::TcpStream::connect(("127.0.0.1", port)),
    )
    .await;
    assert!(
        matches!(refused, Ok(Err(_))),
        "the port still accepts connections after stop"
    );
}

#[cfg(windows)]
#[tokio::test]
async fn http_reports_a_pipe() {
    let dir = short_temp_dir();
    let mut h = http_builder(Plugin::new().http(echo), &dir)
        .options(|o| o.without_grpc())
        .start();
    let init = h.initialize().await;

    assert!(
        init.http_pipe.starts_with(r"\\.\pipe\") && init.http_port == 0,
        "the default on Windows is a pipe: {init:?}"
    );

    let target = Target::Pipe(&init.http_pipe);
    let headers = with_secret(&[(HEADER_USER_ID, "1"), (HEADER_USER, "bob")]);
    assert_eq!(get_ok(&target, "/whoami", &headers).await, "1:bob");

    // The pipe needs the secret as well.
    let (status, _, _) = http_request(&target, "GET", "/whoami", &[]).await.unwrap();
    assert_eq!(status, 401);

    h.stop().await;
}

#[test]
fn user_from_request_reads_the_headers() {
    let mut req = Request::new(());
    assert_eq!(
        user_from_request(&req),
        crate::protocol::HttpUser::default()
    );
    req.headers_mut()
        .insert("nginx-ui-user-id", "3".parse().unwrap());
    req.headers_mut()
        .insert("nginx-ui-user", "carol".parse().unwrap());
    let user = user_from_request(&req);
    assert_eq!((user.id.as_str(), user.name.as_str()), ("3", "carol"));
}

#[tokio::test]
async fn a_panicking_http_handler_answers_500() {
    let dir = short_temp_dir();
    let handler = |_req: Request<Incoming>| async move {
        if true {
            panic!("kaboom");
        }
        text_response(StatusCode::OK, "unreachable")
    };
    let mut h = http_builder(Plugin::new().http(handler), &dir)
        .options(|o| o.without_grpc().with_http_network(Network::Tcp))
        .start();
    let init = h.initialize().await;
    let (status, _, body) = http_request(
        &Target::Tcp(u16::try_from(init.http_port).unwrap()),
        "GET",
        "/",
        &with_secret(&[]),
    )
    .await
    .unwrap();
    assert_eq!(status, 500);
    assert!(!body.contains("kaboom"));
}

#[tokio::test]
async fn run_removes_the_secret_from_the_environment() {
    // Only a run without injected variables touches the process environment.
    const KEY: &str = "NGINX_UI_PLUGIN_HTTP_SECRET";
    std::env::set_var(KEY, SECRET);
    let (host_w, plugin_in) = tokio::io::duplex(1 << 16);
    let (plugin_out, _host_r) = tokio::io::duplex(1 << 16);
    let run = tokio::spawn(crate::run(Plugin::new(), plugin_in, plugin_out));
    // The variable is taken at the start, whatever the plugin serves.
    tokio::time::timeout(WAIT, async {
        while std::env::var_os(KEY).is_some() {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("the secret is still in the environment");

    let mut host_w = host_w;
    tokio::io::AsyncWriteExt::shutdown(&mut host_w)
        .await
        .unwrap();
    tokio::time::timeout(WAIT, run)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let _ = json!({});
}
