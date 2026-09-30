//! A plugin that implements every capability of the plugin contract.
//!
//! It keeps everything in memory or in its data directory and talks to no
//! vendor, so it can run anywhere. It exists to show how each capability is
//! wired, and to be the plugin `nginx-ui plugin conformance` is run against.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use async_trait::async_trait;
use nginxui_plugin_sdk::http::{
    full_body, header, text_response, user_from_request, HeaderValue, HttpBody, Incoming, Request,
    Response, StatusCode,
};
use nginxui_plugin_sdk::protocol::{
    event, notify_severity, BlocklistEntry, DiscoveryTarget, Dns01CheckParams, Dns01CheckResult,
    Dns01OptionsParams, Dns01OptionsResult, Settings, StorageObject,
};
use nginxui_plugin_sdk::{
    full_chain_pem, redact, unknown_service, BlocklistHandler, BlocklistRequest, BlocklistResult,
    Context, DeployHandler, DeployRequest, DiscoveryHandler, DiscoveryRequest, DiscoveryResult,
    Dns01Handler, Dns01Request, Error, HttpHandler, LogSinkEntry, LogSinkHandler, McpResult,
    McpTools, NotifyHandler, NotifyRequest, Plugin, ProbeHandler, ProbeRequest, ProbeResult,
    StorageDeleteRequest, StorageGetRequest, StorageHandler, StorageListRequest, StoragePutRequest,
};

/// Returns the value of a required form field or an invalid config error
/// that names it.
fn required<'a>(config: &'a HashMap<String, String>, key: &str) -> Result<&'a str, Error> {
    match config.get(key).map(String::as_str) {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Err(Error::invalid_config(key, format!("{key} is required"))),
    }
}

// dns01

/// Publishes challenge records into memory.
#[derive(Default)]
struct Dns01 {
    records: Mutex<BTreeMap<String, String>>,
}

#[async_trait]
impl Dns01Handler for Dns01 {
    async fn present(&self, ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        if req.dry_run {
            // Check the shape of the call, touch nothing.
            return Ok(());
        }
        let token = required(&req.config, "MYDNS_API_TOKEN")?;
        nginxui_plugin_sdk::info!(
            "publishing {} for {} (token {})",
            req.effective_fqdn,
            req.domain,
            redact(token)
        );
        self.records
            .lock()
            .unwrap()
            .insert(req.effective_fqdn.clone(), req.value.clone());
        // The private store of the plugin is optional, the manifest asks for it.
        if let Err(e) = ctx
            .host()
            .kv_set(&format!("records/{}", req.effective_fqdn), &req.value)
            .await
        {
            nginxui_plugin_sdk::debug!("kv is not available: {e}");
        }
        Ok(())
    }

    async fn clean_up(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        nginxui_plugin_sdk::info!("removing {}", req.effective_fqdn);
        self.records.lock().unwrap().remove(&req.effective_fqdn);
        Ok(())
    }

    async fn validate(
        &self,
        _ctx: &Context,
        _provider: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        required(config, "MYDNS_API_TOKEN").map(|_| ())
    }

    async fn options(
        &self,
        _ctx: &Context,
        _params: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        Ok(Dns01OptionsResult {
            propagation_timeout_seconds: 120,
            polling_interval_seconds: 2,
            sequential_interval_seconds: 0,
        })
    }

    async fn check(
        &self,
        _ctx: &Context,
        params: Dns01CheckParams,
    ) -> Result<Dns01CheckResult, Error> {
        let seen = self.records.lock().unwrap().get(&params.fqdn).cloned();
        let ready = seen.as_deref() == Some(params.value.as_str());
        Ok(Dns01CheckResult {
            ready,
            effective_fqdn: params.fqdn,
            detail: if ready {
                String::new()
            } else {
                "record not published yet".into()
            },
        })
    }
}

// notify

struct Notify;

#[async_trait]
impl NotifyHandler for Notify {
    async fn send(&self, _ctx: &Context, req: NotifyRequest) -> Result<(), Error> {
        let hook = required(&req.config, "webhook_url")?;
        let severity = if req.severity.is_empty() {
            notify_severity::INFO
        } else {
            &req.severity
        };
        // Post req.title and req.content to the vendor here.
        nginxui_plugin_sdk::info!(
            "would post {severity} notification {:?} to {}",
            req.title,
            redact(hook)
        );
        Ok(())
    }

    async fn validate(
        &self,
        _ctx: &Context,
        _channel: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        let hook = required(config, "webhook_url")?;
        if !hook.starts_with("https://") {
            return Err(Error::invalid_config(
                "webhook_url",
                "webhook_url must be an https URL",
            ));
        }
        Ok(())
    }
}

