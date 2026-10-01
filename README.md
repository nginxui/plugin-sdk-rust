# nginxui-plugin-sdk

Rust SDK for writing [NGINX UI](https://github.com/0xJacky/nginx-ui) plugins.

A plugin is an ordinary executable. The host starts it, speaks bidirectional
JSON-RPC 2.0 framed as NDJSON over the plugin's **stdin** and **stdout**, and
stops it again. `stdout` carries protocol traffic only; every human readable
line must go to `stderr`, which the SDK logger does for you. Never `println!`
in a plugin.

```bash
cargo add nginxui-plugin-sdk async-trait
```

The minimum supported Rust version is **1.88**. It is set by the gRPC
dependency `tonic` and has not been checked with an older toolchain.

## Example

```rust,no_run
use nginxui_plugin_sdk::protocol::{Dns01OptionsParams, Dns01OptionsResult};
use nginxui_plugin_sdk::{redact, Context, Dns01Handler, Dns01Request, Error, Plugin};

struct Provider;

#[async_trait::async_trait]
impl Dns01Handler for Provider {
    // Publishes the challenge TXT record.
    async fn present(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        let token = req.config.get("MY_API_TOKEN").cloned().unwrap_or_default();
        if token.is_empty() {
            return Err(Error::invalid_config("MY_API_TOKEN", "the API token is required"));
        }

        nginxui_plugin_sdk::info!(
            "publishing {} for {} (token {})",
            req.effective_fqdn,
            req.domain,
            redact(&token)
        );

        // Talk to the vendor API here.
        Ok(())
    }

    // Removes what `present` published.
    async fn clean_up(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        nginxui_plugin_sdk::info!("removing {}", req.effective_fqdn);
        Ok(())
    }

    // Optional: report the propagation timings to the host.
    async fn options(
        &self,
        _ctx: &Context,
        _params: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        Ok(Dns01OptionsResult {
            propagation_timeout_seconds: 120,
            polling_interval_seconds: 2,
            ..Default::default()
        })
    }
}

fn main() {
    let plugin = Plugin::new()
        .dns01(Provider)
        // Called on plugin.configure and whenever the user saves settings.
        .configure(|ctx, settings| async move {
            ctx.host()
                .notify("info", "Reconfigured", &format!("{} settings", settings.len()), ())
                .await?;
            Ok(())
        });

    nginxui_plugin_sdk::serve_blocking(plugin);
}
```

`serve_blocking` blocks: it builds a tokio runtime, registers the lifecycle
methods, wires the capability methods your handlers implement, and exits on
`plugin.exit`, on end of stdin or on SIGINT/SIGTERM. Inside your own runtime
use `serve(plugin).await`, which never returns either. In tests use
`run(plugin, reader, writer).await` to drive the same wiring over in-memory
pipes, it returns when the host asked the process to stop and installs no
signal handler. `run_until` also returns when a future of your choice
completes, `serve_with`, `run_with` and `serve_blocking_with` take
[`Options`](#transports). `examples/dns01.rs` is this example.

Handlers get a `&Context`. `ctx.host()` is the [host client](#the-host-client),
`ctx.remaining()` the time left until the deadline of the call and
`ctx.cancelled().await` completes when the plugin is stopping or the deadline
passes. A handler future is dropped when the call is abandoned, so stopping
early needs no code.

Every handler returns `Result<_, Error>`. `?` turns an `std::io::Error`, a
`serde_json::Error`, a `String`, a `&str` and a `HostError` into an `Error`;
use `.map_err(Error::internal)` for any other error type.

## Capabilities

Each capability is one method of `Plugin`. Setting it wires the methods of the
capability and adds its name to the `capabilities` the plugin reports in the
handshake, which must match the manifest (`Plugin::capabilities` overrides the
derived list). The manifest block of each capability is described in the
[specification](https://github.com/nginxui/plugin-spec).

| `Plugin` method | Capability | Methods | Handler |
| --- | --- | --- | --- |
| `dns01` | `dns01` | `dns01.present`, `dns01.cleanup`, optional `dns01.validate`, `dns01.options`, `dns01.check` | `Dns01Handler` |
| `http` | `http` | none, a listener the host proxies to | `HttpHandler`, or an `async fn` |
| `notify` | `notify` | `notify.send`, optional `notify.validate` | `NotifyHandler` |
| `probe` | `probe` | `probe.check` | `ProbeHandler` |
| `mcp` | `mcp` | `mcp.call` | `McpHandler`, or the ready-made `McpTools` |
| `storage` | `storage` | `storage.put`, `storage.get`, `storage.list`, `storage.delete`, optional `storage.validate` | `StorageHandler` |
| `deploy` | `cert.deploy` | `deploy.push`, optional `deploy.validate` | `DeployHandler` |
| `blocklist` | `security.blocklist` | `blocklist.fetch` | `BlocklistHandler` |
| `discovery` | `upstream.discovery` | `discovery.resolve` | `DiscoveryHandler` |
| `log_sink` | `log.sink` | the `log.push` stream, gRPC only | `LogSinkHandler` |

The handler traits are `async_trait` traits. An optional method has a default
body that answers `-32002` (Unsupported), so a handler that does not override
it makes the host fall back or treat it as "no opinion". Every handler trait is
also implemented for `Arc<T>`, to keep a handle on state the plugin shares.

`Plugin` has more builders: `configure` receives the settings on
`plugin.configure`, `shutdown` runs on `plugin.shutdown`, `method` registers an
extra inbound method such as a cron target, and `event` handles an event the
manifest subscribes to.

### HTTP API

A plugin that serves pages or an API declares the `http` capability with
`"http": {"listen": "unix"}` in its manifest and sets `Plugin::http` to an
`HttpHandler`, which is any `async fn(Request<Incoming>) -> Response<HttpBody>`.
NGINX UI proxies `/api/plugins/<id>/http/...` to it after its own
authentication, with WebSocket upgrades (`hyper::upgrade::on`) and streamed
responses passing through.

```rust,no_run
use nginxui_plugin_sdk::http::{
    text_response, user_from_request, HttpBody, Incoming, Request, Response, StatusCode,
};
use nginxui_plugin_sdk::Plugin;

async fn hello(req: Request<Incoming>) -> Response<HttpBody> {
    let user = user_from_request(&req);
    text_response(StatusCode::OK, format!("hello {}", user.name))
}

fn main() {
    nginxui_plugin_sdk::serve_blocking(Plugin::new().http(hello));
}
```

The `http` module re-exports the types of the `http` crate and `hyper`. A
router such as axum's serves a request with `router.oneshot(req.map(Body::new))`
inside such a function. The request carries the `Context` of the plugin in its
extensions.

The SDK owns the listener, so the plugin does not open sockets itself:

* On Linux and macOS it listens on the Unix socket
  `$NGINX_UI_PLUGIN_DATA_DIR/http.sock` with mode `0600` and replaces a socket
  left behind by a process that did not exit cleanly. The host looks for the
  file at exactly this path, so unlike the gRPC socket there is no fallback for
  a data directory whose path is too long (103 bytes on macOS and the BSDs,
  107 on Linux).
* On Windows it creates a named pipe under a random name that only the user
  of the plugin may open, refusing remote clients, and reports it as
  `http_pipe` in the `plugin.initialize` reply, which is where the host reads
  it.

The listener is open before the `plugin.initialize` reply is sent. When it
cannot be opened the reply is an internal error and the handshake fails, so the
host shows the plugin as broken instead of proxying into nothing.

On `plugin.shutdown` the server stops accepting connections at once, runs the
`Plugin::shutdown` hook (use it to end long lived streams and WebSockets, which
the server does not track), then waits up to three seconds for the requests
still running and closes what is left. `Plugin::capabilities` need not list
`http` when `Plugin::http` is set.

The host removes the `Authorization` and `Cookie` headers and sets
`Nginx-UI-User` and `Nginx-UI-User-ID`, which `user_from_request` reads.

Every request also has to carry a secret. The host generates a random one for
each process start, hands it over in `NGINX_UI_PLUGIN_HTTP_SECRET` and sends it
in the header `Nginx-UI-Plugin-Secret` of every proxied request, on the Unix
socket and on the Windows named pipe alike. The SDK reads the variable once
at start and removes it from the environment, so child processes do not inherit
it. It answers `401` to a request without the matching value (compared in
constant time, WebSocket upgrades included) and takes the header off the
request before your handler sees it. Because of that a handler can trust the
user headers on every platform, even though other local processes may reach
the listener. Never log the secret. When the variable is missing the handshake
fails with an error that names it: the host always sets it.

### Notification channels

The host offers every channel of the manifest's `notify` block next to its
built-in notification channels and calls `send` whenever a notification is
routed to one. `req.config` holds the values of the channel form, `req.title`
and `req.content` are already translated plain text, and `req.severity` is
`info`, `success`, `warning` or `error`.

```rust,no_run
use nginxui_plugin_sdk::protocol::Config;
use nginxui_plugin_sdk::{Context, Error, NotifyHandler, NotifyRequest};

struct Chat;

#[async_trait::async_trait]
impl NotifyHandler for Chat {
    async fn send(&self, _ctx: &Context, req: NotifyRequest) -> Result<(), Error> {
        let hook = req.config.get("webhook_url").filter(|h| !h.is_empty());
        let Some(_hook) = hook else {
            return Err(Error::invalid_config("webhook_url", "webhook_url is required"));
        };
        // Post req.title and req.content to the vendor here.
        Ok(())
    }

    // Optional. It must not send anything.
    async fn validate(&self, _ctx: &Context, _channel: &str, config: &Config) -> Result<(), Error> {
        match config.get("webhook_url") {
            Some(url) if url.starts_with("https://") => Ok(()),
            _ => Err(Error::invalid_config("webhook_url", "webhook_url must be an https URL")),
        }
    }
}
```

### Health check probes

The host offers every kind of the manifest's `probe` block as a check method of
a site's health check and calls `check` on its schedule. An unhealthy or
unreachable target is a result, not an error: return `ProbeResult::down` with
a message, and an error only when the check itself could not run. The context
expires after `req.timeout_seconds`, and the call is dropped at that moment.

```rust,no_run
use std::time::Instant;

use nginxui_plugin_sdk::{Context, Error, ProbeHandler, ProbeRequest, ProbeResult};

struct Banner;

#[async_trait::async_trait]
impl ProbeHandler for Banner {
    async fn check(&self, _ctx: &Context, req: ProbeRequest) -> Result<ProbeResult, Error> {
        let started = Instant::now();
        let Some(port) = req.config.get("port") else {
            return Err(Error::invalid_config("port", "port is required"));
        };
        let address = format!("{}:{port}", req.target);
        match tokio::net::TcpStream::connect(address).await {
            Ok(_) => Ok(ProbeResult::up(started.elapsed())),
            Err(e) => Ok(ProbeResult::down(started.elapsed(), e.to_string())),
        }
    }
}
```

### MCP tools

The host publishes every tool of the manifest's `mcp` block on its Model
Context Protocol server under the name `<plugin id with dots replaced by
underscores>__<tool name>` and forwards each call with the unprefixed name. The
manifest must request the `mcp` permission. Arguments come from an AI
assistant: validate them before use. Return `McpResult::error` for a tool that
ran and failed, so the assistant sees why; `McpTools` answers an unknown tool
with `-32602`.

```rust,no_run
use nginxui_plugin_sdk::{McpResult, McpTools, Plugin};

fn main() {
    let tools = McpTools::new().tool("purge_cache", |_ctx, args| async move {
        match args.get("zone").and_then(|z| z.as_str()) {
            Some(zone) if !zone.is_empty() => Ok(McpResult::text(format!("purged {zone}"))),
            _ => Ok(McpResult::error("zone is required")),
        }
    });
    nginxui_plugin_sdk::serve_blocking(Plugin::new().mcp(tools));
}
```

### Storage backends

The host offers every backend of the manifest's `storage` block next to its
built-in storage, for example as a destination of automatic backups. File
contents never travel in a message: for `put` the host has placed the file at
`req.source_path`, for `get` you write the object to `req.target_path`, both
inside `<data dir>/exchange/`, and the host removes them after your reply. Keys
are relative `/` separated paths; the SDK answers a malformed key with `-32602`
before your handler runs, so building a vendor path from it is safe. `delete` of
a missing object must succeed, and `list` takes a plain string prefix.

```rust,no_run
use nginxui_plugin_sdk::protocol::StorageObject;
use nginxui_plugin_sdk::{
    Context, Error, StorageDeleteRequest, StorageGetRequest, StorageHandler, StorageListRequest,
    StoragePutRequest,
};

struct Dav;

#[async_trait::async_trait]
impl StorageHandler for Dav {
    async fn put(&self, _ctx: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        let file = std::fs::File::open(&req.source_path)?;
        // Upload `file` to config["url"] + "/" + key here.
        Ok(file.metadata()?.len())
    }

    async fn get(&self, _ctx: &Context, _req: StorageGetRequest) -> Result<u64, Error> {
        // Download the object into a new file at req.target_path here.
        Err(Error::internal("not implemented"))
    }

    async fn list(&self, _ctx: &Context, _req: StorageListRequest) -> Result<Vec<StorageObject>, Error> {
        // Return StorageObject::stored(key, size, modified) for every key with req.prefix.
        Ok(Vec::new())
    }

    async fn delete(&self, _ctx: &Context, _req: StorageDeleteRequest) -> Result<(), Error> {
        Ok(())
    }
}
```

### Certificate deployment

The host pushes a certificate to every target a person bound to it after each
issuance or renewal, and on demand. `req.certificate` carries the leaf, its
chain and its private key as PEM (`full_chain_pem` joins leaf and chain);
`req.dry_run` asks you to check the target without changing anything. Because
the request carries the private key, the manifest must request the
`cert.deploy` permission, and the key must never reach a log line or an error.
A push must be idempotent: the host retries failures.

```rust,no_run
use nginxui_plugin_sdk::{Context, DeployHandler, DeployRequest, Error};

struct Cdn;

#[async_trait::async_trait]
impl DeployHandler for Cdn {
    async fn push(&self, _ctx: &Context, req: DeployRequest) -> Result<String, Error> {
        let zone = req.config.get("zone_id").cloned().unwrap_or_default();
        if zone.is_empty() {
            return Err(Error::invalid_config("zone_id", "zone_id is required"));
        }
        if req.dry_run {
            // Read-only checks against the CDN here.
            return Ok(format!("zone {zone} is reachable"));
        }
        // Upload full_chain_pem(&req.certificate) and req.certificate.private_key_pem here.
        Ok(format!("certificate bound to zone {zone}"))
    }
}
```

### Blocklists

The host fetches every source a person configured from the manifest's
`blocklist` block on its refresh interval and writes the entries as nginx `deny`
rules to a file the person includes where the list should apply. Return the
complete list every time, as addresses or CIDR networks (`BlocklistEntry::deny`
builds an entry); the host validates each one and drops the rest. An empty list
denies nothing, so when the source cannot be read return an error and the host
keeps the list it has. `ttl_seconds` asks for an earlier refresh. The manifest
must request the `network` permission.

```rust,no_run
use nginxui_plugin_sdk::protocol::BlocklistEntry;
use nginxui_plugin_sdk::{BlocklistHandler, BlocklistRequest, BlocklistResult, Context, Error};

struct Feed;

#[async_trait::async_trait]
impl BlocklistHandler for Feed {
    async fn fetch(&self, _ctx: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        let Some(_key) = req.config.get("api_key").filter(|k| !k.is_empty()) else {
            return Err(Error::invalid_config("api_key", "api_key is required"));
        };
        // Download the list with the key here.
        Ok(BlocklistResult {
            entries: vec![BlocklistEntry::deny("203.0.113.0/24", "botnet")],
            ttl_seconds: 900,
        })
    }
}
```

### Service discovery

The host resolves every upstream a person bound to a service of one of the
manifest's `discovery` providers on its refresh interval and writes the servers
as an nginx `upstream` block. Return every server of `req.service`
(`DiscoveryTarget::new` builds one with weight 1): an IP address or a host name,
a port and a weight. Return `unknown_service` for a service the provider does
not know and any other error when the provider cannot be reached; the host then
keeps the servers it has. The manifest must request the `network` permission.

```rust,no_run
use nginxui_plugin_sdk::protocol::DiscoveryTarget;
use nginxui_plugin_sdk::{Context, DiscoveryHandler, DiscoveryRequest, DiscoveryResult, Error};

struct Registry;

#[async_trait::async_trait]
impl DiscoveryHandler for Registry {
    async fn resolve(&self, _ctx: &Context, req: DiscoveryRequest) -> Result<DiscoveryResult, Error> {
        if req.config.get("address").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("address", "address is required"));
        }
        // Look req.service up in the registry here.
        Ok(DiscoveryResult {
            targets: vec![DiscoveryTarget::new("10.0.1.12", 8080)],
            ttl_seconds: 0,
        })
    }
}
```

### Access log sinks

The host streams the nginx access log lines it reads to a `log.sink` plugin
while nginx writes them, in batches of at most `log_sink.batch_size` entries
(256 by default). `push` gets one batch, every entry with its `log_path` and
the parsed fields (`remote_addr`, `request_uri`, `status`, `body_bytes_sent`,
`request_time`, ...), and returns how many entries it kept; the rest counts as
rejected. `entry.parsed()` is false for a line the host could not parse, which
carries only `raw` and `timestamp`, and `entry.time()` parses the timestamp.
Answer quickly and buffer towards a slow destination: the host queues at most
8192 lines per plugin and drops the rest. An error loses the whole batch. The
manifest must request the `log.read` permission.

```rust,no_run
use nginxui_plugin_sdk::{Context, Error, LogSinkEntry, LogSinkHandler};
use tokio::sync::mpsc;

struct Shipper {
    out: mpsc::Sender<LogSinkEntry>,
}

#[async_trait::async_trait]
impl LogSinkHandler for Shipper {
    async fn push(&self, _ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error> {
        let mut accepted = 0;
        for entry in batch {
            // A full buffer counts the entry as rejected.
            if self.out.try_send(entry).is_ok() {
                accepted += 1;
            }
        }
        Ok(accepted)
    }
}
```

The lines travel as a client stream on the gRPC transport only:
`log.push` has no stdio form and answers `-32601` there. Setting `Plugin::log_sink`
therefore keeps gRPC on even when `Options::without_grpc` or
`NGINX_UI_PLUGIN_DISABLE_GRPC=1` asked for stdio only.

### Content plugins

Config templates and translation files need no process and no SDK: declare them
in the manifest's `content` block and ship the files in the package. See
[Templates and Translations](https://nginxui.com/plugin/capabilities/content).

## Transports

stdio is always served. On top of it the SDK serves the same handlers over gRPC
by default and advertises it in the `plugin.initialize` reply
(`transports: ["stdio", "grpc"]`), so the host can send capability calls
(`dns01.*`, `http.handle`, `notify.*`, `probe.check`, `mcp.call`, `storage.*`,
`deploy.*`, `blocklist.fetch`, `discovery.resolve`) there, and streams
(`log.push`), which exist on gRPC only. Lifecycle methods, `host.*` calls, host
log lines and notifications stay on stdio. Nothing changes for your handlers: a
gRPC call is decoded into the same JSON params and runs the same handler, and a
returned `Error` reaches the host with the same code, message and data on either
transport. The deadline of a gRPC call is applied to the handler.

* On Linux and macOS the server listens on the Unix socket
  `$NGINX_UI_PLUGIN_DATA_DIR/rpc.sock`. When that path is longer than the
  platform allows (103 bytes on macOS and the BSDs, 107 on Linux) or the data
  directory is unusable, the SDK uses a private directory under the system temp
  dir instead. The path is always reported in `rpc_socket`, and the socket is
  removed when the plugin exits.
* On Windows it listens on a named pipe under a random name, reported in
  `rpc_pipe` together with a random `rpc_token`. Calls without the header
  `authorization: Bearer <rpc_token>` are rejected.

To stay on stdio only, pass `Options::new().without_grpc()` to `serve_with`,
`run_with` or `serve_blocking_with`, or set `NGINX_UI_PLUGIN_DISABLE_GRPC=1` in
the plugin environment (a plugin with a `log_sink` keeps gRPC regardless):

```rust,no_run
use nginxui_plugin_sdk::{Options, Plugin};

fn main() {
    nginxui_plugin_sdk::serve_blocking_with(Plugin::new(), Options::new().without_grpc());
}
```

When the listener cannot be opened the SDK logs a warning and advertises stdio
only; the host never depends on gRPC being present. The gRPC transport is a
default feature of the crate that cannot be turned off, since a `log.sink`
plugin needs it and the handlers are wired the same way for both transports.

## Modules

| Module | Contents |
| --- | --- |
| crate root | `Plugin`, `serve`, `run`, the capability handler traits and helpers, `Context`, the `Host` client, `Error` and `redact` |
| `protocol` | The wire types, method names, capability, permission and error-code constants. Mirrors `internal/plugin/protocol` of nginx-ui |
| `jsonrpc` | The bidirectional NDJSON JSON-RPC 2.0 peer, usable on its own |
| `pb` | Generated protobuf bindings of the contract and the table of its rpcs |
| `http` | Types and helpers of the `http` capability |
| `logger` | The logger, `Level` and the `debug!`, `info!`, `warn!` and `error!` macros |

## The proto contract and `pb`

The wire contract is defined in proto, in
[plugin-spec](https://github.com/nginxui/plugin-spec) under
`proto/nginxui/plugin/v1`. A JSON-RPC `method` is the rpc's `rpc_name` option and
`params` / `result` are the protobuf JSON mapping of its messages with proto
field names, so the JSON the SDK exchanges is exactly what the proto describes.

`pb` holds the code generated from those files: the messages (`pb::v1`), the
JSON mapping of each of them, and `pb::registry::RPCS`, the table of every rpc
with its `rpc_name`, gRPC path, kind and the codecs between protobuf bytes and
the JSON mapping. The plugin runtime keeps using the hand written types of
`protocol`; `pb` is there for the gRPC transport, for reflection and for code
that prefers generated types. The gRPC transport resolves every call through
the table, so a new rpc in the contract is served as soon as `pb` is
regenerated and a handler exists; a client streaming rpc is detected from the
table, read until the end of the stream and handed to its stream handler.

The generated files are checked in, so building the crate needs no `protoc`.
The alignment tests (`src/tests/alignment.rs`) fail when a `protocol` type
drifts from its proto message (a field name or a JSON shape), when a message
has no counterpart, when a method constant has no rpc or the other way round,
when an error code differs from the `ErrorCode` enum, and, when the spec
checkout is there, when the rpc table differs from `gen/methods.json`.

To pick up a contract change, run `make generate` in the spec repository, then

```bash
cargo xtask generate            # regenerates src/pb from ../plugin-spec (or SPEC_DIR)
cargo xtask generate --check    # only reports files that differ
cargo test
```

here, and update `protocol` until the tests pass. The generator is the
`xtask` crate of this workspace; it uses the `protoc` of the
`protoc-bin-vendored` crate and expects the spec checkout next to this
repository. `serde_json` is the only JSON codec: the messages get their
`Serialize` and `Deserialize` from `pbjson-build` with proto field names, and
`google.protobuf.Struct`, `Value` and `ListValue` come from `pb::wkt`.

## Lifecycle

| Method | Direction | Meaning |
| --- | --- | --- |
| `plugin.initialize` | host → plugin | Handshake. The reply carries `api_version` and the implemented capabilities |
| `plugin.initialized` | host → plugin | Notification. `host.*` calls are allowed from here on |
| `plugin.configure` | host → plugin | New settings map |
| `plugin.ping` | host → plugin | Liveness probe, replies `{}` |
| `plugin.shutdown` | host → plugin | Finish in-flight work, then reply `{}` |
| `plugin.exit` | host → plugin | Notification. Exit now |

Requests are matched by id and may be concurrent. Notifications carry no id and
are never answered. A message larger than 4 MiB is rejected; a reply that is
larger becomes an internal error instead of silence. A message that is not
JSON-RPC 2.0 (its `jsonrpc` member is not `"2.0"`) is answered with `-32600`
(invalid request). `plugin.shutdown` waits for the capability calls in flight, on stdio
and on gRPC, at most `SHUTDOWN_DRAIN` (30 seconds).

## Error codes

| Code | Helper | Meaning |
| --- | --- | --- |
| `-32601` | `Error::method_not_found` | Unknown method |
| `-32602` | `Error::invalid_params`, `unknown_tool` | Malformed params, an MCP tool the plugin does not serve, or a malformed storage key |
| `-32000` | `Error::internal` | Internal failure |
| `-32002` | `Error::unsupported` | Capability method the plugin does not implement |
| `-32003` | `Error::invalid_config`, `unknown_service` | Bad credential or setting, `data.field` names it |

Any other error a handler returns becomes `-32000`. `Error::new` builds an
error with any code and payload, and `protocol::code` holds the constants.

## The host client

Once `plugin.initialized` arrived, `ctx.host()` (or `current_host()`) returns a
client for the `host.*` side of the protocol: `log`, `kv_get` / `kv_set` /
`kv_delete` / `kv_list`, `settings_get`, `locale`, `credentials_get`,
`cron_register` / `cron_unregister`, `notify`, `metrics_snapshot`, `logs_list`,
`activity_set` (`activity` wraps it in a guard that clears the entry),
`nginx_snippet_put` / `nginx_snippet_delete` / `nginx_snippet_list`,
`nginx_config_list` / `nginx_config_get`, `sites_list` and `certs_list`. Each
call needs the matching manifest permission; without it the host answers
`-32001`, which `HostError::rpc_error` returns. A call before the handshake
finished fails with `HostError::NotReady`. `settings()` returns the latest
settings map and `info()` the plugin id, data directory and host information.

### Log files and events

`logs_list` returns the nginx log files the host allows the plugin to read, each
with `path`, `type` (`protocol::log_type::ACCESS` or `ERROR`), `source` and
`config_file`. It needs the `log.files` permission
(`protocol::permission::LOG_FILES`). Rotated files are not listed: read them next
to a listed path. Subscribe to `log.paths_changed` in the manifest `events` and
list again when it arrives:

```rust,no_run
use nginxui_plugin_sdk::protocol::event;
use nginxui_plugin_sdk::Plugin;

fn main() {
    let plugin = Plugin::new().event(event::LOG_PATHS_CHANGED, |ctx, _ev| async move {
        if let Ok(logs) = ctx.host().logs_list().await {
            nginxui_plugin_sdk::info!("{} log files", logs.len());
        }
    });
    nginxui_plugin_sdk::serve_blocking(plugin);
}
```

`Plugin::event` takes over `events.on` and ignores the types it has no handler
for. `activity_set(key, label, active)` shows a background task in the host
processing indicator. `label` is an English source string; the browser bundle
translates it with `registerTranslations`.

### nginx configuration

With the `nginx.snippet` permission a plugin keeps nginx configuration of its
own. `nginx_snippet_put(name, content)` writes the snippet, and the host tests
the whole configuration and reloads nginx. When nginx rejects it, the previous
snippet stays and the call fails with `-32602`, carrying what nginx said. The
result holds the `include` directive a person adds where the snippet should
apply. A snippet that is still included cannot be deleted.

```rust,no_run
use nginxui_plugin_sdk::{Context, Plugin};
use serde_json::json;

fn main() {
    let plugin = Plugin::new().method("cache.apply", |ctx: Context, _params| async move {
        let put = ctx.host().nginx_snippet_put("static", "expires 7d;\n").await?;
        Ok(json!({"include": put.include}))
    });
    nginxui_plugin_sdk::serve_blocking(plugin);
}
```

`nginx_config_list` and `nginx_config_get` (`nginx.config.read`) read the
configuration files, `sites_list` (`sites.read`) lists the sites and
`certs_list` (`certs.read`) the certificates, never with their private keys.

A cron entry, from the manifest or from `cron_register`, names a method of the
plugin. When it fires, the host calls that method as an ordinary request with
params `{"type": "<cron id>", "ts": <unix seconds>}` and waits for the reply, so
register the handler with `Plugin::method`. Cron invocations do not go through
`events.on`.

## Logging

`info!`, `debug!`, `warn!` and `error!` write to the process wide logger, whose
lines go to stderr and, once the host is ready, to `host.log`. `Level` is the
severity, `logger::logger()` the `Logger` (its levels and output can be
changed) and `logger::log_fields` writes a line with structured fields. Use
`redact` before putting a credential derived string in a line.

## Environment

| Variable | Meaning |
| --- | --- |
| `NGINX_UI_PLUGIN_ID` | The plugin id from the manifest |
| `NGINX_UI_PLUGIN_API_VERSION` | The protocol version the host speaks |
| `NGINX_UI_PLUGIN_DATA_DIR` | The only directory the plugin may write to |
| `NGINX_UI_VERSION` | The host version |
| `NGINX_UI_PLUGIN_HTTP_SECRET` | Per process secret for the `http` capability, read and removed by the SDK, see [HTTP API](#http-api) |
| `HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY` (and lowercase) | Set only for a plugin that holds the `network` permission, and only when the host has a proxy configured. Most HTTP clients pick them up |
| `NGINX_UI_PLUGIN_DISABLE_GRPC` | Set to `1` to serve stdio only, like `Options::without_grpc()` |

`Options::with_env` reads these from a map instead, which makes a run
independent of the process environment in tests.

## Testing a plugin

`run(plugin, reader, writer)` serves a plugin over any pair of `AsyncRead` and
`AsyncWrite`, so a test speaks the protocol over `tokio::io::duplex`. The
`jsonrpc::Conn` peer is the host side: register `host.*` handlers on it, call
`plugin.initialize`, then the capability methods. The tests of this crate
(`src/tests`) do exactly that, and `src/tests/vectors.rs` replays every vector
of the specification.

`examples/reference-plugin` implements every capability. It is the plugin that
`nginx-ui plugin conformance` is run against, over stdio and gRPC:

```bash
examples/reference-plugin/package.sh
nginx-ui plugin conformance target/plugin-package
nginx-ui plugin conformance target/plugin-package --transport grpc
```

## License

AGPL-3.0. See the `LICENSE` file.
