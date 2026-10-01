use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::sync::mpsc;

use super::harness::{start, WAIT};
use crate::protocol::{
    capability, code, method, notify_severity, probe_status, Error, NotifySendParams,
    ProbeCheckParams, MCP_CONTENT_TYPE_TEXT,
};
use crate::{
    Context, McpResult, McpTools, NotifyHandler, NotifyRequest, Plugin, ProbeHandler, ProbeRequest,
    ProbeResult,
};

/// A `NotifyHandler` that remembers what it was asked to send.
struct Notifier {
    sent: mpsc::UnboundedSender<NotifyRequest>,
}

#[async_trait]
impl NotifyHandler for Notifier {
    async fn send(&self, _ctx: &Context, req: NotifyRequest) -> Result<(), Error> {
        if req.config.get("webhook_url").is_none_or(String::is_empty) {
            return Err(Error::invalid_config(
                "webhook_url",
                "webhook_url is required",
            ));
        }
        let _ = self.sent.send(req);
        Ok(())
    }
}

/// Also implements the validator.
struct ValidatingNotifier(Notifier);

#[async_trait]
impl NotifyHandler for ValidatingNotifier {
    async fn send(&self, ctx: &Context, req: NotifyRequest) -> Result<(), Error> {
        self.0.send(ctx, req).await
    }

    async fn validate(
        &self,
        _ctx: &Context,
        channel: &str,
        config: &HashMap<String, String>,
    ) -> Result<(), Error> {
        if channel != "mychat" {
            return Err(Error::invalid_config("channel", "unknown channel"));
        }
        if config.get("webhook_url").is_none_or(String::is_empty) {
            return Err(Error::invalid_config(
                "webhook_url",
                "webhook_url is required",
            ));
        }
        Ok(())
    }
}

pub fn notifier() -> (impl NotifyHandler, mpsc::UnboundedReceiver<NotifyRequest>) {
    let (sent, rx) = mpsc::unbounded_channel();
    (Notifier { sent }, rx)
}

/// A `ProbeHandler` that reports what the context deadline was.
struct Prober {
    deadline: mpsc::UnboundedSender<Option<Duration>>,
}

#[async_trait]
impl ProbeHandler for Prober {
    async fn check(&self, ctx: &Context, req: ProbeRequest) -> Result<ProbeResult, Error> {
        let _ = self.deadline.send(ctx.remaining());
        if req.config.get("port").map(String::as_str) == Some("ssh") {
            return Err(Error::invalid_config("port", "port must be a number"));
        }
        if req.target == "http://down.invalid" {
            return Ok(ProbeResult::down(
                Duration::from_millis(1500),
                "connection refused",
            ));
        }
        if req.target == "http://slow.invalid" {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
        Ok(ProbeResult::up(Duration::from_millis(42)))
    }
}

pub fn prober() -> (impl ProbeHandler, mpsc::UnboundedReceiver<Option<Duration>>) {
    let (deadline, rx) = mpsc::unbounded_channel();
    (Prober { deadline }, rx)
}

#[tokio::test]
async fn notify_send_reaches_the_handler() {
    let (n, mut sent) = notifier();
    let mut h = start(Plugin::new().notify(n));

    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::NOTIFY]);

    let params = NotifySendParams {
        channel: "mychat".into(),
        config: HashMap::from([(
            "webhook_url".to_owned(),
            "https://chat.example/hook".to_owned(),
        )]),
        title: "Certificate Expiring Soon".into(),
        content: "example.com expires in 7 days".into(),
        severity: notify_severity::WARNING.into(),
    };
    let result = h.call(method::NOTIFY_SEND, &params).await.unwrap();
    assert_eq!(result, json!({}));
    let got = sent.recv().await.unwrap();
    assert_eq!(got.channel, "mychat");
    assert_eq!(got.title, params.title);
    assert_eq!(got.severity, notify_severity::WARNING);

    let mut bad = params;
    bad.config = HashMap::new();
    let err = h.call(method::NOTIFY_SEND, &bad).await.unwrap_err();
    assert_eq!(err.code, code::INVALID_CONFIG);
    assert_eq!(err.field(), Some("webhook_url"));
}

