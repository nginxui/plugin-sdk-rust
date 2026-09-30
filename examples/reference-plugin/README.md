# Reference plugin

A plugin for NGINX UI that implements every capability of the plugin contract
with the Rust SDK: `dns01`, `http`, `notify`, `probe`, `mcp`, `storage`,
`cert.deploy`, `security.blocklist`, `upstream.discovery` and `log.sink`. It
also handles events, a cron method and settings.

It keeps state in memory or in its data directory and contacts no vendor, so
it runs anywhere. Read `src/main.rs` to see how each capability is wired, and
`plugin.json.in` for the manifest block each one needs.

## Permissions

* `network`: the probe opens a TCP connection to its target.
* `kv`: the dns01 provider notes the records it published.
* `cron`: the manifest schedules `cron.tick`.
* `notify`: a plugin that sends notifications raises host notifications.
* `mcp`: publishes the tools of the `mcp` capability.
* `cert.deploy`: receives certificates and their private keys.
* `log.read` and `log.files`: receives access log lines and lists log files.

## Supported platforms

Any platform Rust builds for. `package.sh` builds the one it runs on, a
release lists a package per platform.

## Build a package

```sh
./package.sh    # the plugin directory for this platform in ../../target/plugin-package
```

`plugin.json.in` is the manifest with the platform left open, `package.sh`
fills it in.

## Check it against the reference host

```sh
nginx-ui plugin lint ../../target/plugin-package
nginx-ui plugin conformance ../../target/plugin-package
nginx-ui plugin conformance ../../target/plugin-package --transport grpc
```
