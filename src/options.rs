use std::collections::HashMap;

pub use crate::transport::Network;

/// The host sets these environment variables on the plugin process.
pub mod env {
    /// The plugin id from the manifest.
    pub const PLUGIN_ID: &str = "NGINX_UI_PLUGIN_ID";
    /// The protocol version the host speaks.
    pub const PLUGIN_API_VERSION: &str = "NGINX_UI_PLUGIN_API_VERSION";
    /// The only directory the plugin may write to.
    pub const PLUGIN_DATA_DIR: &str = "NGINX_UI_PLUGIN_DATA_DIR";
    /// The host version.
    pub const HOST_VERSION: &str = "NGINX_UI_VERSION";
    /// Per process secret for the `http` capability. The SDK reads it once at
    /// start and removes it from the environment, so child processes do not
    /// inherit it.
    pub const PLUGIN_HTTP_SECRET: &str = "NGINX_UI_PLUGIN_HTTP_SECRET";
    /// Set to `1` to keep the plugin on stdio only, like
    /// [`Options::without_grpc`](super::Options::without_grpc).
    pub const DISABLE_GRPC: &str = "NGINX_UI_PLUGIN_DISABLE_GRPC";
}

/// Configures [`serve_with`](crate::serve_with) and
/// [`run_with`](crate::run_with).
#[derive(Debug, Clone, Default)]
pub struct Options {
    pub(crate) disable_grpc: bool,
    pub(crate) grpc_network: Option<Network>,
    pub(crate) http_network: Option<Network>,
    pub(crate) grpc_fallback_dirs: Option<Vec<std::path::PathBuf>>,
    pub(crate) env: Option<HashMap<String, String>>,
}

impl Options {
    /// Builds the default options.
    pub fn new() -> Options {
        Options::default()
    }

    /// Keeps the plugin on stdio only: `plugin.initialize` does not advertise
    /// the `grpc` transport and no listener is opened. Setting the
    /// environment variable `NGINX_UI_PLUGIN_DISABLE_GRPC=1` has the same
    /// effect. A plugin with a `log.sink` handler keeps gRPC regardless.
    #[must_use]
    pub fn without_grpc(mut self) -> Options {
        self.disable_grpc = true;
        self
    }

    /// Reads the environment variables from `vars` instead of the process
    /// environment, and leaves the process environment alone. It makes a run
    /// independent of the process, which tests need.
    #[must_use]
    pub fn with_env<I, K, V>(mut self, vars: I) -> Options
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.env = Some(
            vars.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    /// Forces the network of the gRPC listener. It exists to exercise the
    /// Windows loopback path on any platform.
    #[doc(hidden)]
    #[must_use]
    pub fn with_grpc_network(mut self, network: Network) -> Options {
        self.grpc_network = Some(network);
        self
    }

    /// Replaces the directories the gRPC socket falls back to when the data
    /// directory cannot hold it. It exists to test the failure of the
    /// listener.
    #[doc(hidden)]
    #[must_use]
    pub fn with_grpc_fallback_dirs(mut self, dirs: Vec<std::path::PathBuf>) -> Options {
        self.grpc_fallback_dirs = Some(dirs);
        self
    }

    /// Forces the network of the `http` capability listener. It exists to
    /// exercise the Windows loopback path on any platform.
    #[doc(hidden)]
    #[must_use]
    pub fn with_http_network(mut self, network: Network) -> Options {
        self.http_network = Some(network);
        self
    }
}

/// Reads environment variables from an injected map or the process.
pub(crate) struct Env {
    vars: Option<HashMap<String, String>>,
}

impl Env {
    pub(crate) fn new(options: &Options) -> Env {
        Env {
            vars: options.env.clone(),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<String> {
        match &self.vars {
            Some(vars) => vars.get(key).cloned(),
            None => std::env::var(key).ok(),
        }
    }

    /// Returns a variable and removes it from the process environment.
    pub(crate) fn take(&self, key: &str) -> Option<String> {
        match &self.vars {
            Some(vars) => vars.get(key).cloned(),
            None => {
                let value = std::env::var(key).ok();
                std::env::remove_var(key);
                value
            }
        }
    }
}
