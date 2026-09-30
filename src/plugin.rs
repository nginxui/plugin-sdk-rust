use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use serde_json::Value;

use crate::blocklist::BlocklistHandler;
use crate::context::Context;
use crate::deploy::DeployHandler;
use crate::discovery::DiscoveryHandler;
use crate::dispatch::RtHandler;
use crate::dns01::Dns01Handler;
use crate::events::EventFn;
use crate::http::HttpHandler;
use crate::jsonrpc::BoxFuture;
use crate::logsink::LogSinkHandler;
use crate::mcp::McpHandler;
use crate::notify::NotifyHandler;
use crate::probe::ProbeHandler;
use crate::protocol::{capability, Error, EventNotification, Settings};
use crate::storage::StorageHandler;

pub(crate) type ConfigureFn =
    Arc<dyn Fn(Context, Settings) -> BoxFuture<Result<(), Error>> + Send + Sync>;
pub(crate) type ShutdownFn = Arc<dyn Fn(Context) -> BoxFuture<Result<(), Error>> + Send + Sync>;

/// Declares what a plugin process implements.
///
/// Each capability is one optional handler. Setting it wires the methods of
/// the capability and adds its name to the `capabilities` the plugin reports
/// in the handshake, which must match the manifest.
///
/// ```
/// use nginxui_plugin_sdk::{Context, Dns01Handler, Dns01Request, Error, Plugin};
///
/// struct Provider;
///
/// #[async_trait::async_trait]
/// impl Dns01Handler for Provider {
///     async fn present(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
///         if req.config.get("API_TOKEN").is_none() {
///             return Err(Error::invalid_config("API_TOKEN", "the API token is required"));
///         }
///         Ok(())
///     }
///
///     async fn clean_up(&self, _ctx: &Context, _req: Dns01Request) -> Result<(), Error> {
///         Ok(())
///     }
/// }
///
/// let plugin = Plugin::new().dns01(Provider);
/// # drop(plugin);
/// ```
#[derive(Default)]
pub struct Plugin {
    pub(crate) dns01: Option<Arc<dyn Dns01Handler>>,
    pub(crate) http: Option<Arc<dyn HttpHandler>>,
    pub(crate) notify: Option<Arc<dyn NotifyHandler>>,
    pub(crate) probe: Option<Arc<dyn ProbeHandler>>,
    pub(crate) mcp: Option<Arc<dyn McpHandler>>,
    pub(crate) storage: Option<Arc<dyn StorageHandler>>,
    pub(crate) deploy: Option<Arc<dyn DeployHandler>>,
    pub(crate) blocklist: Option<Arc<dyn BlocklistHandler>>,
    pub(crate) discovery: Option<Arc<dyn DiscoveryHandler>>,
    pub(crate) log_sink: Option<Arc<dyn LogSinkHandler>>,
    pub(crate) configure: Option<ConfigureFn>,
    pub(crate) shutdown: Option<ShutdownFn>,
    pub(crate) methods: HashMap<String, RtHandler>,
    pub(crate) events: HashMap<String, EventFn>,
    pub(crate) capabilities: Vec<String>,
}

impl std::fmt::Debug for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("capabilities", &self.declared_capabilities())
            .finish_non_exhaustive()
    }
}

impl Plugin {
    /// Builds a plugin that implements nothing but the lifecycle.
    pub fn new() -> Plugin {
        Plugin::default()
    }

    /// Serves the `dns01` capability.
    #[must_use]
    pub fn dns01(mut self, handler: impl Dns01Handler) -> Plugin {
        self.dns01 = Some(Arc::new(handler));
        self
    }

    /// Serves the `http` capability with the manifest setting `http.listen`
    /// `"unix"`. See the [`http`](crate::http) module.
    #[must_use]
    pub fn http(mut self, handler: impl HttpHandler) -> Plugin {
        self.http = Some(Arc::new(handler));
        self
    }

    /// Serves the `notify` capability.
    #[must_use]
    pub fn notify(mut self, handler: impl NotifyHandler) -> Plugin {
        self.notify = Some(Arc::new(handler));
        self
    }

    /// Serves the `probe` capability.
    #[must_use]
    pub fn probe(mut self, handler: impl ProbeHandler) -> Plugin {
        self.probe = Some(Arc::new(handler));
        self
    }

    /// Serves the `mcp` capability. [`McpTools`](crate::McpTools) is the
    /// ready-made handler that dispatches by tool name.
    #[must_use]
    pub fn mcp(mut self, handler: impl McpHandler) -> Plugin {
        self.mcp = Some(Arc::new(handler));
        self
    }