// probe

struct Probe;

#[async_trait]
impl ProbeHandler for Probe {
    async fn check(&self, ctx: &Context, req: ProbeRequest) -> Result<ProbeResult, Error> {
        let started = Instant::now();
        let target = req
            .target
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let host = target.split('/').next().unwrap_or_default();
        if host.is_empty() {
            return Err(Error::invalid_config("target", "target is not a URL"));
        }
        let port = req.config.get("port").map_or(Ok(80), |p| {
            p.parse::<u16>()
                .map_err(|_| Error::invalid_config("port", "port must be a number"))
        })?;
        let address = if host.contains(':') {
            host.to_owned()
        } else {
            format!("{host}:{port}")
        };

        // An unhealthy target is a result, not an error.
        let budget = ctx.remaining().unwrap_or(Duration::from_secs(5));
        match tokio::time::timeout(budget, tokio::net::TcpStream::connect(&address)).await {
            Ok(Ok(_)) => Ok(ProbeResult::up(started.elapsed())),
            Ok(Err(e)) => Ok(ProbeResult::down(started.elapsed(), e.to_string())),
            Err(_) => Ok(ProbeResult::down(started.elapsed(), "connect timed out")),
        }
    }
}

// mcp

fn mcp_tools() -> McpTools {
    McpTools::new()
        .tool("purge_cache", |_ctx, args| async move {
            match args.get("zone").and_then(|z| z.as_str()) {
                Some(zone) if !zone.is_empty() => Ok(McpResult::text(format!("purged {zone}"))),
                _ => Ok(McpResult::error("zone is required")),
            }
        })
        .tool("list_zones", |_ctx, _args| async {
            Ok(McpResult::text("example.com, example.org"))
        })
}

// storage

/// Keeps objects in memory. A real backend moves the file at
/// `req.source_path` and `req.target_path`, which sit in the exchange
/// directory of the data directory.
#[derive(Default)]
struct Storage {
    objects: Mutex<BTreeMap<String, Vec<u8>>>,
}

#[async_trait]
impl StorageHandler for Storage {
    async fn put(&self, _ctx: &Context, req: StoragePutRequest) -> Result<u64, Error> {
        required(&req.config, "url")?;
        let data = tokio::fs::read(&req.source_path).await?;
        let size = data.len() as u64;
        self.objects.lock().unwrap().insert(req.key, data);
        Ok(size)
    }

    async fn get(&self, _ctx: &Context, req: StorageGetRequest) -> Result<u64, Error> {
        required(&req.config, "url")?;
        let data = self.objects.lock().unwrap().get(&req.key).cloned();
        let Some(data) = data else {
            return Err(Error::internal(format!(
                "no object is stored under {}",
                req.key
            )));
        };
        tokio::fs::write(&req.target_path, &data).await?;
        Ok(data.len() as u64)
    }

    async fn list(
        &self,
        _ctx: &Context,
        req: StorageListRequest,
    ) -> Result<Vec<StorageObject>, Error> {
        required(&req.config, "url")?;
        Ok(self
            .objects
            .lock()
            .unwrap()
            .iter()
            .filter(|(key, _)| key.starts_with(&req.prefix))
            .map(|(key, data)| {
                StorageObject::stored(key.clone(), data.len() as u64, Some(SystemTime::now()))
            })
            .collect())
    }

    async fn delete(&self, _ctx: &Context, req: StorageDeleteRequest) -> Result<(), Error> {
        required(&req.config, "url")?;
        // A missing object is not an error.
        self.objects.lock().unwrap().remove(&req.key);
        Ok(())
    }

    async fn validate(
        &self,
        _ctx: &Context,
        _backend: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        let url = required(config, "url")?;
        if !url.starts_with("https://") {
            return Err(Error::invalid_config("url", "url must be an https URL"));
        }
        Ok(())
    }
}

// cert.deploy

/// Writes the certificate into the data directory.
struct Deploy;

