//! The client for the `host.*` side of the protocol.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::jsonrpc::{CallError, Conn};
use crate::logger::Level;
use crate::protocol::{
    self, method, Error, HostActivitySetParams, HostCert, HostCertsListResult,
    HostCredentialsGetParams, HostCredentialsGetResult, HostCronRegisterParams,
    HostCronUnregisterParams, HostInfo, HostKvGetParams, HostKvGetResult, HostKvListParams,
    HostKvListResult, HostKvSetParams, HostLogFile, HostLogParams, HostLogsListResult,
    HostNginxConfigGetParams, HostNginxConfigGetResult, HostNginxConfigListResult,
    HostNginxSnippet, HostNginxSnippetDeleteParams, HostNginxSnippetDeleteResult,
    HostNginxSnippetListResult, HostNginxSnippetPutParams, HostNginxSnippetPutResult,
    HostNotifyParams, HostSettingsGetResult, HostSite, HostSitesListResult, InitializeParams,
    Settings,
};

/// Why a `host.*` call failed.
#[derive(Debug, thiserror::Error)]
pub enum HostError {
    /// The call was made before the host sent `plugin.initialized`.
    #[error("sdk: host calls are not allowed before plugin.initialized")]
    NotReady,
    /// The call failed on the connection or the host answered with an error,
    /// for example -32001 when the plugin lacks the permission.
    #[error(transparent)]
    Call(#[from] CallError),
}

impl HostError {
    /// The error object the host answered with, if it did.
    pub fn rpc_error(&self) -> Option<&Error> {
        match self {
            HostError::Call(CallError::Rpc(e)) => Some(e),
            _ => None,
        }
    }
}

/// A host error that carries an error object is forwarded verbatim, anything
/// else becomes an internal error.
impl From<HostError> for Error {
    fn from(e: HostError) -> Self {
        match e {
            HostError::Call(CallError::Rpc(e)) => e,
            other => Error::internal(other),
        }
    }
}

/// Describes the running plugin process and the host behind it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Info {
    /// From `NGINX_UI_PLUGIN_ID`.
    pub plugin_id: String,
    /// From `NGINX_UI_PLUGIN_API_VERSION`.
    pub api_version: i32,
    /// From `NGINX_UI_PLUGIN_DATA_DIR`, the only writable directory the
    /// plugin owns.
    pub data_dir: PathBuf,
    /// From `NGINX_UI_VERSION`, refined by the handshake.
    pub host_version: String,
    /// The handshake payload, empty until `plugin.initialize` arrives.
    pub host: HostInfo,
    /// The manifest permissions the host granted.
    pub permissions: Vec<String>,
}

#[derive(Default)]
struct State {
    settings: Settings,
    info: Info,
}

struct Inner {
    conn: Option<Conn>,
    ready: AtomicBool,
    state: RwLock<State>,
}

/// The plugin's client for the `host.*` side of the protocol.
///
/// Obtain it with [`Context::host`](crate::Context::host) or
/// [`current_host`]. It is cheap to clone. Every call needs the matching
/// manifest permission, without it the host answers -32001.
#[derive(Clone)]
pub struct Host {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Host")
            .field("ready", &self.ready())
            .finish_non_exhaustive()
    }
}

static CURRENT: Mutex<Option<Host>> = Mutex::new(None);

tokio::task_local! {
    /// The host of the plugin whose call the current task serves.
    static TASK_HOST: Host;
}

/// Returns the host client of the running plugin, or `None` when the process
/// is not serving. It is usable for logging from the moment the runtime
/// starts.
///
/// A task that serves a call of the plugin gets the host of that plugin, so
/// several plugins may run in one process, as tests do. Any other task gets
/// the client of the plugin that started last.
pub fn current_host() -> Option<Host> {
    TASK_HOST
        .try_with(Clone::clone)
        .ok()
        .or_else(|| CURRENT.lock().expect("current host lock").clone())
}

/// Runs `fut` with `host` as the host of the current task.
pub(crate) fn with_task_host<F: std::future::Future>(
    host: Host,
    fut: F,
) -> impl std::future::Future<Output = F::Output> {
    TASK_HOST.scope(host, fut)
}

pub(crate) fn set_current_host(host: Host) {
    *CURRENT.lock().expect("current host lock") = Some(host);
}

/// Clears the process wide client if it is still `host`.
pub(crate) fn clear_current_host(host: &Host) {
    let mut current = CURRENT.lock().expect("current host lock");
    if current
        .as_ref()
        .is_some_and(|h| Arc::ptr_eq(&h.inner, &host.inner))
    {
        *current = None;
    }
}

