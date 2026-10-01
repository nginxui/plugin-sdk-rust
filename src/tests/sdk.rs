use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::mpsc;

use super::harness::{start, WAIT};
use crate::protocol::{
    capability, code, event, log_source, log_type, method, Dns01CheckParams, Dns01CheckResult,
    Dns01OptionsParams, Dns01OptionsResult, Dns01ValidateParams, Error, EventNotification,
    HostActivitySetParams, HostLogFile, HostLogsListResult, Settings,
};
use crate::{Context, Dns01Handler, Dns01Request, HostError, Level, Plugin};

/// A `Dns01Handler` that only remembers what it was asked.
pub struct Recorder {
    pub present: mpsc::UnboundedSender<Dns01Request>,
    pub cleanup: mpsc::UnboundedSender<Dns01Request>,
}

pub fn recorder() -> (
    Recorder,
    mpsc::UnboundedReceiver<Dns01Request>,
    mpsc::UnboundedReceiver<Dns01Request>,
) {
    let (present, present_rx) = mpsc::unbounded_channel();
    let (cleanup, cleanup_rx) = mpsc::unbounded_channel();
    (Recorder { present, cleanup }, present_rx, cleanup_rx)
}

#[async_trait]
impl Dns01Handler for Recorder {
    async fn present(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        // The logger must reach the host once the handshake completed.
        crate::info!("presenting {}", req.fqdn);
        let _ = self.present.send(req);
        Ok(())
    }

    async fn clean_up(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        let _ = self.cleanup.send(req);
        Ok(())
    }
}

async fn recv<T>(rx: &mut mpsc::UnboundedReceiver<T>, what: &str) -> T {
    tokio::time::timeout(WAIT, rx.recv())
        .await
        .unwrap_or_else(|_| panic!("{what} was not delivered"))
        .unwrap_or_else(|| panic!("{what}: channel closed"))
}

#[tokio::test]
async fn lifecycle() {
    let (rec, mut present, mut cleanup) = recorder();
    let (configured_tx, mut configured) = mpsc::unbounded_channel::<Settings>();
    let (shutdown_tx, mut shutdown) = mpsc::unbounded_channel::<()>();

    let plugin = Plugin::new()
        .dns01(rec)
        .configure(move |_ctx, settings| {
            let tx = configured_tx.clone();
            async move {
                let _ = tx.send(settings);
                Ok(())
            }
        })
        .shutdown(move |_ctx| {
            let tx = shutdown_tx.clone();
            async move {
                let _ = tx.send(());
                Ok(())
            }
        });
    let mut h = start(plugin);

    let res = h.initialize().await;
    assert_eq!(res.api_version, crate::protocol::API_VERSION);
    assert_eq!(res.capabilities, [capability::DNS01]);

    // plugin.ping replies with an empty object.
    assert_eq!(h.call(method::PING, ()).await.unwrap(), json!({}));

    // plugin.configure updates the settings the host client reports.
    h.call(
        method::CONFIGURE,
        json!({"settings": {"recursive_nameservers": "1.1.1.1:53"}}),
    )
    .await
    .unwrap();
    let got = recv(&mut configured, "configure").await;
    assert_eq!(got["recursive_nameservers"], "1.1.1.1:53");

    // dns01.present reaches the handler and its log line reaches the host.
    let req = json!({
        "provider": "exec",
        "domain": "example.com",
        "fqdn": "_acme-challenge.example.com.",
        "effective_fqdn": "_acme-challenge.example.com.",
        "value": "token-value",
        "token": "tok",
        "key_auth": "key",
    });
    h.call(method::DNS01_PRESENT, &req).await.unwrap();
    assert_eq!(
        recv(&mut present, "present").await.fqdn,
        "_acme-challenge.example.com."
    );
    let line = tokio::time::timeout(WAIT, h.logs.recv())
        .await
        .expect("host.log was not received")
        .unwrap();
    assert_eq!(line.level, Level::Info.as_str());

    h.call(method::DNS01_CLEANUP, &req).await.unwrap();
    recv(&mut cleanup, "cleanup").await;

    // plugin.shutdown runs the hook and replies, then plugin.exit ends run.
    h.call(method::SHUTDOWN, ()).await.unwrap();
    recv(&mut shutdown, "the shutdown hook").await;

    h.host.notify(method::EXIT, ()).await.unwrap();
    h.finished()
        .await
        .expect("run returns Ok after plugin.exit");
}

