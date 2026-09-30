//! Runs the built plugin as a process, the way the host does: a handshake on
//! stdin and stdout, then each of the ways the process is asked to stop.

#![cfg(unix)]

use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use serde_json::{json, Value};

struct Plugin {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    data_dir: PathBuf,
}

impl Plugin {
    fn start() -> Plugin {
        let data_dir =
            std::env::temp_dir().join(format!("refp{}-{}", std::process::id(), rand_suffix()));
        std::fs::create_dir_all(&data_dir).unwrap();
        let mut child = Command::new(env!("CARGO_BIN_EXE_reference-plugin"))
            .env("NGINX_UI_PLUGIN_DATA_DIR", &data_dir)
            .env("NGINX_UI_PLUGIN_ID", "io.github.nginxui.reference")
            .env("NGINX_UI_PLUGIN_HTTP_SECRET", "secret")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start the plugin");
        let stdin = child.stdin.take();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Plugin {
            child,
            stdin,
            stdout,
            data_dir,
        }
    }

    fn send(&mut self, frame: &Value) {
        let stdin = self.stdin.as_mut().expect("stdin is open");
        writeln!(stdin, "{frame}").unwrap();
        stdin.flush().unwrap();
    }

    fn recv(&mut self) -> Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("stdout holds no frame ({e}): {line:?}"))
    }

    /// Reads frames until the reply with `id`, the plugin may log to the host
    /// meanwhile.
    fn recv_reply(&mut self, id: i64) -> Value {
        loop {
            let frame = self.recv();
            if frame["id"] == id {
                return frame;
            }
            assert_eq!(frame["method"], "host.log", "{frame}");
        }
    }

    fn handshake(&mut self) -> Value {
        self.send(&json!({
            "jsonrpc": "2.0", "id": 1, "method": "plugin.initialize",
            "params": {"host": {"version": "2.7.0"}, "settings": {}, "permissions": []},
        }));
        let reply = self.recv_reply(1);
        self.send(&json!({"jsonrpc": "2.0", "method": "plugin.initialized"}));
        reply["result"].clone()
    }

    fn files(&self) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(&self.data_dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    fn signal(&self, name: &str) {
        let status = Command::new("kill")
            .arg(format!("-{name}"))
            .arg(self.child.id().to_string())
            .status()
            .unwrap();
        assert!(status.success());
    }

    /// Waits for the exit and returns the code, and what was left on stdout.
    fn finish(mut self) -> (i32, String, Vec<String>) {
        let started = Instant::now();
        let code = loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                break status.code().unwrap_or(-1);
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "the plugin did not exit"
            );
            std::thread::sleep(Duration::from_millis(20));
        };
        let mut rest = String::new();
        self.stdout.read_to_string(&mut rest).unwrap();
        let files = self.files();
        let _ = std::fs::remove_dir_all(&self.data_dir);
        (code, rest, files)
    }
}

/// A number that is different for every call in this process.
fn rand_suffix() -> usize {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    NEXT.fetch_add(1, Ordering::SeqCst)
}

fn assert_clean_exit(plugin: Plugin) {
    let (code, stdout, files) = plugin.finish();
    assert_eq!(code, 0);
    assert_eq!(stdout, "", "stdout carries frames only");
    assert!(files.is_empty(), "sockets are removed on exit: {files:?}");
}

#[test]
fn the_handshake_advertises_both_transports_and_opens_the_sockets() {
    let mut plugin = Plugin::start();
    let result = plugin.handshake();
    assert_eq!(result["api_version"], 1);
    assert_eq!(result["transports"], json!(["stdio", "grpc"]));
    assert_eq!(result["capabilities"].as_array().unwrap().len(), 10);
    let socket = result["rpc_socket"].as_str().unwrap();
    assert!(socket.ends_with("rpc.sock"), "{socket}");
    assert_eq!(plugin.files(), ["http.sock", "rpc.sock"]);
    plugin.signal("TERM");
    assert_clean_exit(plugin);
}

#[test]
fn sigterm_stops_the_plugin() {
    let mut plugin = Plugin::start();
    plugin.handshake();
    plugin.signal("TERM");
    assert_clean_exit(plugin);
}

#[test]
fn sigint_stops_the_plugin() {
    let mut plugin = Plugin::start();
    plugin.handshake();
    plugin.signal("INT");
    assert_clean_exit(plugin);
}

#[test]
fn the_end_of_stdin_stops_the_plugin() {
    let mut plugin = Plugin::start();
    plugin.handshake();
    plugin.stdin = None;
    assert_clean_exit(plugin);
}

#[test]
fn shutdown_and_exit_stop_the_plugin() {
    let mut plugin = Plugin::start();
    plugin.handshake();
    plugin.send(&json!({"jsonrpc": "2.0", "id": 2, "method": "plugin.shutdown"}));
    assert_eq!(
        plugin.recv_reply(2),
        json!({"jsonrpc": "2.0", "id": 2, "result": {}})
    );
    plugin.send(&json!({"jsonrpc": "2.0", "method": "plugin.exit"}));
    assert_clean_exit(plugin);
}

#[test]
fn a_plugin_that_is_killed_leaves_a_socket_the_next_start_replaces() {
    let mut plugin = Plugin::start();
    plugin.handshake();
    let data_dir = plugin.data_dir.clone();
    plugin.signal("KILL");
    let _ = plugin.child.wait();
    assert!(
        data_dir.join("rpc.sock").exists(),
        "the killed process left its socket"
    );

    // A new process on the same directory starts fine.
    let mut child = Command::new(env!("CARGO_BIN_EXE_reference-plugin"))
        .env("NGINX_UI_PLUGIN_DATA_DIR", &data_dir)
        .env("NGINX_UI_PLUGIN_HTTP_SECRET", "secret")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":1,"method":"plugin.initialize","params":{{}}}}"#
    )
    .unwrap();
    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    let reply: Value = serde_json::from_str(&line).unwrap();
    assert_eq!(
        reply["result"]["transports"],
        json!(["stdio", "grpc"]),
        "{reply}"
    );
    drop(stdin);
    let _ = child.wait();
    let _ = std::fs::remove_dir_all(&data_dir);
}
