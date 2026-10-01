//! Listener plumbing shared by the gRPC and HTTP servers: an accept loop with
//! a graceful stop, over Unix sockets, named pipes or loopback TCP.

use std::io;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tokio::task::{JoinHandle, JoinSet};

use crate::jsonrpc::BoxFuture;

/// The kind of listener a transport opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// A Unix domain socket in the data directory.
    Unix,
    /// A named pipe, on Windows only.
    Pipe,
    /// A loopback TCP port on `127.0.0.1`.
    Tcp,
}

impl Network {
    /// The default for the platform: a named pipe on Windows, a Unix socket
    /// everywhere else.
    pub(crate) fn platform_default() -> Network {
        if cfg!(windows) {
            Network::Pipe
        } else {
            Network::Unix
        }
    }
}

pub(crate) enum Listener {
    #[cfg(unix)]
    Unix(tokio::net::UnixListener),
    #[cfg(windows)]
    Pipe(crate::pipe::PipeListener),
    Tcp(TcpListener),
}

impl Listener {
    async fn accept(&self) -> io::Result<Stream> {
        match self {
            #[cfg(unix)]
            Listener::Unix(l) => l.accept().await.map(|(s, _)| Stream::Unix(s)),
            #[cfg(windows)]
            Listener::Pipe(l) => l.accept().await.map(Stream::Pipe),
            Listener::Tcp(l) => l.accept().await.map(|(s, _)| Stream::Tcp(s)),
        }
    }
}

pub(crate) enum Stream {
    #[cfg(unix)]
    Unix(tokio::net::UnixStream),
    #[cfg(windows)]
    Pipe(tokio::net::windows::named_pipe::NamedPipeServer),
    Tcp(TcpStream),
}

impl AsyncRead for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            Stream::Unix(s) => Pin::new(s).poll_read(cx, buf),
            #[cfg(windows)]
            Stream::Pipe(s) => Pin::new(s).poll_read(cx, buf),
            Stream::Tcp(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            #[cfg(unix)]
            Stream::Unix(s) => Pin::new(s).poll_write(cx, buf),
            #[cfg(windows)]
            Stream::Pipe(s) => Pin::new(s).poll_write(cx, buf),
            Stream::Tcp(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            Stream::Unix(s) => Pin::new(s).poll_flush(cx),
            #[cfg(windows)]
            Stream::Pipe(s) => Pin::new(s).poll_flush(cx),
            Stream::Tcp(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            Stream::Unix(s) => Pin::new(s).poll_shutdown(cx),
            #[cfg(windows)]
            Stream::Pipe(s) => Pin::new(s).poll_shutdown(cx),
            Stream::Tcp(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

/// Serves connections until stopped. Every connection is handed to the
/// `serve_conn` function together with a receiver that flips to `true` when
/// the server begins to stop, the signal for it to finish its work and close.
pub(crate) struct Server {
    stop: watch::Sender<bool>,
    task: Mutex<Option<JoinHandle<()>>>,
}

impl Server {
    pub(crate) fn spawn<F>(listener: Listener, serve_conn: F) -> Server
    where
        F: Fn(Stream, watch::Receiver<bool>) -> BoxFuture<()> + Send + Sync + 'static,
    {
        let (stop, stop_rx) = watch::channel(false);
        let task = tokio::spawn(accept_loop(listener, stop_rx, serve_conn));
        Server {
            stop,
            task: Mutex::new(Some(task)),
        }
    }

    /// Stops accepting connections and tells the running ones to finish.
    /// It returns at once, [`Server::wait`] collects the result.
    pub(crate) fn begin_stop(&self) {
        self.stop.send_replace(true);
    }

    /// Waits until every connection ended, at most `grace`.
    pub(crate) async fn wait(&self, grace: Duration) {
        let task = self.task.lock().expect("server task lock").take();
        let Some(mut task) = task else { return };
        if tokio::time::timeout(grace, &mut task).await.is_err() {
            // Aborting the accept task drops its connections.
            task.abort();
            let _ = task.await;
        }
    }

    /// Ends the server at once, dropping every connection.
    pub(crate) fn abort(&self) {
        self.stop.send_replace(true);
        if let Some(task) = self.task.lock().expect("server task lock").take() {
            task.abort();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.abort();
    }
}

async fn accept_loop<F>(listener: Listener, mut stop: watch::Receiver<bool>, serve_conn: F)
where
    F: Fn(Stream, watch::Receiver<bool>) -> BoxFuture<()> + Send + Sync + 'static,
{
    let mut connections: JoinSet<()> = JoinSet::new();
    loop {
        tokio::select! {
            biased;
            () = wait_flag(&mut stop) => break,
            accepted = listener.accept() => match accepted {
                Ok(stream) => {
                    connections.spawn(serve_conn(stream, stop.clone()));
                }
                Err(e) => {
                    crate::logger::warn(format!("accept failed: {e}"));
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            },
            Some(_) = connections.join_next(), if !connections.is_empty() => {}
        }
    }
    // New connections are refused from here on.
    drop(listener);
    while connections.join_next().await.is_some() {}
}

/// Completes once the flag is set, or when the sender is gone.
pub(crate) async fn wait_flag(rx: &mut watch::Receiver<bool>) {
    let _ = rx.wait_for(|set| *set).await;
}
