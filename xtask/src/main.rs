//! Regenerates `src/pb` from the proto files of the plugin-spec repository.
//!
//! Usage: `cargo xtask generate [--check]`
//!
//! The spec checkout is expected next to this repository (`../plugin-spec`),
//! set `SPEC_DIR` to use another path. `--check` only reports files that
//! differ from what the generator would write. protoc comes from the
//! `protoc-bin-vendored` crate, no system protoc is needed.

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use heck::ToUpperCamelCase;
use prost::Message;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Proto package of the contract.
const PACKAGE: &str = "nginxui.plugin.v1";

/// Well known types replaced by the hand written shims in `src/pb/wkt.rs`.
const WKT: [(&str, &str); 3] = [
    (".google.protobuf.Struct", "crate::pb::wkt::Struct"),
    (".google.protobuf.Value", "crate::pb::wkt::Value"),
    (".google.protobuf.ListValue", "crate::pb::wkt::ListValue"),
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let check = args.iter().any(|a| a == "--check");
    if args.first().map(String::as_str) != Some("generate") {
        eprintln!("usage: cargo xtask generate [--check]");
        return ExitCode::from(2);
    }
    match generate(check) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("xtask: {err}");
            ExitCode::FAILURE
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives in the repository")
        .to_path_buf()
}