#[async_trait]
impl DeployHandler for Deploy {
    async fn push(&self, ctx: &Context, req: DeployRequest) -> Result<String, Error> {
        let name: String = req
            .certificate
            .name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let dir = ctx
            .host()
            .info()
            .data_dir
            .join("deploy")
            .join(if name.is_empty() {
                "certificate".into()
            } else {
                name
            });
        if req.dry_run {
            // Say what a push would do, change nothing.
            return Ok(format!("would write the certificate to {}", dir.display()));
        }
        tokio::fs::create_dir_all(&dir).await?;
        tokio::fs::write(dir.join("fullchain.pem"), full_chain_pem(&req.certificate)).await?;
        // The private key never reaches a log line or an error.
        let key_path = dir.join("privkey.pem");
        tokio::fs::write(&key_path, &req.certificate.private_key_pem).await?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tokio::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600)).await?;
        }
        Ok(format!("wrote the certificate to {}", dir.display()))
    }

    async fn validate(
        &self,
        _ctx: &Context,
        _kind: &str,
        _config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

// security.blocklist

struct Blocklist;

#[async_trait]
impl BlocklistHandler for Blocklist {
    async fn fetch(&self, _ctx: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        required(&req.config, "api_key")?;
        Ok(BlocklistResult {
            entries: vec![
                BlocklistEntry::deny("203.0.113.0/24", "documentation network"),
                BlocklistEntry::deny("2001:db8:bad::/48", ""),
            ],
            ttl_seconds: 900,
        })
    }
}

// upstream.discovery

struct Discovery;

#[async_trait]
impl DiscoveryHandler for Discovery {
    async fn resolve(
        &self,
        _ctx: &Context,
        req: DiscoveryRequest,
    ) -> Result<DiscoveryResult, Error> {
        match req.service.as_str() {
            "api" => Ok(DiscoveryResult {
                targets: vec![
                    DiscoveryTarget::new("10.0.1.12", 8080).with_tags(["zone-a"]),
                    DiscoveryTarget::new("10.0.2.7", 8080).with_weight(2),
                ],
                ttl_seconds: 30,
            }),
            other => Err(unknown_service(other)),
        }
    }
}

// log.sink

/// Counts the access log lines. A real sink buffers them towards its
/// destination and answers at once.
#[derive(Default)]
struct LogSink {
    lines: AtomicU64,
}

#[async_trait]
impl LogSinkHandler for LogSink {
    async fn push(&self, _ctx: &Context, batch: Vec<LogSinkEntry>) -> Result<usize, Error> {
        let total =
            self.lines.fetch_add(batch.len() as u64, Ordering::Relaxed) + batch.len() as u64;
        if let Some(first) = batch.first() {
            nginxui_plugin_sdk::debug!(
                "{} lines so far, the last batch starts with {}",
                total,
                first.request_uri
            );
        }
        Ok(batch.len())
    }
}

// http

struct Http {
    log_sink: Arc<LogSink>,
}

#[async_trait]
impl HttpHandler for Http {
    async fn handle(&self, req: Request<Incoming>) -> Response<HttpBody> {
        let user = user_from_request(&req);
        match req.uri().path() {
            "/hello" => text_response(StatusCode::OK, format!("hello {}", user.name)),
            "/status" => {
                let body =
                    serde_json::json!({"log_lines": self.log_sink.lines.load(Ordering::Relaxed)});
                let mut response = Response::new(full_body(body.to_string()));
                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                response
            }
            _ => text_response(StatusCode::NOT_FOUND, "not found"),
        }
    }
}

fn main() {
    let log_sink = Arc::new(LogSink::default());
    let settings: Arc<Mutex<Settings>> = Arc::default();
    let configured = settings.clone();

    let plugin = Plugin::new()
        .dns01(Dns01::default())
        .http(Http {
            log_sink: log_sink.clone(),
        })
        .notify(Notify)
        .probe(Probe)
        .mcp(mcp_tools())
        .storage(Storage::default())
        .deploy(Deploy)
        .blocklist(Blocklist)
        .discovery(Discovery)
        .log_sink(log_sink)
        .configure(move |_ctx, new_settings| {
            let configured = configured.clone();
            async move {
                nginxui_plugin_sdk::info!("configured with {} settings", new_settings.len());
                *configured.lock().unwrap() = new_settings;
                Ok(())
            }
        })
        .event(event::CERT_ISSUED, |_ctx, ev| async move {
            nginxui_plugin_sdk::info!("a certificate was issued at {}", ev.ts);
        })
        .event(event::LOG_PATHS_CHANGED, |ctx, _ev| async move {
            // List the files again, the set of logs changed.
            match ctx.host().logs_list().await {
                Ok(logs) => nginxui_plugin_sdk::info!("{} log files", logs.len()),
                Err(e) => nginxui_plugin_sdk::warn!("cannot list the log files: {e}"),
            }
        })
        // The cron entry of the manifest calls this method.
        .method("cron.tick", |_ctx, params| async move {
            nginxui_plugin_sdk::debug!("cron tick {params}");
            Ok(serde_json::json!({}))
        })
        .shutdown(|_ctx| async {
            nginxui_plugin_sdk::info!("shutting down");
            Ok(())
        });

    // Before the handshake the line goes to stderr, never to stdout.
    nginxui_plugin_sdk::info!("reference plugin starting");
    nginxui_plugin_sdk::serve_blocking(plugin);
}