#[tokio::test]
async fn optional_dns01_methods_are_unsupported() {
    let (rec, _p, _c) = recorder();
    let mut h = start(Plugin::new().dns01(rec));
    h.initialize().await;

    for m in [
        method::DNS01_VALIDATE,
        method::DNS01_OPTIONS,
        method::DNS01_CHECK,
    ] {
        let err = h.call(m, json!({"provider": "exec"})).await.unwrap_err();
        assert_eq!(err.code, code::UNSUPPORTED, "{m}");
    }
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let (rec, _p, _c) = recorder();
    let mut h = start(Plugin::new().dns01(rec));
    h.initialize().await;

    let err = h.call("http.handle", ()).await.unwrap_err();
    assert_eq!(err.code, code::METHOD_NOT_FOUND);
}

/// Implements every optional dns01 method.
struct Full;

#[async_trait]
impl Dns01Handler for Full {
    async fn present(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }

    async fn clean_up(&self, _: &Context, _: Dns01Request) -> Result<(), Error> {
        Ok(())
    }

    async fn validate(
        &self,
        _: &Context,
        _provider: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        if config.get("EXEC_PATH").is_none_or(String::is_empty) {
            return Err(Error::invalid_config("EXEC_PATH", "missing program path"));
        }
        Ok(())
    }

    async fn options(
        &self,
        _: &Context,
        _: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        Ok(Dns01OptionsResult {
            propagation_timeout_seconds: 120,
            polling_interval_seconds: 2,
            ..Default::default()
        })
    }

    async fn check(&self, _: &Context, _: Dns01CheckParams) -> Result<Dns01CheckResult, Error> {
        Ok(Dns01CheckResult {
            ready: true,
            effective_fqdn: "_acme-challenge.example.com.".into(),
            ..Default::default()
        })
    }
}