    /// Serves the `storage` capability.
    #[must_use]
    pub fn storage(mut self, handler: impl StorageHandler) -> Plugin {
        self.storage = Some(Arc::new(handler));
        self
    }

    /// Serves the `cert.deploy` capability.
    #[must_use]
    pub fn deploy(mut self, handler: impl DeployHandler) -> Plugin {
        self.deploy = Some(Arc::new(handler));
        self
    }

    /// Serves the `security.blocklist` capability.
    #[must_use]
    pub fn blocklist(mut self, handler: impl BlocklistHandler) -> Plugin {
        self.blocklist = Some(Arc::new(handler));
        self
    }

    /// Serves the `upstream.discovery` capability.
    #[must_use]
    pub fn discovery(mut self, handler: impl DiscoveryHandler) -> Plugin {
        self.discovery = Some(Arc::new(handler));
        self
    }

    /// Serves the `log.sink` capability: it receives the access log lines of
    /// the host as a stream on the gRPC transport. Setting it keeps the gRPC
    /// transport on even under [`Options::without_grpc`] or
    /// `NGINX_UI_PLUGIN_DISABLE_GRPC=1`, since the lines never travel on
    /// stdio.
    ///
    /// [`Options::without_grpc`]: crate::Options::without_grpc
    #[must_use]
    pub fn log_sink(mut self, handler: impl LogSinkHandler) -> Plugin {
        self.log_sink = Some(Arc::new(handler));
        self
    }

    /// Receives the settings map on `plugin.configure`. It is called with the
    /// latest settings, and its error is the reply of the call.
    #[must_use]
    pub fn configure<F, Fut>(mut self, f: F) -> Plugin
    where
        F: Fn(Context, Settings) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        self.configure = Some(Arc::new(move |ctx, settings| Box::pin(f(ctx, settings))));
        self
    }

    /// Runs on `plugin.shutdown` to finish in-flight work. The host waits for
    /// the reply before it sends `plugin.exit`.
    #[must_use]
    pub fn shutdown<F, Fut>(mut self, f: F) -> Plugin
    where
        F: Fn(Context) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        self.shutdown = Some(Arc::new(move |ctx| Box::pin(f(ctx))));
        self
    }

    /// Registers an extra inbound method, such as a cron target named in the
    /// manifest. It gets the `params` of the message, `null` when there are
    /// none. The names of the lifecycle and capability methods are reserved,
    /// registering one replaces the built in handler.
    #[must_use]
    pub fn method<F, Fut>(mut self, name: impl Into<String>, f: F) -> Plugin
    where
        F: Fn(Context, Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, Error>> + Send + 'static,
    {
        self.methods.insert(
            name.into(),
            Arc::new(move |ctx, params| Box::pin(f(ctx, params))),
        );
        self
    }

    /// Handles the events the manifest subscribes to, for example
    /// [`event::LOG_PATHS_CHANGED`](crate::protocol::event::LOG_PATHS_CHANGED).
    /// Other event types are ignored. Handling any event takes over
    /// `events.on`.
    #[must_use]
    pub fn event<F, Fut>(mut self, event_type: impl Into<String>, f: F) -> Plugin
    where
        F: Fn(Context, EventNotification) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.events.insert(
            event_type.into(),
            Arc::new(move |ctx, ev| Box::pin(f(ctx, ev))),
        );
        self
    }

    /// Overrides the capability list reported in `plugin.initialize`. Empty
    /// means derive it from the handlers.
    #[must_use]
    pub fn capabilities<I, S>(mut self, capabilities: I) -> Plugin
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.capabilities = capabilities.into_iter().map(Into::into).collect();
        self
    }

    /// Returns the declared list, or the one derived from the handlers.
    pub(crate) fn declared_capabilities(&self) -> Vec<String> {
        if !self.capabilities.is_empty() {
            return self.capabilities.clone();
        }
        let derived = [
            (self.dns01.is_some(), capability::DNS01),
            (self.http.is_some(), capability::HTTP),
            (self.notify.is_some(), capability::NOTIFY),
            (self.probe.is_some(), capability::PROBE),
            (self.mcp.is_some(), capability::MCP),
            (self.storage.is_some(), capability::STORAGE),
            (self.deploy.is_some(), capability::CERT_DEPLOY),
            (self.blocklist.is_some(), capability::SECURITY_BLOCKLIST),
            (self.discovery.is_some(), capability::UPSTREAM_DISCOVERY),
            (self.log_sink.is_some(), capability::LOG_SINK),
        ];
        derived
            .into_iter()
            .filter(|(set, _)| *set)
            .map(|(_, name)| name.to_owned())
            .collect()
    }
}