#[tokio::test]
async fn notify_validate_is_unsupported_without_a_validator() {
    let (n, _sent) = notifier();
    let mut h = start(Plugin::new().notify(n));
    h.initialize().await;

    let err = h
        .call(method::NOTIFY_VALIDATE, json!({"channel": "mychat"}))
        .await
        .unwrap_err();
    assert_eq!(err.code, code::UNSUPPORTED);
}

#[tokio::test]
async fn notify_validate_uses_the_validator() {
    let (sent, _rx) = mpsc::unbounded_channel();
    let mut h = start(Plugin::new().notify(ValidatingNotifier(Notifier { sent })));
    h.initialize().await;

    let err = h
        .call(
            method::NOTIFY_VALIDATE,
            json!({"channel": "mychat", "config": {}}),
        )
        .await
        .unwrap_err();
    assert_eq!(err.field(), Some("webhook_url"));

    h.call(
        method::NOTIFY_VALIDATE,
        json!({"channel": "mychat", "config": {"webhook_url": "https://chat.example/hook"}}),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn probe_check_applies_the_timeout() {
    let (p, mut deadline) = prober();
    let mut h = start(Plugin::new().probe(p));

    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::PROBE]);

    let result = h
        .call(
            method::PROBE_CHECK,
            ProbeCheckParams {
                kind: "tcp-banner".into(),
                target: "https://example.com".into(),
                timeout_seconds: 5,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(result["status"], probe_status::UP);
    assert_eq!(result["latency_ms"], 42);
    let remaining = deadline.recv().await.unwrap().expect("a deadline");
    assert!(
        remaining > Duration::ZERO && remaining <= Duration::from_secs(5),
        "{remaining:?}"
    );

    let result = h
        .call(
            method::PROBE_CHECK,
            ProbeCheckParams {
                kind: "tcp-banner".into(),
                target: "http://down.invalid".into(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(
        deadline.recv().await.unwrap().is_none(),
        "no timeout was asked for"
    );
    assert_eq!(result["status"], probe_status::DOWN);
    assert_eq!(result["message"], "connection refused");
    assert_eq!(result["latency_ms"], 1500);

    let err = h
        .call(
            method::PROBE_CHECK,
            ProbeCheckParams {
                kind: "tcp-banner".into(),
                target: "https://example.com".into(),
                config: HashMap::from([("port".to_owned(), "ssh".to_owned())]),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    deadline.recv().await.unwrap();
    assert_eq!(err.field(), Some("port"));
}

#[tokio::test(start_paused = true)]
async fn probe_check_gives_up_at_the_deadline() {
    let (p, _deadline) = prober();
    let mut h = start(Plugin::new().probe(p));
    h.initialize().await;

    let err = h
        .call(
            method::PROBE_CHECK,
            ProbeCheckParams {
                kind: "tcp-banner".into(),
                target: "http://slow.invalid".into(),
                timeout_seconds: 2,
                ..Default::default()
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INTERNAL_ERROR);
}

#[tokio::test]
async fn mcp_tools_dispatch_by_name() {
    let got_args: Arc<Mutex<Option<crate::protocol::Settings>>> = Arc::default();
    let record = got_args.clone();
    let tools = McpTools::new()
        .tool("purge_cache", move |_ctx, args| {
            let record = record.clone();
            async move {
                *record.lock().unwrap() = Some(args.clone());
                match args.get("zone").and_then(Value::as_str) {
                    Some(zone) if !zone.is_empty() => Ok(McpResult::text(format!("purged {zone}"))),
                    _ => Ok(McpResult::error("zone is required")),
                }
            }
        })
        .tool("noop", |_ctx, _args| async { Ok(McpResult::default()) });
    let mut h = start(Plugin::new().mcp(tools));

    let res = h.initialize().await;
    assert_eq!(res.capabilities, [capability::MCP]);

    let result = h
        .call(
            method::MCP_CALL,
            json!({"tool": "purge_cache", "arguments": {"zone": "example.com"}}),
        )
        .await
        .unwrap();
    assert_eq!(
        result,
        json!({"content": [{"type": MCP_CONTENT_TYPE_TEXT, "text": "purged example.com"}]})
    );
    assert_eq!(
        got_args.lock().unwrap().as_ref().unwrap()["zone"],
        "example.com"
    );

    let result = h
        .call(method::MCP_CALL, json!({"tool": "purge_cache"}))
        .await
        .unwrap();
    assert_eq!(result["is_error"], true);
    assert_eq!(result["content"][0]["text"], "zone is required");
    assert!(
        got_args.lock().unwrap().as_ref().unwrap().is_empty(),
        "a call without arguments must hand the tool an empty map"
    );

    let raw = h
        .call(method::MCP_CALL, json!({"tool": "noop"}))
        .await
        .unwrap();
    assert_eq!(
        raw["content"],
        json!([]),
        "the content list is always there"
    );

    let err = h
        .call(method::MCP_CALL, json!({"tool": "drop_database"}))
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INVALID_PARAMS);
}

#[tokio::test]
async fn capabilities_follow_the_handlers() {
    let (n, _r) = notifier();
    let (p, _d) = prober();
    let (rec, _p, _c) = super::sdk::recorder();
    let mut h = start(
        Plugin::new()
            .dns01(rec)
            .notify(n)
            .probe(p)
            .mcp(McpTools::new())
            .storage(super::storage_deploy::memory_storage())
            .deploy(super::storage_deploy::deployer().0)
            .blocklist(super::blocklist_discovery::Feed)
            .discovery(super::blocklist_discovery::Registry),
    );

    let res = h.initialize().await;
    assert_eq!(
        res.capabilities,
        [
            capability::DNS01,
            capability::NOTIFY,
            capability::PROBE,
            capability::MCP,
            capability::STORAGE,
            capability::CERT_DEPLOY,
            capability::SECURITY_BLOCKLIST,
            capability::UPSTREAM_DISCOVERY,
        ]
    );
}

#[tokio::test]
async fn unserved_capability_methods_are_unknown() {
    let (rec, _p, _c) = super::sdk::recorder();
    let mut h = start(Plugin::new().dns01(rec));
    h.initialize().await;

    for m in [
        method::NOTIFY_SEND,
        method::PROBE_CHECK,
        method::MCP_CALL,
        method::STORAGE_PUT,
        method::STORAGE_VALIDATE,
        method::DEPLOY_PUSH,
        method::DEPLOY_VALIDATE,
        method::BLOCKLIST_FETCH,
        method::DISCOVERY_RESOLVE,
    ] {
        let err = h.call(m, json!({})).await.unwrap_err();
        assert_eq!(err.code, code::METHOD_NOT_FOUND, "{m}");
    }
}

#[tokio::test]
async fn malformed_params_are_invalid_params() {
    let (n, _r) = notifier();
    let mut h = start(Plugin::new().notify(n));
    h.initialize().await;

    // The params the malformed params conformance case sends: a string for an
    // object.
    let err = h
        .call(method::NOTIFY_SEND, "not-an-object")
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INVALID_PARAMS);
    let err = h
        .call(
            method::NOTIFY_SEND,
            json!({"channel": "x", "config": "not-an-object"}),
        )
        .await
        .unwrap_err();
    assert_eq!(err.code, code::INVALID_PARAMS);
}

#[tokio::test]
async fn shutdown_waits_for_the_capability_calls_in_flight() {
    struct Slow(mpsc::UnboundedSender<()>);

    #[async_trait]
    impl NotifyHandler for Slow {
        async fn send(&self, _ctx: &Context, _req: NotifyRequest) -> Result<(), Error> {
            let _ = self.0.send(());
            tokio::time::sleep(Duration::from_millis(300)).await;
            Ok(())
        }
    }

    let (tx, mut started) = mpsc::unbounded_channel();
    let mut h = start(Plugin::new().notify(Slow(tx)));
    h.initialize().await;

    let host = h.host.clone();
    let send = tokio::spawn(async move {
        host.call(method::NOTIFY_SEND, json!({"channel": "c"}))
            .await
    });
    tokio::time::timeout(WAIT, started.recv()).await.unwrap();

    let begin = std::time::Instant::now();
    h.call(method::SHUTDOWN, ()).await.unwrap();
    assert!(
        begin.elapsed() >= Duration::from_millis(150),
        "shutdown answered before the call finished"
    );
    send.await.unwrap().unwrap();
}