impl Host {
    pub(crate) fn new(conn: Conn, info: Info) -> Host {
        Host {
            inner: Arc::new(Inner {
                conn: Some(conn),
                ready: AtomicBool::new(false),
                state: RwLock::new(State {
                    settings: Settings::new(),
                    info,
                }),
            }),
        }
    }

    /// Builds a client that is not connected. Every call fails with
    /// [`HostError::NotReady`].
    pub fn detached() -> Host {
        Host {
            inner: Arc::new(Inner {
                conn: None,
                ready: AtomicBool::new(false),
                state: RwLock::new(State::default()),
            }),
        }
    }

    /// Reports whether `host.*` calls are allowed yet.
    pub fn ready(&self) -> bool {
        self.inner.ready.load(Ordering::SeqCst)
    }

    pub(crate) fn set_ready(&self) {
        self.inner.ready.store(true, Ordering::SeqCst);
    }

    /// Returns a snapshot of the plugin and host identity.
    pub fn info(&self) -> Info {
        self.inner.state.read().expect("host state").info.clone()
    }

    /// Returns a copy of the latest settings the host pushed through
    /// `plugin.initialize` or `plugin.configure`.
    pub fn settings(&self) -> Settings {
        self.inner
            .state
            .read()
            .expect("host state")
            .settings
            .clone()
    }

    pub(crate) fn set_settings(&self, settings: Settings) {
        self.inner.state.write().expect("host state").settings = settings;
    }

    pub(crate) fn set_handshake(&self, params: InitializeParams) {
        let mut state = self.inner.state.write().expect("host state");
        if !params.host.version.is_empty() {
            state.info.host_version.clone_from(&params.host.version);
        }
        state.info.host = params.host;
        state.info.permissions = params.permissions;
        state.settings = params.settings;
    }

    fn conn(&self) -> Result<&Conn, HostError> {
        match &self.inner.conn {
            Some(conn) if self.ready() => Ok(conn),
            _ => Err(HostError::NotReady),
        }
    }

    async fn call(&self, method: &str, params: impl Serialize) -> Result<Value, HostError> {
        Ok(self.conn()?.call(method, params).await?)
    }

    async fn call_typed<R: DeserializeOwned>(
        &self,
        method: &str,
        params: impl Serialize,
    ) -> Result<R, HostError> {
        Ok(self.conn()?.call_typed(method, params).await?)
    }

    /// Sends one line to the host log. It is a notification queued without
    /// waiting, so it never blocks on a reply. The logger falls back to
    /// stderr when it fails.
    pub fn log(&self, level: Level, msg: &str, fields: &Settings) -> Result<(), HostError> {
        let params = HostLogParams {
            level: level.as_str().to_owned(),
            message: msg.to_owned(),
            fields: fields.clone(),
        };
        Ok(self.conn()?.try_notify(method::HOST_LOG, params)?)
    }

    /// Reads a key from the private store of the plugin. It returns `None`
    /// when the key does not exist.
    pub async fn kv_get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, HostError> {
        match self.kv_get_value(key).await? {
            None => Ok(None),
            Some(value) => serde_json::from_value(value)
                .map(Some)
                .map_err(|e| CallError::Unmarshal {
                    method: method::HOST_KV_GET.to_owned(),
                    message: e.to_string(),
                })
                .map_err(HostError::from),
        }
    }

    /// Reads a key as a JSON value. It returns `None` when the key does not
    /// exist.
    pub async fn kv_get_value(&self, key: &str) -> Result<Option<Value>, HostError> {
        let res: HostKvGetResult = self
            .call_typed(
                method::HOST_KV_GET,
                HostKvGetParams {
                    key: key.to_owned(),
                },
            )
            .await?;
        Ok(res.found.then_some(res.value))
    }

    /// Stores a JSON value, up to 64 KiB encoded.
    pub async fn kv_set(&self, key: &str, value: impl Serialize) -> Result<(), HostError> {
        let value = serde_json::to_value(value).map_err(|e| CallError::Marshal {
            method: method::HOST_KV_SET.to_owned(),
            message: e.to_string(),
        })?;
        self.call(
            method::HOST_KV_SET,
            HostKvSetParams {
                key: key.to_owned(),
                value,
            },
        )
        .await?;
        Ok(())
    }