fn generate(check: bool) -> Result<bool> {
    let root = repo_root();
    let spec = env::var_os("SPEC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("../plugin-spec"));
    let proto_root = spec.join("proto");
    let proto_dir = proto_root.join("nginxui/plugin/v1");
    if !proto_dir.is_dir() {
        return Err(format!("{} not found, set SPEC_DIR", proto_dir.display()).into());
    }

    let mut protos: Vec<PathBuf> = fs::read_dir(&proto_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "proto"))
        .collect();
    protos.sort();

    let tmp = env::temp_dir().join(format!("nginxui-plugin-sdk-gen-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let prost_out = tmp.join("prost");
    let serde_out = tmp.join("serde");
    fs::create_dir_all(&prost_out)?;
    fs::create_dir_all(&serde_out)?;
    let fds_path = tmp.join("descriptors.bin");

    // Message code.
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let include = protoc_bin_vendored::include_path()?;
    let mut config = prost_build::Config::new();
    config
        .protoc_executable(&protoc)
        .out_dir(&prost_out)
        .file_descriptor_set_path(&fds_path)
        .btree_map(["."])
        .disable_comments(["."]);
    for (proto, rust) in WKT {
        config.extern_path(proto, rust);
    }
    config.compile_protos(&protos, &[proto_root.clone(), include])?;

    // JSON mapping code with proto field names.
    let fds_bytes = fs::read(&fds_path)?;
    let mut builder = pbjson_build::Builder::new();
    builder
        .out_dir(&serde_out)
        .register_descriptors(&fds_bytes)?
        .preserve_proto_field_names()
        .ignore_unknown_fields()
        .btree_map(["."]);
    for (proto, rust) in WKT {
        builder.extern_path(proto, rust);
    }
    builder.build(&[format!(".{PACKAGE}")])?;

    // Registry of the rpcs, with the options prost-types cannot keep.
    let registry = build_registry(&fds_bytes)?;

    // Slim descriptor set of the contract package for the alignment tests.
    let slim = slim_descriptors(&fds_bytes)?;

    let out = root.join("src/pb");
    let files: Vec<(PathBuf, Vec<u8>)> = vec![
        (
            out.join(format!("{PACKAGE}.rs")),
            fs::read(prost_out.join(format!("{PACKAGE}.rs")))?,
        ),
        (
            out.join(format!("{PACKAGE}.serde.rs")),
            fs::read(serde_out.join(format!("{PACKAGE}.serde.rs")))?,
        ),
        (out.join("registry.rs"), registry.into_bytes()),
        (out.join("descriptors.bin"), slim),
    ];

    let mut clean = true;
    for (path, mut bytes) in files {
        if path.extension().is_some_and(|x| x == "rs") {
            bytes = rustfmt(&bytes)?;
        }
        let same = fs::read(&path).map(|old| old == bytes).unwrap_or(false);
        if check {
            if !same {
                eprintln!("stale: {}", path.display());
                clean = false;
            }
        } else if !same {
            fs::write(&path, &bytes)?;
            println!("wrote {}", path.display());
        }
    }
    let _ = fs::remove_dir_all(&tmp);
    if check && clean {
        println!("src/pb matches {}", spec.display());
    }
    Ok(clean)
}

/// Formats generated code the way `cargo fmt` would.
fn rustfmt(source: &[u8]) -> Result<Vec<u8>> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2021", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().expect("piped");
    let input = source.to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let output = child.wait_with_output()?;
    writer.join().map_err(|_| "rustfmt writer panicked")??;
    if !output.status.success() {
        return Err("rustfmt failed on generated code".into());
    }
    Ok(output.stdout)
}

// Minimal descriptor messages. Only the fields the generator reads are
// declared, prost skips the rest, and the method options carry the three
// extensions of options.proto that prost-types drops.

#[derive(Clone, PartialEq, Message)]
struct Fds {
    #[prost(message, repeated, tag = "1")]
    file: Vec<FileProto>,
}

#[derive(Clone, PartialEq, Message)]
struct FileProto {
    #[prost(string, optional, tag = "2")]
    package: Option<String>,
    #[prost(message, repeated, tag = "6")]
    service: Vec<ServiceProto>,
}

#[derive(Clone, PartialEq, Message)]
struct ServiceProto {
    #[prost(string, optional, tag = "1")]
    name: Option<String>,
    #[prost(message, repeated, tag = "2")]
    method: Vec<MethodProto>,
}

#[derive(Clone, PartialEq, Message)]
struct MethodProto {
    #[prost(string, optional, tag = "1")]
    name: Option<String>,
    #[prost(string, optional, tag = "2")]
    input_type: Option<String>,
    #[prost(string, optional, tag = "3")]
    output_type: Option<String>,
    #[prost(message, optional, tag = "4")]
    options: Option<MethodOptionsExt>,
    #[prost(bool, optional, tag = "5")]
    client_streaming: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    server_streaming: Option<bool>,
}

#[derive(Clone, PartialEq, Message)]
struct MethodOptionsExt {
    #[prost(string, optional, tag = "52101")]
    rpc_name: Option<String>,
    #[prost(bool, optional, tag = "52102")]
    notification: Option<bool>,
    #[prost(bool, optional, tag = "52103")]
    streaming: Option<bool>,
}

struct RpcInfo {
    rpc_name: String,
    service: String,
    method: String,
    /// Proto name of the request message.
    request: String,
    /// Proto name of the response message.
    response: String,
    notification: bool,
    streaming: bool,
}

/// Strips the package from the type name of a message.
fn proto_name(proto_type: &str) -> Result<String> {
    let prefix = format!(".{PACKAGE}.");
    proto_type
        .strip_prefix(&prefix)
        .map(str::to_owned)
        .ok_or_else(|| format!("message {proto_type} is outside the contract package").into())
}

fn build_registry(fds_bytes: &[u8]) -> Result<String> {
    let fds = Fds::decode(fds_bytes)?;
    let mut rpcs: BTreeMap<String, RpcInfo> = BTreeMap::new();
    for file in fds
        .file
        .iter()
        .filter(|f| f.package.as_deref() == Some(PACKAGE))
    {
        for service in &file.service {
            let service_name = service.name.clone().unwrap_or_default();
            for method in &service.method {
                let options = method.options.clone().unwrap_or_default();
                let rpc_name = options.rpc_name.clone().unwrap_or_default();
                if rpc_name.is_empty() {
                    return Err(format!("{service_name}.{:?} has no rpc_name", method.name).into());
                }
                let client_streaming = method.client_streaming.unwrap_or(false);
                let streaming = options.streaming.unwrap_or(false);
                if streaming != client_streaming || method.server_streaming.unwrap_or(false) {
                    return Err(format!(
                        "{rpc_name}: only client streams marked streaming are part of the contract"
                    )
                    .into());
                }
                let info = RpcInfo {
                    rpc_name: rpc_name.clone(),
                    service: service_name.clone(),
                    method: method.name.clone().unwrap_or_default(),
                    request: proto_name(method.input_type.as_deref().unwrap_or_default())?,
                    response: proto_name(method.output_type.as_deref().unwrap_or_default())?,
                    notification: options.notification.unwrap_or(false),
                    streaming,
                };
                if rpcs.insert(rpc_name.clone(), info).is_some() {
                    return Err(format!("rpc_name {rpc_name} is not unique").into());
                }
            }
        }
    }

    let mut out = String::new();
    out.push_str(
        "// @generated by `cargo xtask generate`. Do not edit.\n\n\
         //! Table of every rpc of the contract.\n\n\
         use super::{codec, v1};\n\
         use serde_json::Value;\n\n\
         /// One rpc of the contract, with the codecs between its protobuf messages\n\
         /// and the protobuf JSON mapping the stdio transport carries.\n\
         #[derive(Clone, Copy)]\n\
         pub struct Rpc {\n\
         \x20   /// JSON-RPC method name, the `rpc_name` option.\n\
         \x20   pub rpc_name: &'static str,\n\
         \x20   /// Fully qualified service name.\n\
         \x20   pub service: &'static str,\n\
         \x20   /// Method name inside the service.\n\
         \x20   pub method: &'static str,\n\
         \x20   /// gRPC path, `/<service>/<method>`.\n\
         \x20   pub full_method: &'static str,\n\
         \x20   /// Proto name of the request message.\n\
         \x20   pub request: &'static str,\n\
         \x20   /// Proto name of the response message.\n\
         \x20   pub response: &'static str,\n\
         \x20   /// One way message, sent without an id on stdio.\n\
         \x20   pub notification: bool,\n\
         \x20   /// Client streaming rpc, served on gRPC only.\n\
         \x20   pub streaming: bool,\n\
         \x20   /// Decodes request bytes into the JSON params of the method.\n\
         \x20   pub request_to_json: fn(&[u8]) -> Result<Value, String>,\n\
         \x20   /// Encodes JSON params as request bytes.\n\
         \x20   pub json_to_request: fn(&Value) -> Result<Vec<u8>, String>,\n\
         \x20   /// Decodes response bytes into the JSON result of the method.\n\
         \x20   pub response_to_json: fn(&[u8]) -> Result<Value, String>,\n\
         \x20   /// Encodes a JSON result as response bytes, dropping unknown members.\n\
         \x20   pub json_to_response: fn(&Value) -> Result<Vec<u8>, String>,\n\
         }\n\n\
         /// Every rpc of the contract, sorted by `rpc_name`.\n\
         pub static RPCS: &[Rpc] = &[\n",
    );
    for info in rpcs.values() {
        let full = format!("/{PACKAGE}.{}/{}", info.service, info.method);
        write!(
            out,
            "    Rpc {{\n\
             \x20       rpc_name: {:?},\n\
             \x20       service: \"{PACKAGE}.{}\",\n\
             \x20       method: {:?},\n\
             \x20       full_method: {:?},\n\
             \x20       request: {:?},\n\
             \x20       response: {:?},\n\
             \x20       notification: {},\n\
             \x20       streaming: {},\n\
             \x20       request_to_json: codec::to_json::<v1::{}>,\n\
             \x20       json_to_request: codec::from_json::<v1::{}>,\n\
             \x20       response_to_json: codec::to_json::<v1::{}>,\n\
             \x20       json_to_response: codec::from_json::<v1::{}>,\n\
             \x20   }},\n",
            info.rpc_name,
            info.service,
            info.method,
            full,
            info.request,
            info.response,
            info.notification,
            info.streaming,
            info.request.to_upper_camel_case(),
            info.request.to_upper_camel_case(),
            info.response.to_upper_camel_case(),
            info.response.to_upper_camel_case(),
        )?;
    }
    out.push_str("];\n");
    Ok(out)
}

/// Keeps the files of the contract package, without imports.
fn slim_descriptors(fds_bytes: &[u8]) -> Result<Vec<u8>> {
    let mut fds = prost_types::FileDescriptorSet::decode(fds_bytes)?;
    fds.file.retain(|f| f.package.as_deref() == Some(PACKAGE));
    fds.file.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(fds.encode_to_vec())
}
