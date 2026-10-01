//! The `http` capability: serving pages or an API on a listener the host
//! proxies to.
//!
//! A plugin whose manifest declares `"http": {"listen": "unix"}` sets
//! [`Plugin::http`](crate::Plugin::http). The SDK owns the listener: it opens
//! `<data dir>/http.sock` (mode 0600, a stale socket is replaced), or a
//! loopback port reported in `http_port` on Windows, before it answers
//! `plugin.initialize`, and shuts the server down on `plugin.shutdown`.
//!
//! The host removes the credentials of the client and identifies the user with
//! the headers [`HEADER_USER`] and [`HEADER_USER_ID`], see
//! [`user_from_request`]. Every request has to carry the per process secret
//! of the host in [`HEADER_PLUGIN_SECRET`]: the SDK answers 401 to any other
//! and the handler never sees the header.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::service::service_fn;
use hyper_util::rt::{TokioIo, TokioTimer};

use crate::context::Context;
use crate::host::with_task_host;
use crate::panic::catch_unwind;
use crate::protocol::{HttpUser, InitializeResult};
use crate::transport::{wait_flag, Listener, Network, Server, Stream};
use crate::util::{
    constant_time_eq, max_socket_path_len, remove_stale_socket, restrict_permissions,
};

pub use ::http::{header, HeaderMap, HeaderValue, Method, Request, Response, StatusCode, Uri};
pub use ::hyper;
pub use hyper::body::Incoming;

/// File name of the `http` capability socket inside the data directory. The
/// host looks for it there and takes no other path.
pub const HTTP_SOCKET_NAME: &str = "http.sock";

/// Header the host sets on every proxied request with the name of the
/// nginx-ui user behind it. The host removes any value the client sent.
pub const HEADER_USER: &str = "Nginx-UI-User";

/// Header the host sets on every proxied request with the id of the nginx-ui
/// user behind it. The host removes any value the client sent.
pub const HEADER_USER_ID: &str = "Nginx-UI-User-ID";

/// Request header that carries the secret the host generated for this
/// process. The SDK answers 401 to a request without the matching value and
/// removes the header before it hands the request to the handler.
pub const HEADER_PLUGIN_SECRET: &str = "Nginx-UI-Plugin-Secret";

/// How long a client may take to send the request headers.
const READ_HEADER_TIMEOUT: Duration = Duration::from_secs(30);

/// How long `plugin.shutdown` waits for the requests still running before it
/// closes their connections. The host waits only a few seconds for the reply,
/// so this stays below that.
pub(crate) const SHUTDOWN_GRACE: Duration = Duration::from_secs(3);

/// A boxed error, the error type of [`HttpBody`].
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// The body of a response of an [`HttpHandler`].
pub type HttpBody = BoxBody<Bytes, BoxError>;

/// Builds an [`HttpBody`] from bytes that are already in memory.
pub fn full_body(data: impl Into<Bytes>) -> HttpBody {
    Full::new(data.into())
        .map_err(|never| match never {})
        .boxed()
}

/// Builds a plain text response.
pub fn text_response(status: StatusCode, text: impl Into<Bytes>) -> Response<HttpBody> {
    let mut response = Response::new(full_body(text));
    *response.status_mut() = status;
    response.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

/// Serves the requests the host proxies to the plugin.
///
/// The request carries the [`Context`] of the plugin as an extension. Any
/// `async fn(Request<Incoming>) -> Response<HttpBody>` closure is a handler
/// too. Upgrades, such as WebSockets, work through
/// [`hyper::upgrade::on`].
#[async_trait]
pub trait HttpHandler: Send + Sync + 'static {
    /// Handles one request.
    async fn handle(&self, req: Request<Incoming>) -> Response<HttpBody>;
}