    /// Removes a key.
    pub async fn kv_delete(&self, key: &str) -> Result<(), HostError> {
        self.call(
            method::HOST_KV_DELETE,
            HostKvGetParams {
                key: key.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    /// Lists the keys under a prefix.
    pub async fn kv_list(&self, prefix: &str) -> Result<Vec<String>, HostError> {
        let res: HostKvListResult = self
            .call_typed(
                method::HOST_KV_LIST,
                HostKvListParams {
                    prefix: prefix.to_owned(),
                },
            )
            .await?;
        Ok(res.keys)
    }

    /// Re-reads the plugin settings from the host.
    pub async fn settings_get(&self) -> Result<Settings, HostError> {
        let res: HostSettingsGetResult = self.call_typed(method::HOST_SETTINGS_GET, ()).await?;
        self.set_settings(res.settings.clone());
        Ok(res.settings)
    }

    /// Returns the host UI locale.
    pub async fn locale(&self) -> Result<String, HostError> {
        let res: protocol::HostI18nLocaleResult =
            self.call_typed(method::HOST_I18N_LOCALE, ()).await?;
        Ok(res.locale)
    }

    /// Resolves a stored credential the plugin was granted access to.
    pub async fn credentials_get(
        &self,
        kind: &str,
        id: &str,
    ) -> Result<HostCredentialsGetResult, HostError> {
        self.call_typed(
            method::HOST_CREDENTIALS_GET,
            HostCredentialsGetParams {
                kind: kind.to_owned(),
                id: id.to_owned(),
            },
        )
        .await
    }

    /// Asks the host to call `method` on a schedule.
    pub async fn cron_register(
        &self,
        id: &str,
        schedule: &str,
        method_name: &str,
    ) -> Result<(), HostError> {
        self.call(
            method::HOST_CRON_REGISTER,
            HostCronRegisterParams {
                id: id.to_owned(),
                schedule: schedule.to_owned(),
                method: method_name.to_owned(),
            },
        )
        .await?;
        Ok(())
    }

    /// Drops a previously registered schedule.
    pub async fn cron_unregister(&self, id: &str) -> Result<(), HostError> {
        self.call(
            method::HOST_CRON_UNREGISTER,
            HostCronUnregisterParams { id: id.to_owned() },
        )
        .await?;
        Ok(())
    }

    /// Raises a user facing notification in the nginx-ui UI. `level` is
    /// `info`, `success`, `warning` or `error`, `details` is any JSON, pass
    /// `()` for none.
    pub async fn notify(
        &self,
        level: &str,
        title: &str,
        content: &str,
        details: impl Serialize,
    ) -> Result<(), HostError> {
        let details = serde_json::to_value(details).map_err(|e| CallError::Marshal {
            method: method::HOST_NOTIFY.to_owned(),
            message: e.to_string(),
        })?;
        self.call(
            method::HOST_NOTIFY,
            HostNotifyParams {
                level: level.to_owned(),
                title: title.to_owned(),
                content: content.to_owned(),
                details,
            },
        )
        .await?;
        Ok(())
    }

    /// Reads the host analytics snapshot into `T`.
    pub async fn metrics_snapshot<T: DeserializeOwned>(&self) -> Result<T, HostError> {
        let value = self.metrics_snapshot_value().await?;
        serde_json::from_value(value)
            .map_err(|e| CallError::Unmarshal {
                method: method::HOST_METRICS_SNAPSHOT.to_owned(),
                message: e.to_string(),
            })
            .map_err(HostError::from)
    }

    /// Reads the host analytics snapshot as a JSON value.
    pub async fn metrics_snapshot_value(&self) -> Result<Value, HostError> {
        let res: protocol::HostMetricsSnapshotResult =
            self.call_typed(method::HOST_METRICS_SNAPSHOT, ()).await?;
        Ok(res.snapshot)
    }

    /// Returns the nginx log files the host allows the plugin to read.
    /// Rotated files are not listed, read them next to the listed path. It
    /// needs the `log.files` permission.
    pub async fn logs_list(&self) -> Result<Vec<HostLogFile>, HostError> {
        let res: HostLogsListResult = self.call_typed(method::HOST_LOGS_LIST, ()).await?;
        Ok(res.logs)
    }

    /// Writes one nginx configuration snippet of the plugin. The host tests
    /// the whole configuration and reloads nginx, and puts the previous
    /// snippet back when nginx rejects the new one, which is reported as an
    /// invalid params error. The result carries the directive a person adds
    /// where the snippet should apply. It needs the `nginx.snippet`
    /// permission.
    pub async fn nginx_snippet_put(
        &self,
        name: &str,
        content: &str,
    ) -> Result<HostNginxSnippetPutResult, HostError> {
        self.call_typed(
            method::HOST_NGINX_SNIPPET_PUT,
            HostNginxSnippetPutParams {
                name: name.to_owned(),
                content: content.to_owned(),
            },
        )
        .await
    }

    /// Removes one snippet of the plugin the same way. A snippet that is
    /// still included cannot go and is kept. It returns whether a snippet
    /// was removed and needs the `nginx.snippet` permission.
    pub async fn nginx_snippet_delete(&self, name: &str) -> Result<bool, HostError> {
        let res: HostNginxSnippetDeleteResult = self
            .call_typed(
                method::HOST_NGINX_SNIPPET_DELETE,
                HostNginxSnippetDeleteParams {
                    name: name.to_owned(),
                },
            )
            .await?;
        Ok(res.removed)
    }

    /// Lists the snippets of the plugin. It needs the `nginx.snippet`
    /// permission.
    pub async fn nginx_snippet_list(&self) -> Result<Vec<HostNginxSnippet>, HostError> {
        let res: HostNginxSnippetListResult =
            self.call_typed(method::HOST_NGINX_SNIPPET_LIST, ()).await?;
        Ok(res.snippets)
    }

    /// Lists the nginx configuration files, relative to the configuration
    /// directory. It needs the `nginx.config.read` permission.
    pub async fn nginx_config_list(&self) -> Result<Vec<String>, HostError> {
        let res: HostNginxConfigListResult =
            self.call_typed(method::HOST_NGINX_CONFIG_LIST, ()).await?;
        Ok(res.files)
    }

    /// Reads one file [`Host::nginx_config_list`] returns. It needs the
    /// `nginx.config.read` permission.
    pub async fn nginx_config_get(&self, path: &str) -> Result<String, HostError> {
        let res: HostNginxConfigGetResult = self
            .call_typed(
                method::HOST_NGINX_CONFIG_GET,
                HostNginxConfigGetParams {
                    path: path.to_owned(),
                },
            )
            .await?;
        Ok(res.content)
    }

    /// Lists the sites. It needs the `sites.read` permission.
    pub async fn sites_list(&self) -> Result<Vec<HostSite>, HostError> {
        let res: HostSitesListResult = self.call_typed(method::HOST_SITES_LIST, ()).await?;
        Ok(res.sites)
    }

    /// Lists the certificates without their private keys. It needs the
    /// `certs.read` permission.
    pub async fn certs_list(&self) -> Result<Vec<HostCert>, HostError> {
        let res: HostCertsListResult = self.call_typed(method::HOST_CERTS_LIST, ()).await?;
        Ok(res.certs)
    }

    /// Shows or clears one entry of the host processing indicator. `label` is
    /// an English source string the host translates, the browser bundle
    /// supplies the translations. Entries are cleared when the plugin stops.
    pub async fn activity_set(
        &self,
        key: &str,
        label: &str,
        active: bool,
    ) -> Result<(), HostError> {
        self.call(
            method::HOST_ACTIVITY_SET,
            HostActivitySetParams {
                key: key.to_owned(),
                label: label.to_owned(),
                active,
            },
        )
        .await?;
        Ok(())
    }

    /// Shows an indicator entry and returns the guard that clears it. Call
    /// [`Activity::stop`] to clear it and wait for the host, dropping the
    /// guard clears it in the background.
    pub async fn activity(&self, key: &str, label: &str) -> Result<Activity, HostError> {
        self.activity_set(key, label, true).await?;
        Ok(Activity {
            host: self.clone(),
            key: key.to_owned(),
            label: label.to_owned(),
            active: true,
        })
    }
}

/// An entry of the host processing indicator, see [`Host::activity`].
#[derive(Debug)]
pub struct Activity {
    host: Host,
    key: String,
    label: String,
    active: bool,
}

impl Activity {
    /// Clears the entry and waits for the host.
    pub async fn stop(mut self) -> Result<(), HostError> {
        self.active = false;
        self.host.activity_set(&self.key, &self.label, false).await
    }
}

impl Drop for Activity {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        // Clearing is a call, so it needs a runtime. Without one the host
        // clears the entry when the plugin stops.
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            let host = self.host.clone();
            let key = std::mem::take(&mut self.key);
            let label = std::mem::take(&mut self.label);
            runtime.spawn(async move {
                let _ = host.activity_set(&key, &label, false).await;
            });
        }
    }
}
