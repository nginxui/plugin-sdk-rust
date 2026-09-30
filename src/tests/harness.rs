//! Runs a plugin over in-memory pipes and exposes the host side of the
//! connection.

use std::io;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Value};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::jsonrpc::{handler, Conn};
use crate::protocol::{
    method, permission, HostInfo, HostLogParams, InitializeParams, InitializeResult, Settings,
};
use crate::{env, run_until, Options, Plugin};

pub const WAIT: Duration = Duration::from_secs(5);

pub struct Harness {
    pub host: Conn,
    pub logs: mpsc::UnboundedReceiver<HostLogParams>,
    pub init: InitializeResult,
    stop: Option<oneshot::Sender<()>>,
    run: Option<JoinHandle<io::Result<()>>>,
}

/// Starts a plugin with the given environment variables.
pub struct Builder {
    plugin: Plugin,
    options: Options,
    vars: Vec<(String, String)>,
}

impl Builder {
    pub fn new(plugin: Plugin) -> Builder {
        Builder {
            plugin,
            options: Options::new(),
            vars: Vec::new(),
        }
    }

    pub fn options(mut self, f: impl FnOnce(Options) -> Options) -> Builder {
        self.options = f(self.options);
        self
    }

    pub fn var(mut self, key: &str, value: impl Into<String>) -> Builder {
        self.vars.push((key.to_owned(), value.into()));
        self
    }

    pub fn data_dir(self, dir: &Path) -> Builder {
        self.var(env::PLUGIN_DATA_DIR, dir.to_string_lossy())
    }

    /// Starts the plugin, without a handshake.
    pub fn start(self) -> Harness {
        let (host_w, plugin_in) = tokio::io::duplex(8 << 20);
        let (plugin_out, host_r) = tokio::io::duplex(8 << 20);

        let host = Conn::new(host_r, host_w);
        let (log_tx, logs) = mpsc::unbounded_channel();
        host.handle(
            method::HOST_LOG,
            handler(move |params| {
                let log_tx = log_tx.clone();
                async move {
                    if let Ok(p) = serde_json::from_value::<HostLogParams>(params) {
                        let _ = log_tx.send(p);
                    }
                    Ok(json!({}))
                }
            }),
        );
        let serving = host.clone();
        tokio::spawn(async move {
            let _ = serving.serve().await;
        });

        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        let options = self.options.with_env(self.vars);
        let run = tokio::spawn(async move {
            run_until(self.plugin, plugin_in, plugin_out, options, async {
                let _ = stop_rx.await;
            })
            .await
        });

        Harness {
            host,
            logs,
            init: InitializeResult::default(),
            stop: Some(stop_tx),
            run: Some(run),
        }
    }
}

/// Starts a plugin with default options and no environment.
pub fn start(plugin: Plugin) -> Harness {
    Builder::new(plugin).start()
}

impl Harness {
    /// Performs the handshake and returns the reply of the plugin.
    pub async fn initialize(&mut self) -> InitializeResult {
        let params = InitializeParams {
            host: HostInfo {
                version: "2.7.0".into(),
                os: "linux".into(),
                arch: "amd64".into(),
                locale: "en".into(),
            },
            settings: json!({"default_propagation_timeout_seconds": 90})
                .as_object()
                .cloned()
                .unwrap_or_else(Settings::new),
            permissions: vec![permission::NETWORK.to_owned()],
        };
        let res: InitializeResult = self
            .host
            .call_typed(method::INITIALIZE, params)
            .await
            .expect("initialize");
        self.host
            .notify(method::INITIALIZED, ())
            .await
            .expect("initialized");
        self.init = res.clone();
        res
    }

    /// Calls a method and decodes the result.
    pub async fn call(
        &self,
        method: &str,
        params: impl serde::Serialize,
    ) -> Result<Value, crate::protocol::Error> {
        match self.host.call(method, params).await {
            Ok(v) => Ok(v),
            Err(crate::jsonrpc::CallError::Rpc(e)) => Err(e),
            Err(other) => panic!("{method}: {other}"),
        }
    }

    /// Ends the run and waits for it.
    pub async fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(run) = self.run.take() {
            tokio::time::timeout(WAIT, run)
                .await
                .expect("run did not return")
                .expect("run panicked")
                .expect("run failed");
        }
    }

    /// Waits for the run to return by itself.
    pub async fn finished(&mut self) -> io::Result<()> {
        let run = self.run.take().expect("already stopped");
        tokio::time::timeout(WAIT, run)
            .await
            .expect("run did not return")
            .expect("run panicked")
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        self.host.close();
    }
}

/// A directory whose socket path fits every platform.
pub fn short_temp_dir() -> tempfile::TempDir {
    let mut builder = tempfile::Builder::new();
    builder.prefix("sdkt");
    if cfg!(unix) {
        builder.tempdir_in("/tmp").expect("temp dir")
    } else {
        builder.tempdir().expect("temp dir")
    }
}