#[tokio::test]
async fn optional_dns01_methods_are_served_when_implemented() {
    let mut h = start(Plugin::new().dns01(Full));
    h.initialize().await;

    let opts = h
        .call(
            method::DNS01_OPTIONS,
            Dns01OptionsParams {
                provider: "exec".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(opts["propagation_timeout_seconds"], 120);

    let check = h
        .call(
            method::DNS01_CHECK,
            Dns01CheckParams {
                provider: "exec".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(check["ready"], true);

    // An invalid config maps to -32003 with the offending field.
    let err = h
        .call(
            method::DNS01_VALIDATE,
            Dns01ValidateParams {
                provider: "exec".into(),
                config: HashMap::new(),
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INVALID_CONFIG);
    assert_eq!(err.field(), Some("EXEC_PATH"));
}

#[tokio::test]
async fn run_returns_on_input_eof() {
    let (host_w, plugin_in) = tokio::io::duplex(1 << 16);
    let (plugin_out, _host_r) = tokio::io::duplex(1 << 16);
    let run = tokio::spawn(crate::run(Plugin::new(), plugin_in, plugin_out));

    let mut host_w = host_w;
    tokio::io::AsyncWriteExt::shutdown(&mut host_w)
        .await
        .unwrap();

    let out = tokio::time::timeout(WAIT, run)
        .await
        .expect("run did not return after EOF");
    out.unwrap().expect("run returns Ok on EOF");
}

#[tokio::test]
async fn host_calls_before_initialized_are_rejected() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let plugin = Plugin::new().method("test.early", move |ctx: Context, _params| {
        let tx = tx.clone();
        async move {
            let out = ctx.host().kv_set("k", "v").await;
            let _ = tx.send(out);
            Ok(json!({}))
        }
    });
    let h = start(plugin);

    h.call("test.early", ()).await.unwrap();
    let out = recv(&mut rx, "the handler").await;
    assert!(matches!(out, Err(HostError::NotReady)), "{out:?}");
}

#[tokio::test]
async fn capabilities_override() {
    let mut h = start(Plugin::new().capabilities([capability::HTTP]));
    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::HTTP]);
}

#[tokio::test]
async fn host_logs_list_and_activity() {
    let (events_tx, mut events) = mpsc::unbounded_channel::<EventNotification>();
    let plugin = Plugin::new()
        .event(event::LOG_PATHS_CHANGED, move |_ctx, ev| {
            let tx = events_tx.clone();
            async move {
                let _ = tx.send(ev);
            }
        })
        .method("test.logs", |ctx: Context, _params| async move {
            let host = ctx.host();
            let logs = host.logs_list().await?;
            let activity = host.activity("indexing", "Nginx Log Indexing...").await?;
            activity.stop().await?;
            Ok(json!({"logs": logs.len()}))
        });
    let mut h = start(plugin);

    let activity: Arc<Mutex<Vec<HostActivitySetParams>>> = Arc::default();
    h.host.handle(
        method::HOST_LOGS_LIST,
        crate::jsonrpc::handler(|_| async {
            Ok(serde_json::to_value(HostLogsListResult {
                logs: vec![
                    HostLogFile {
                        path: "/var/log/nginx/access.log".into(),
                        r#type: log_type::ACCESS.into(),
                        source: log_source::DEFAULT.into(),
                        config_file: String::new(),
                    },
                    HostLogFile {
                        path: "/var/log/nginx/a.error.log".into(),
                        r#type: log_type::ERROR.into(),
                        source: log_source::CONFIG.into(),
                        config_file: "/etc/nginx/a.conf".into(),
                    },
                ],
            })
            .unwrap())
        }),
    );
    let seen = activity.clone();
    h.host.handle(
        method::HOST_ACTIVITY_SET,
        crate::jsonrpc::handler(move |params| {
            let seen = seen.clone();
            async move {
                seen.lock()
                    .unwrap()
                    .push(serde_json::from_value(params).unwrap());
                Ok(json!({}))
            }
        }),
    );

    h.initialize().await;

    let res = h.call("test.logs", ()).await.unwrap();
    assert_eq!(res["logs"], 2);

    let got = activity.lock().unwrap().clone();
    let want = vec![
        HostActivitySetParams {
            key: "indexing".into(),
            label: "Nginx Log Indexing...".into(),
            active: true,
        },
        HostActivitySetParams {
            key: "indexing".into(),
            label: "Nginx Log Indexing...".into(),
            active: false,
        },
    ];
    assert_eq!(got, want);

    // An event without a handler is ignored, the subscribed one is
    // delivered.
    h.host
        .notify(
            method::EVENTS_ON,
            json!({"type": event::CERT_ISSUED, "ts": 1}),
        )
        .await
        .unwrap();
    h.host
        .notify(
            method::EVENTS_ON,
            json!({"type": event::LOG_PATHS_CHANGED, "ts": 2}),
        )
        .await
        .unwrap();
    let ev = recv(&mut events, "log.paths_changed").await;
    assert_eq!(ev.r#type, event::LOG_PATHS_CHANGED);
    assert_eq!(ev.ts, 2);
}

#[tokio::test]
async fn host_client_covers_the_host_api() {
    // Every host.* call reaches the host with the params of the contract.
    let calls: Arc<Mutex<Vec<(String, Value)>>> = Arc::default();
    let plugin = Plugin::new().method("test.host", |ctx: Context, _params| async move {
        let host = ctx.host();
        assert_eq!(host.kv_get::<Value>("missing").await?, None);
        assert_eq!(host.kv_get::<Value>("k").await?, Some(json!({"at": 1})));
        host.kv_set("k", json!({"at": 2})).await?;
        host.kv_delete("k").await?;
        assert_eq!(host.kv_list("p").await?, ["p1", "p2"]);
        let settings = host.settings_get().await?;
        assert_eq!(settings["a"], 1);
        assert_eq!(host.settings()["a"], 1);
        assert_eq!(host.locale().await?, "en");
        let cred = host.credentials_get("dns", "7").await?;
        assert_eq!(cred.provider_code, "cloudflare");
        host.cron_register("job", "@every 10m", "cron.job").await?;
        host.cron_unregister("job").await?;
        host.notify("info", "Title", "Body", json!({"x": 1}))
            .await?;
        let snapshot: Value = host.metrics_snapshot().await?;
        assert_eq!(snapshot["requests"], 5);
        host.activity_set("k", "Label", true).await?;
        let info = host.info();
        assert_eq!(info.host.version, "2.7.0");
        assert_eq!(info.permissions, ["network"]);
        Ok(json!({}))
    });
    let mut h = start(plugin);

    for (m, result) in [
        (method::HOST_KV_SET, json!({})),
        (method::HOST_KV_DELETE, json!({})),
        (method::HOST_KV_LIST, json!({"keys": ["p1", "p2"]})),
        (method::HOST_SETTINGS_GET, json!({"settings": {"a": 1}})),
        (method::HOST_I18N_LOCALE, json!({"locale": "en"})),
        (
            method::HOST_CREDENTIALS_GET,
            json!({"id": "7", "name": "prod", "provider_code": "cloudflare", "config": {"TOKEN": "x"}}),
        ),
        (method::HOST_CRON_REGISTER, json!({})),
        (method::HOST_CRON_UNREGISTER, json!({})),
        (method::HOST_NOTIFY, json!({})),
        (
            method::HOST_METRICS_SNAPSHOT,
            json!({"snapshot": {"requests": 5}}),
        ),
        (method::HOST_ACTIVITY_SET, json!({})),
    ] {
        let calls = calls.clone();
        h.host.handle(
            m,
            crate::jsonrpc::handler(move |params| {
                let calls = calls.clone();
                let result = result.clone();
                async move {
                    calls.lock().unwrap().push((m.to_owned(), params));
                    Ok(result)
                }
            }),
        );
    }
    let kv_reads = Arc::new(Mutex::new(0));
    h.host.handle(
        method::HOST_KV_GET,
        crate::jsonrpc::handler(move |params| {
            let kv_reads = kv_reads.clone();
            async move {
                *kv_reads.lock().unwrap() += 1;
                if params["key"] == "missing" {
                    Ok(json!({"value": null, "found": false}))
                } else {
                    Ok(json!({"value": {"at": 1}, "found": true}))
                }
            }
        }),
    );

    h.initialize().await;
    h.call("test.host", ()).await.unwrap();

    let calls = calls.lock().unwrap().clone();
    let find = |m: &str| {
        calls
            .iter()
            .find(|(name, _)| name == m)
            .map(|(_, p)| p.clone())
    };
    assert_eq!(
        find(method::HOST_KV_SET),
        Some(json!({"key": "k", "value": {"at": 2}}))
    );
    assert_eq!(find(method::HOST_KV_DELETE), Some(json!({"key": "k"})));
    assert_eq!(find(method::HOST_KV_LIST), Some(json!({"prefix": "p"})));
    assert_eq!(
        find(method::HOST_CREDENTIALS_GET),
        Some(json!({"kind": "dns", "id": "7"}))
    );
    assert_eq!(
        find(method::HOST_CRON_REGISTER),
        Some(json!({"id": "job", "schedule": "@every 10m", "method": "cron.job"}))
    );
    assert_eq!(
        find(method::HOST_CRON_UNREGISTER),
        Some(json!({"id": "job"}))
    );
    assert_eq!(
        find(method::HOST_NOTIFY),
        Some(json!({"level": "info", "title": "Title", "content": "Body", "details": {"x": 1}}))
    );
    assert_eq!(
        find(method::HOST_ACTIVITY_SET),
        Some(json!({"key": "k", "label": "Label", "active": true}))
    );
}

#[tokio::test]
async fn a_host_permission_error_reaches_the_caller_verbatim() {
    let plugin = Plugin::new().method("test.kv", |ctx: Context, _params| async move {
        // The `?` converts the host error, an error object is forwarded as is.
        ctx.host().kv_get::<Value>("k").await?;
        Ok(json!({}))
    });
    let mut h = start(plugin);
    h.host.handle(
        method::HOST_KV_GET,
        crate::jsonrpc::handler(|_| async {
            Err(Error::new(
                code::PERMISSION_DENIED,
                "permission kv is not granted",
                None,
            ))
        }),
    );
    h.initialize().await;

    let err = h.call("test.kv", ()).await.unwrap_err();
    assert_eq!(err.code, code::PERMISSION_DENIED);
    assert_eq!(err.message, "permission kv is not granted");
}

#[tokio::test]
async fn the_host_of_the_plugin_is_current_inside_a_handler() {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let plugin = Plugin::new().method("test.current", move |ctx: Context, _params| {
        let tx = tx.clone();
        async move {
            let current = crate::current_host().expect("a host inside a handler");
            let _ = tx.send(current.ready() && ctx.host().ready());
            Ok(json!({}))
        }
    });
    let mut h = start(plugin);
    h.initialize().await;
    h.call("test.current", ()).await.unwrap();
    assert!(recv(&mut rx, "the handler").await);
}

#[tokio::test]
async fn null_members_mean_their_defaults() {
    let (tx, mut configured) = mpsc::unbounded_channel::<Settings>();
    let h = start(Plugin::new().configure(move |_ctx, settings| {
        let tx = tx.clone();
        async move {
            let _ = tx.send(settings);
            Ok(())
        }
    }));

    // null and absent are the same.
    let res = h
        .call(
            method::INITIALIZE,
            json!({"host": null, "settings": null, "permissions": null}),
        )
        .await
        .unwrap();
    assert_eq!(res["api_version"], 1);
    h.host.notify(method::INITIALIZED, ()).await.unwrap();

    h.call(method::CONFIGURE, json!({"settings": null}))
        .await
        .unwrap();
    assert!(recv(&mut configured, "configure").await.is_empty());
    h.call(method::CONFIGURE, ()).await.unwrap();
    assert!(recv(&mut configured, "configure").await.is_empty());
    let _ = Value::Null;
}

#[tokio::test]
async fn host_nginx_calls() {
    let plugin = Plugin::new().method("test.nginx", |ctx: Context, _params| async move {
        let host = ctx.host();
        let put = host.nginx_snippet_put("cache", "expires 1d;\n").await?;
        let snippets = host.nginx_snippet_list().await?;
        let removed = host.nginx_snippet_delete("cache").await?;
        let files = host.nginx_config_list().await?;
        let content = host.nginx_config_get(&files[0]).await?;
        let sites = host.sites_list().await?;
        let certs = host.certs_list().await?;
        Ok(json!({
            "changed": put.changed,
            "include": put.include,
            "snippets": snippets.len(),
            "removed": removed,
            "content": content,
            "site": sites[0].name,
            "cert": certs[0].not_after,
        }))
    });
    let mut h = start(plugin);

    let answers = [
        (
            method::HOST_NGINX_SNIPPET_PUT,
            json!({"changed": true, "include": "include snippets/plugins/x/cache.conf;"}),
        ),
        (
            method::HOST_NGINX_SNIPPET_LIST,
            json!({"snippets": [{"name": "cache", "include": ""}]}),
        ),
        (method::HOST_NGINX_SNIPPET_DELETE, json!({"removed": true})),
        (
            method::HOST_NGINX_CONFIG_LIST,
            json!({"files": ["nginx.conf"]}),
        ),
        (
            method::HOST_NGINX_CONFIG_GET,
            json!({"content": "events {}"}),
        ),
        (
            method::HOST_SITES_LIST,
            json!({"sites": [{"name": "a.test"}]}),
        ),
        (
            method::HOST_CERTS_LIST,
            json!({"certs": [{"not_after": "2026-12-01T00:00:00Z"}]}),
        ),
    ];
    let params: Arc<Mutex<Vec<Value>>> = Arc::default();
    for (name, answer) in answers {
        let seen = params.clone();
        h.host.handle(
            name,
            crate::jsonrpc::handler(move |p| {
                let seen = seen.clone();
                let answer = answer.clone();
                async move {
                    seen.lock().unwrap().push(p);
                    Ok(answer)
                }
            }),
        );
    }

    h.initialize().await;

    let res = h.call("test.nginx", ()).await.unwrap();
    assert_eq!(
        res,
        json!({
            "changed": true,
            "include": "include snippets/plugins/x/cache.conf;",
            "snippets": 1,
            "removed": true,
            "content": "events {}",
            "site": "a.test",
            "cert": "2026-12-01T00:00:00Z",
        })
    );
    let seen = params.lock().unwrap().clone();
    assert_eq!(
        seen[0],
        json!({"name": "cache", "content": "expires 1d;\n"})
    );
    assert_eq!(seen[4], json!({"path": "nginx.conf"}));
}
