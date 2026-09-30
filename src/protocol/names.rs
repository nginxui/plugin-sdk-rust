/// JSON-RPC method names, the `rpc_name` options of the proto contract.
pub mod method {
    /// Lifecycle, request: handshake.
    pub const INITIALIZE: &str = "plugin.initialize";
    /// Lifecycle, notification: the handshake succeeded.
    pub const INITIALIZED: &str = "plugin.initialized";
    /// Lifecycle, request: new settings.
    pub const CONFIGURE: &str = "plugin.configure";
    /// Lifecycle, request: liveness probe.
    pub const PING: &str = "plugin.ping";
    /// Lifecycle, request: finish in-flight work.
    pub const SHUTDOWN: &str = "plugin.shutdown";
    /// Lifecycle, notification: exit now.
    pub const EXIT: &str = "plugin.exit";

    /// Capability `dns01`.
    pub const DNS01_PRESENT: &str = "dns01.present";
    /// Capability `dns01`.
    pub const DNS01_CLEANUP: &str = "dns01.cleanup";
    /// Capability `dns01`, optional.
    pub const DNS01_OPTIONS: &str = "dns01.options";
    /// Capability `dns01`, optional.
    pub const DNS01_CHECK: &str = "dns01.check";
    /// Capability `dns01`, optional.
    pub const DNS01_VALIDATE: &str = "dns01.validate";

    /// Capability `http`, the RPC fallback.
    pub const HTTP_HANDLE: &str = "http.handle";

    /// Capability `notify`.
    pub const NOTIFY_SEND: &str = "notify.send";
    /// Capability `notify`, optional.
    pub const NOTIFY_VALIDATE: &str = "notify.validate";

    /// Capability `probe`.
    pub const PROBE_CHECK: &str = "probe.check";

    /// Capability `mcp`.
    pub const MCP_CALL: &str = "mcp.call";

    /// Capability `storage`, optional.
    pub const STORAGE_VALIDATE: &str = "storage.validate";
    /// Capability `storage`.
    pub const STORAGE_PUT: &str = "storage.put";
    /// Capability `storage`.
    pub const STORAGE_GET: &str = "storage.get";
    /// Capability `storage`.
    pub const STORAGE_LIST: &str = "storage.list";
    /// Capability `storage`.
    pub const STORAGE_DELETE: &str = "storage.delete";

    /// Capability `cert.deploy`, optional.
    pub const DEPLOY_VALIDATE: &str = "deploy.validate";
    /// Capability `cert.deploy`.
    pub const DEPLOY_PUSH: &str = "deploy.push";

    /// Capability `security.blocklist`.
    pub const BLOCKLIST_FETCH: &str = "blocklist.fetch";

    /// Capability `upstream.discovery`.
    pub const DISCOVERY_RESOLVE: &str = "discovery.resolve";

    /// Capability `log.sink`. A client stream on the gRPC transport only, it
    /// has no JSON-RPC form and stdio answers -32601 for it (spec WIRE-12).
    pub const LOG_PUSH: &str = "log.push";

    /// Event delivery, notification.
    pub const EVENTS_ON: &str = "events.on";

    /// Host API: log a line.
    pub const HOST_LOG: &str = "host.log";
    /// Host API: read a key.
    pub const HOST_KV_GET: &str = "host.kv.get";
    /// Host API: write a key.
    pub const HOST_KV_SET: &str = "host.kv.set";
    /// Host API: delete a key.
    pub const HOST_KV_DELETE: &str = "host.kv.delete";
    /// Host API: list keys.
    pub const HOST_KV_LIST: &str = "host.kv.list";
    /// Host API: read the settings.
    pub const HOST_SETTINGS_GET: &str = "host.settings.get";
    /// Host API: read the UI locale.
    pub const HOST_I18N_LOCALE: &str = "host.i18n.locale";
    /// Host API: read a stored credential.
    pub const HOST_CREDENTIALS_GET: &str = "host.credentials.get";
    /// Host API: register a cron entry.
    pub const HOST_CRON_REGISTER: &str = "host.cron.register";
    /// Host API: drop a cron entry.
    pub const HOST_CRON_UNREGISTER: &str = "host.cron.unregister";
    /// Host API: raise a notification.
    pub const HOST_NOTIFY: &str = "host.notify";
    /// Host API: read the analytics snapshot.
    pub const HOST_METRICS_SNAPSHOT: &str = "host.metrics.snapshot";
    /// Host API: list the log files.
    pub const HOST_LOGS_LIST: &str = "host.logs.list";
    /// Host API: show or clear a processing indicator entry.
    pub const HOST_ACTIVITY_SET: &str = "host.activity.set";
}

/// Capability names a plugin may declare in its manifest.
pub mod capability {
    /// Solves DNS-01 challenges.
    pub const DNS01: &str = "dns01";
    /// Serves HTTP on a listener the host proxies to.
    pub const HTTP: &str = "http";
    /// Delivers notifications.
    pub const NOTIFY: &str = "notify";
    /// Checks the health of a target.
    pub const PROBE: &str = "probe";
    /// Serves Model Context Protocol tools.
    pub const MCP: &str = "mcp";
    /// Keeps host files in a backend.
    pub const STORAGE: &str = "storage";
    /// Pushes certificates to external targets.
    pub const CERT_DEPLOY: &str = "cert.deploy";
    /// Fetches lists of addresses to deny.
    pub const SECURITY_BLOCKLIST: &str = "security.blocklist";
    /// Resolves services into upstream servers.
    pub const UPSTREAM_DISCOVERY: &str = "upstream.discovery";
    /// Receives the nginx access log lines as they are written.
    pub const LOG_SINK: &str = "log.sink";
}

/// Permission names a plugin may request in its manifest.
pub mod permission {
    /// Private key value store.
    pub const KV: &str = "kv";
    /// Outbound network access.
    pub const NETWORK: &str = "network";
    /// Cron entries.
    pub const CRON: &str = "cron";
    /// User facing notifications.
    pub const NOTIFY: &str = "notify";
    /// Analytics snapshot.
    pub const METRICS_READ: &str = "metrics.read";
    /// Core API of the host.
    pub const CORE_API: &str = "core_api";
    /// Lets the host publish the tools of the `mcp` capability.
    pub const MCP: &str = "mcp";
    /// Lets the host send certificates and their private keys to the
    /// `cert.deploy` capability.
    pub const CERT_DEPLOY: &str = "cert.deploy";
    /// Lets the host stream the access log lines to the `log.sink`
    /// capability.
    pub const LOG_READ: &str = "log.read";
    /// Lets the plugin list the nginx log files it may read and receive the
    /// `log.paths_changed` event.
    pub const LOG_FILES: &str = "log.files";
    /// Prefix of a credential permission, followed by the credential kind,
    /// for example `credentials.read:dns`.
    pub const CREDENTIALS_READ_PREFIX: &str = "credentials.read:";
}

/// Transport names a plugin may advertise in `InitializeResult::transports`.
pub mod transport {
    /// JSON-RPC over stdin and stdout, always served.
    pub const STDIO: &str = "stdio";
    /// gRPC on a Unix socket or a loopback port.
    pub const GRPC: &str = "grpc";
}

/// Values of `server.lifecycle` in the manifest.
pub mod lifecycle {
    /// Started with the host and kept running.
    pub const RESIDENT: &str = "resident";
    /// Started when needed and stopped when idle.
    pub const ON_DEMAND: &str = "on_demand";
}
