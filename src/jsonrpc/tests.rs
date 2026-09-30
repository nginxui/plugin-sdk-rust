use std::sync::Arc;
use std::time::Duration;

use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

use super::*;
use crate::protocol::{code, Error, InvalidConfigData};

const WAIT: Duration = Duration::from_secs(5);

/// Two peers wired back to back. The handlers are registered before the read
/// loops start.
struct Pair {
    a: Conn,
    b: Conn,
}

fn pair() -> Pair {
    let (x, y) = tokio::io::duplex(1 << 20);
    let (xr, xw) = tokio::io::split(x);
    let (yr, yw) = tokio::io::split(y);
    Pair {
        a: Conn::new(xr, xw),
        b: Conn::new(yr, yw),
    }
}

impl Pair {
    fn serve(&self) {
        for conn in [&self.a, &self.b] {
            let conn = conn.clone();
            tokio::spawn(async move {
                let _ = conn.serve().await;
            });
        }
    }
}

fn rpc_error(err: CallError) -> Error {
    match err {
        CallError::Rpc(e) => e,
        other => panic!("want an rpc error, got {other:?}"),
    }
}

#[tokio::test]
async fn call_returns_result() {
    let p = pair();
    p.b.handle(
        "sum",
        handler(|params| async move {
            let a = params["A"].as_i64().unwrap();
            let b = params["B"].as_i64().unwrap();
            Ok(json!({"total": a + b}))
        }),
    );
    p.serve();

    let out = p.a.call("sum", json!({"A": 2, "B": 40})).await.unwrap();
    assert_eq!(out["total"], 42);
}

#[tokio::test]
async fn call_without_params_and_result() {
    let p = pair();
    p.b.handle("ping", handler(|_| async { Ok(json!({})) }));
    p.serve();
    p.a.call("ping", ()).await.unwrap();
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let p = pair();
    p.serve();
    let err = rpc_error(p.a.call("nope", ()).await.unwrap_err());
    assert_eq!(err.code, code::METHOD_NOT_FOUND);
}

#[tokio::test]
async fn handler_protocol_error_is_forwarded_verbatim() {
    let p = pair();
    p.b.handle(
        "check",
        handler(|_| async {
            Err(Error::new(
                code::INVALID_CONFIG,
                "missing token",
                serde_json::to_value(InvalidConfigData {
                    field: "CF_DNS_API_TOKEN".into(),
                })
                .ok(),
            ))
        }),
    );
    p.serve();

    let err = rpc_error(p.a.call("check", ()).await.unwrap_err());
    assert_eq!(err.code, code::INVALID_CONFIG);
    assert_eq!(err.message, "missing token");
    assert_eq!(err.field(), Some("CF_DNS_API_TOKEN"));
}

#[tokio::test]
async fn plain_error_becomes_internal_error() {
    let p = pair();
    p.b.handle("boom", handler(|_| async { Err(Error::from("exploded")) }));
    p.serve();

    let err = rpc_error(p.a.call("boom", ()).await.unwrap_err());
    assert_eq!(err.code, code::INTERNAL_ERROR);
    assert_eq!(err.message, "exploded");
}

#[tokio::test]
async fn panic_in_handler_becomes_internal_error() {
    let p = pair();
    p.b.handle(
        "panic",
        handler(|_| async {
            if true {
                panic!("kaboom");
            }
            Ok(Value::Null)
        }),
    );
    p.serve();

    let err = rpc_error(p.a.call("panic", ()).await.unwrap_err());
    assert_eq!(err.code, code::INTERNAL_ERROR);
    assert!(err.message.contains("kaboom"), "{}", err.message);
}