#[async_trait]
impl<F, Fut> HttpHandler for F
where
    F: Fn(Request<Incoming>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response<HttpBody>> + Send + 'static,
{
    async fn handle(&self, req: Request<Incoming>) -> Response<HttpBody> {
        self(req).await
    }
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: HttpHandler + ?Sized> HttpHandler for Arc<T> {
    async fn handle(&self, req: Request<Incoming>) -> Response<HttpBody> {
        (**self).handle(req).await
    }
}

/// Returns the nginx-ui user the host proxied a request for. It is the zero
/// value for a request that did not come through the host.
pub fn user_from_request<B>(req: &Request<B>) -> HttpUser {
    let header = |name: &str| {
        req.headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned()
    };
    HttpUser {
        id: header(HEADER_USER_ID),
        name: header(HEADER_USER),
    }
}

/// The running HTTP listener of one plugin process.
pub(crate) struct HttpTransport {
    server: Server,
    /// The Unix socket path, `None` on a pipe or TCP.
    socket: Option<PathBuf>,
    /// The named pipe on Windows.
    pipe: Option<String>,
    port: u16,
}

/// Opens the listener for `handler` and starts serving in the background.
/// The listener is up when this returns.
pub(crate) fn start(
    data_dir: &Path,
    network: Option<Network>,
    secret: &str,
    handler: Arc<dyn HttpHandler>,
    make_context: Arc<dyn Fn() -> Context + Send + Sync>,
) -> Result<HttpTransport, String> {
    if secret.is_empty() {
        return Err(format!(
            "{} is not set, the host provides it to every plugin serving the http capability",
            crate::env::PLUGIN_HTTP_SECRET
        ));
    }
    let network = network.unwrap_or_else(Network::platform_default);

    let (listener, socket, pipe, port) = match network {
        Network::Unix => {
            let (l, path) = listen_unix(data_dir)?;
            (l, Some(path), None, 0)
        }
        Network::Pipe => {
            let (l, name) = crate::pipe::listen()?;
            (l, None, Some(name), 0)
        }
        Network::Tcp => {
            let (l, port) = listen_tcp()?;
            (l, None, None, port)
        }
    };

    let secret: Arc<str> = Arc::from(secret);
    let server = Server::spawn(listener, move |stream, stop| {
        Box::pin(serve_connection(
            stream,
            stop,
            secret.clone(),
            handler.clone(),
            make_context.clone(),
        ))
    });
    Ok(HttpTransport {
        server,
        socket,
        pipe,
        port,
    })
}

#[cfg(unix)]
fn listen_unix(data_dir: &Path) -> Result<(Listener, PathBuf), String> {
    use std::os::unix::fs::DirBuilderExt;

    if data_dir.as_os_str().is_empty() {
        return Err("the plugin data directory is not set".to_owned());
    }
    let dir =
        std::path::absolute(data_dir).map_err(|e| format!("resolve the data directory: {e}"))?;

    // The host reaches the socket at exactly this path, there is no fallback.
    let path = dir.join(HTTP_SOCKET_NAME);
    let limit = max_socket_path_len();
    if path.as_os_str().len() > limit {
        return Err(format!(
            "socket path {} is longer than {limit} bytes",
            path.display()
        ));
    }
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&dir)
        .map_err(|e| format!("create the data directory: {e}"))?;

    // A crashed predecessor may have left its socket behind.
    remove_stale_socket(&path).map_err(|e| format!("remove the stale socket: {e}"))?;

    let listener = tokio::net::UnixListener::bind(&path)
        .map_err(|e| format!("listen on {}: {e}", path.display()))?;
    // Only the user of the host process may talk to the plugin.
    if let Err(e) = restrict_permissions(&path) {
        crate::logger::warn(format!(
            "Could not restrict the permissions of {}: {e}",
            path.display()
        ));
    }
    Ok((Listener::Unix(listener), path))
}

#[cfg(not(unix))]
fn listen_unix(_data_dir: &Path) -> Result<(Listener, PathBuf), String> {
    let _ = (
        max_socket_path_len(),
        remove_stale_socket,
        restrict_permissions,
    );
    Err("unix sockets are not supported on this platform".to_owned())
}

fn listen_tcp() -> Result<(Listener, u16), String> {
    let std_listener = std::net::TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("listen on a loopback port: {e}"))?;
    std_listener
        .set_nonblocking(true)
        .map_err(|e| format!("listen on a loopback port: {e}"))?;
    let port = std_listener
        .local_addr()
        .map_err(|e| format!("listen on a loopback port: {e}"))?
        .port();
    let listener = tokio::net::TcpListener::from_std(std_listener)
        .map_err(|e| format!("listen on a loopback port: {e}"))?;
    Ok((Listener::Tcp(listener), port))
}