#[tokio::test]
async fn notification_is_never_answered() {
    let p = pair();
    let (tx, mut rx) = mpsc::unbounded_channel();
    p.b.handle(
        "event",
        handler(move |params| {
            let tx = tx.clone();
            async move {
                tx.send(params).unwrap();
                // A result and an error must both be dropped for a
                // notification.
                Err(Error::from("ignored too"))
            }
        }),
    );
    // A follow up request proves nothing was written in between.
    p.b.handle("ping", handler(|_| async { Ok(json!({})) }));
    p.serve();

    p.a.notify("event", json!({"n": 1})).await.unwrap();
    let got = tokio::time::timeout(WAIT, rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(got, json!({"n": 1}));
    p.a.call("ping", ()).await.unwrap();
}

#[tokio::test]
async fn concurrent_calls_are_matched_by_id() {
    let p = pair();
    p.b.handle(
        "echo",
        handler(|params| async move {
            let n = params.as_i64().unwrap();
            // Reverse the natural ordering so replies cannot arrive in send
            // order.
            tokio::time::sleep(Duration::from_millis(u64::try_from(20 - n).unwrap())).await;
            Ok(json!(n))
        }),
    );
    p.serve();

    let mut tasks = Vec::new();
    for i in 0..20i64 {
        let a = p.a.clone();
        tasks.push(tokio::spawn(
            async move { (i, a.call("echo", json!(i)).await) },
        ));
    }
    for task in tasks {
        let (i, out) = task.await.unwrap();
        assert_eq!(out.unwrap(), json!(i), "call {i}");
    }
}

#[tokio::test]
async fn bidirectional_calls() {
    let p = pair();
    p.a.handle("host.log", handler(|_| async { Ok(json!({})) }));
    let b = p.b.clone();
    p.b.handle(
        "work",
        handler(move |_| {
            let b = b.clone();
            async move {
                // Call back into the peer while serving its request.
                b.call("host.log", json!({"level": "info"}))
                    .await
                    .map_err(|e| Error::from(e.to_string()))?;
                Ok(json!({}))
            }
        }),
    );
    p.serve();
    p.a.call("work", ()).await.unwrap();
}

/// A connection whose peer is driven by hand.
struct Manual {
    write: tokio::io::WriteHalf<tokio::io::DuplexStream>,
    lines: tokio::io::Lines<BufReader<tokio::io::ReadHalf<tokio::io::DuplexStream>>>,
    conn: Conn,
}

fn manual(setup: impl FnOnce(&Conn)) -> Manual {
    let (x, y) = tokio::io::duplex(8 << 20);
    let (xr, xw) = tokio::io::split(x);
    let (yr, yw) = tokio::io::split(y);
    let conn = Conn::new(xr, xw);
    setup(&conn);
    let serving = conn.clone();
    tokio::spawn(async move {
        let _ = serving.serve().await;
    });
    Manual {
        write: yw,
        lines: BufReader::new(yr).lines(),
        conn,
    }
}

impl Manual {
    async fn next(&mut self) -> Value {
        let line = tokio::time::timeout(WAIT, self.lines.next_line())
            .await
            .expect("timed out")
            .unwrap()
            .expect("the peer closed");
        serde_json::from_str(&line).unwrap()
    }
}

#[tokio::test]
async fn batch_input_produces_one_reply_per_request() {
    let mut m = manual(|c| c.handle("ping", handler(|_| async { Ok(json!({})) })));
    let batch = concat!(
        r#"[{"jsonrpc":"2.0","id":1,"method":"ping"},"#,
        r#"{"jsonrpc":"2.0","method":"ping"},"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"ping"}]"#,
        "\n"
    );
    m.write.write_all(batch.as_bytes()).await.unwrap();

    // Exactly two replies come back: the notification in the middle is
    // silent.
    let mut seen = Vec::new();
    for _ in 0..2 {
        let reply = m.next().await;
        assert!(reply.get("error").is_none(), "{reply}");
        seen.push(reply["id"].as_i64().unwrap());
    }
    seen.sort_unstable();
    assert_eq!(seen, [1, 2]);
}

#[tokio::test]
async fn oversized_message_is_rejected_and_the_stream_stays_usable() {
    let mut m = manual(|c| c.handle("ping", handler(|_| async { Ok(json!({})) })));
    let huge = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"ping\",\"params\":\"{}\"}}\n",
        "x".repeat(MAX_MESSAGE_BYTES + 16)
    );
    m.write.write_all(huge.as_bytes()).await.unwrap();
    m.write
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"ping\"}\n")
        .await
        .unwrap();

    let rejected = m.next().await;
    assert_eq!(rejected["error"]["code"], code::INVALID_REQUEST);
    assert_eq!(rejected["id"], Value::Null);

    let ok = m.next().await;
    assert!(ok.get("error").is_none(), "{ok}");
    assert_eq!(ok["id"], 2);
}

#[tokio::test]
async fn malformed_line_returns_parse_error() {
    let mut m = manual(|_| {});
    m.write.write_all(b"{not json}\n").await.unwrap();
    let reply = m.next().await;
    assert_eq!(reply["error"]["code"], code::PARSE_ERROR);
    assert_eq!(reply["id"], Value::Null);
}