async fn serve_connection(
    stream: Stream,
    mut stop: tokio::sync::watch::Receiver<bool>,
    secret: Arc<str>,
    handler: Arc<dyn HttpHandler>,
    make_context: Arc<dyn Fn() -> Context + Send + Sync>,
) {
    let service = service_fn(move |req: Request<Incoming>| {
        let secret = secret.clone();
        let handler = handler.clone();
        let make_context = make_context.clone();
        async move {
            Ok::<_, std::convert::Infallible>(
                serve_request(req, &secret, &*handler, &make_context).await,
            )
        }
    });

    let conn = hyper::server::conn::http1::Builder::new()
        .timer(TokioTimer::new())
        .header_read_timeout(READ_HEADER_TIMEOUT)
        .serve_connection(TokioIo::new(stream), service)
        .with_upgrades();
    tokio::pin!(conn);

    tokio::select! {
        result = conn.as_mut() => {
            if let Err(e) = result {
                crate::logger::debug(format!("http: connection ended: {e}"));
            }
        }
        () = wait_flag(&mut stop) => {
            // Let the request that is running finish, then close.
            conn.as_mut().graceful_shutdown();
            let _ = conn.await;
        }
    }
}

/// Refuses every request that does not carry the secret of the host,
/// WebSocket upgrades included, and hides the header from the handler. The
/// comparison takes the same time whatever the value, and the secret is never
/// written to a log or a response.
async fn serve_request(
    mut req: Request<Incoming>,
    secret: &str,
    handler: &dyn HttpHandler,
    make_context: &Arc<dyn Fn() -> Context + Send + Sync>,
) -> Response<HttpBody> {
    let values: Vec<_> = req.headers().get_all(HEADER_PLUGIN_SECRET).iter().collect();
    let authorized = values.len() == 1 && constant_time_eq(secret.as_bytes(), values[0].as_bytes());
    req.headers_mut().remove(HEADER_PLUGIN_SECRET);
    if !authorized {
        return text_response(StatusCode::UNAUTHORIZED, "unauthorized\n");
    }

    let ctx = make_context();
    req.extensions_mut().insert(ctx.clone());
    let host = ctx.host();
    match catch_unwind(Box::pin(with_task_host(host, handler.handle(req)))).await {
        Ok(response) => response,
        Err(message) => {
            crate::logger::error(format!("http: handler panicked: {message}"));
            text_response(StatusCode::INTERNAL_SERVER_ERROR, "internal server error\n")
        }
    }
}

impl HttpTransport {
    /// Adds the pipe or loopback port to the `plugin.initialize` reply. A
    /// Unix socket needs no entry: the host takes it from the data directory.
    pub(crate) fn advertise(&self, res: &mut InitializeResult) {
        if let Some(name) = &self.pipe {
            res.http_pipe.clone_from(name);
        } else if self.socket.is_none() {
            res.http_port = i32::from(self.port);
        }
    }

    /// Closes the listener and lets the requests still running finish. It
    /// returns at once, [`HttpTransport::wait`] collects the result.
    pub(crate) fn begin_stop(&self) {
        self.server.begin_stop();
    }

    /// Waits until the graceful shutdown finished or the grace period
    /// elapsed, then closes what is left and removes the socket.
    pub(crate) async fn wait(&self) {
        self.server.wait(SHUTDOWN_GRACE).await;
        self.remove_socket();
    }

    /// Ends the server at once and removes the socket.
    pub(crate) fn close(&self) {
        self.server.abort();
        self.remove_socket();
    }

    fn remove_socket(&self) {
        if let Some(path) = &self.socket {
            let _ = std::fs::remove_file(path);
        }
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        self.close();
    }
}