#[tokio::test]
async fn a_message_that_is_not_json_rpc_2_is_an_invalid_request() {
    let mut m = manual(|c| c.handle("ping", handler(|_| async { Ok(json!({})) })));

    // Vector 18: no jsonrpc member, the id is echoed.
    m.write
        .write_all(b"{\"id\": 1, \"method\": \"plugin.ping\"}\n")
        .await
        .unwrap();
    let reply = m.next().await;
    assert_eq!(reply["error"]["code"], code::INVALID_REQUEST);
    assert_eq!(reply["id"], 1);

    // A notification is never answered, whatever it looks like.
    m.write
        .write_all(b"{\"method\": \"ping\"}\n[1]\n")
        .await
        .unwrap();
    let reply = m.next().await;
    assert_eq!(reply["error"]["code"], code::INVALID_REQUEST);
    assert_eq!(reply["id"], Value::Null);

    m.write
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"ping\"}\n")
        .await
        .unwrap();
    assert_eq!(m.next().await["id"], 3);
}

#[tokio::test]
async fn serve_returns_ok_on_eof_and_waits_for_running_handlers() {
    let (x, y) = tokio::io::duplex(1 << 16);
    let (xr, xw) = tokio::io::split(x);
    let (yr, mut yw) = tokio::io::split(y);
    let conn = Conn::new(xr, xw);
    conn.handle(
        "slow",
        handler(|_| async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(json!({"done": true}))
        }),
    );
    let serving = conn.clone();
    let task = tokio::spawn(async move { serving.serve().await });

    yw.write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"slow\"}\n")
        .await
        .unwrap();
    yw.shutdown().await.unwrap();

    let result = tokio::time::timeout(WAIT, task).await.unwrap().unwrap();
    assert!(result.is_ok());
    conn.flush(WAIT).await;

    // The reply of the handler that was running at EOF was written.
    let mut lines = BufReader::new(yr).lines();
    let line = lines.next_line().await.unwrap().unwrap();
    let reply: Value = serde_json::from_str(&line).unwrap();
    assert_eq!(reply["result"]["done"], true);
}

#[tokio::test]
async fn call_after_close_returns_closed() {
    let p = pair();
    p.a.close();
    assert!(matches!(p.a.call("ping", ()).await, Err(CallError::Closed)));
    assert!(matches!(
        p.a.notify("ping", ()).await,
        Err(CallError::Closed)
    ));
    assert!(matches!(p.a.try_notify("ping", ()), Err(CallError::Closed)));
}

#[tokio::test]
async fn closing_fails_the_pending_calls() {
    let p = pair();
    p.b.handle(
        "hang",
        handler(|_| async {
            std::future::pending::<()>().await;
            Ok(Value::Null)
        }),
    );
    p.serve();

    let a = p.a.clone();
    let call = tokio::spawn(async move { a.call("hang", ()).await });
    tokio::time::sleep(Duration::from_millis(50)).await;
    p.a.close();
    let out = tokio::time::timeout(WAIT, call).await.unwrap().unwrap();
    assert!(matches!(out, Err(CallError::Closed)));
}

#[tokio::test]
async fn a_reply_that_is_too_large_becomes_an_internal_error() {
    let p = pair();
    p.b.handle(
        "big",
        handler(|_| async { Ok(Value::String("x".repeat(MAX_MESSAGE_BYTES))) }),
    );
    p.serve();
    let err = rpc_error(p.a.call("big", ()).await.unwrap_err());
    assert_eq!(err.code, code::INTERNAL_ERROR);
}

#[tokio::test]
async fn inline_handlers_run_before_the_next_line() {
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let seen = flag.clone();
    let mut m = manual(|c| {
        let set = flag.clone();
        c.handle_inline("first", move |_| {
            set.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(Value::Null)
        });
        c.handle(
            "second",
            handler(move |_| {
                let seen = seen.clone();
                async move { Ok(json!(seen.load(std::sync::atomic::Ordering::SeqCst))) }
            }),
        );
    });
    m.write
        .write_all(
            b"{\"jsonrpc\":\"2.0\",\"method\":\"first\"}\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"second\"}\n",
        )
        .await
        .unwrap();
    assert_eq!(m.next().await["result"], true);
    assert!(m.conn.handled("first"));
}
