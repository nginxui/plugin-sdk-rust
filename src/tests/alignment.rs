//! Fails when a hand written protocol type or a method or error code
//! constant drifts from the proto contract.
//!
//! The field names of a type are read from serde itself: a derived
//! `Deserialize` announces them to `deserialize_struct`. Their shapes are
//! checked by a round trip: a sample document built from the descriptor of
//! the message has to survive being decoded into the type and encoded again.

use std::collections::{BTreeMap, BTreeSet};

use prost::Message;
use prost_types::{
    field_descriptor_proto::{Label, Type},
    DescriptorProto, FileDescriptorSet,
};
use serde::de::{self, DeserializeOwned, Deserializer, Visitor};
use serde::Serialize;
use serde_json::{json, Value};

use crate::pb::registry::RPCS;
use crate::pb::v1 as pb;
use crate::protocol::{self as p, code, method};

/// Proto package of the contract.
const PACKAGE: &str = "nginxui.plugin.v1";

struct Alignment {
    rust: &'static str,
    message: &'static str,
    fields: fn() -> Vec<&'static str>,
    round_trip: fn(&Value) -> Result<Value, String>,
}

macro_rules! align {
    ($ty:ty, $message:literal) => {
        Alignment {
            rust: stringify!($ty),
            message: $message,
            fields: fields_of::<$ty>,
            round_trip: round_trip::<$ty>,
        }
    };
}

/// Every request, result and nested type of the protocol module. Messages
/// without fields are covered by `EmptyResult` implicitly.
fn alignments() -> Vec<Alignment> {
    vec![
        // handshake.rs
        align!(p::HostInfo, "HostInfo"),
        align!(p::InitializeParams, "PluginInitializeRequest"),
        align!(p::InitializeResult, "PluginInitializeResponse"),
        align!(p::ConfigureParams, "PluginConfigureRequest"),
        // host.rs
        align!(p::HostLogParams, "HostLogRequest"),
        align!(p::HostKvGetParams, "HostKVGetRequest"),
        align!(p::HostKvGetResult, "HostKVGetResponse"),
        align!(p::HostKvSetParams, "HostKVSetRequest"),
        align!(p::HostKvGetParams, "HostKVDeleteRequest"),
        align!(p::HostKvListParams, "HostKVListRequest"),
        align!(p::HostKvListResult, "HostKVListResponse"),
        align!(p::HostSettingsGetResult, "HostSettingsGetResponse"),
        align!(p::HostI18nLocaleResult, "HostI18nLocaleResponse"),
        align!(p::HostCredentialsGetParams, "HostCredentialsGetRequest"),
        align!(p::HostCredentialsGetResult, "HostCredentialsGetResponse"),
        align!(p::HostCronRegisterParams, "HostCronRegisterRequest"),
        align!(p::HostCronUnregisterParams, "HostCronUnregisterRequest"),
        align!(p::HostNotifyParams, "HostNotifyRequest"),
        align!(p::HostMetricsSnapshotResult, "HostMetricsSnapshotResponse"),
        align!(p::HostLogsListResult, "HostLogsListResponse"),
        align!(p::HostLogFile, "HostLogFile"),
        align!(p::HostActivitySetParams, "HostActivitySetRequest"),
        align!(p::HostNginxSnippetPutParams, "HostNginxSnippetPutRequest"),
        align!(p::HostNginxSnippetPutResult, "HostNginxSnippetPutResponse"),
        align!(
            p::HostNginxSnippetDeleteParams,
            "HostNginxSnippetDeleteRequest"
        ),
        align!(
            p::HostNginxSnippetDeleteResult,
            "HostNginxSnippetDeleteResponse"
        ),
        align!(
            p::HostNginxSnippetListResult,
            "HostNginxSnippetListResponse"
        ),
        align!(p::HostNginxSnippet, "HostNginxSnippet"),
        align!(p::HostNginxConfigListResult, "HostNginxConfigListResponse"),
        align!(p::HostNginxConfigGetParams, "HostNginxConfigGetRequest"),
        align!(p::HostNginxConfigGetResult, "HostNginxConfigGetResponse"),
        align!(p::HostSitesListResult, "HostSitesListResponse"),
        align!(p::HostSite, "HostSite"),
        align!(p::HostCertsListResult, "HostCertsListResponse"),
        align!(p::HostCert, "HostCert"),
        // dns01.rs
        align!(p::Dns01ChallengeParams, "DNS01PresentRequest"),
        align!(p::Dns01ChallengeParams, "DNS01CleanupRequest"),
        align!(p::Dns01OptionsParams, "DNS01OptionsRequest"),
        align!(p::Dns01OptionsResult, "DNS01OptionsResponse"),
        align!(p::Dns01CheckParams, "DNS01CheckRequest"),
        align!(p::Dns01CheckResult, "DNS01CheckResponse"),
        align!(p::Dns01ValidateParams, "DNS01ValidateRequest"),
        // http.rs
        align!(p::HttpHandleParams, "HTTPHandleRequest"),
        align!(p::HttpUser, "HTTPUser"),
        align!(p::HttpHandleResult, "HTTPHandleResponse"),
        // notify.rs
        align!(p::NotifySendParams, "NotifySendRequest"),
        align!(p::NotifyValidateParams, "NotifyValidateRequest"),
        // probe.rs
        align!(p::ProbeCheckParams, "ProbeCheckRequest"),
        align!(p::ProbeCheckResult, "ProbeCheckResponse"),
        // mcp.rs
        align!(p::McpCallParams, "MCPCallRequest"),
        align!(p::McpCallResult, "MCPCallResponse"),
        align!(p::McpContent, "MCPContent"),
        // storage.rs
        align!(p::StorageValidateParams, "StorageValidateRequest"),
        align!(p::StoragePutParams, "StoragePutRequest"),
        align!(p::StorageSizeResult, "StoragePutResponse"),
        align!(p::StorageGetParams, "StorageGetRequest"),
        align!(p::StorageSizeResult, "StorageGetResponse"),
        align!(p::StorageListParams, "StorageListRequest"),
        align!(p::StorageListResult, "StorageListResponse"),
        align!(p::StorageObject, "StorageObject"),
        align!(p::StorageDeleteParams, "StorageDeleteRequest"),
        // deploy.rs
        align!(p::DeployValidateParams, "DeployValidateRequest"),
        align!(p::DeployPushParams, "DeployPushRequest"),
        align!(p::DeployCertificate, "DeployCertificate"),
        align!(p::DeployPushResult, "DeployPushResponse"),
        // blocklist.rs
        align!(p::BlocklistFetchParams, "BlocklistFetchRequest"),
        align!(p::BlocklistFetchResult, "BlocklistFetchResponse"),
        align!(p::BlocklistEntry, "BlocklistEntry"),
        // discovery.rs
        align!(p::DiscoveryResolveParams, "DiscoveryResolveRequest"),
        align!(p::DiscoveryResolveResult, "DiscoveryResolveResponse"),
        align!(p::DiscoveryTarget, "DiscoveryTarget"),
        // logsink.rs
        align!(p::LogSinkPushParams, "LogSinkPushRequest"),
        align!(p::LogEntry, "LogEntry"),
        align!(p::LogSinkPushResult, "LogSinkPushResponse"),
        // events.rs
        align!(p::EventNotification, "EventsOnRequest"),
        // errors.rs
        align!(p::Error, "PluginError"),
        align!(p::InvalidConfigData, "InvalidConfigData"),
        // manifest.rs
        align!(p::Manifest, "Manifest"),
        align!(p::ManifestI18n, "ManifestI18n"),
        align!(p::ManifestServer, "ManifestServer"),
        align!(p::ManifestResources, "ManifestResources"),
        align!(p::ManifestWebapp, "ManifestWebapp"),
        align!(p::ManifestPage, "ManifestPage"),
        align!(p::ManifestContent, "ManifestContent"),
        align!(p::ManifestRequirement, "ManifestRequirement"),
        align!(p::ManifestCron, "ManifestCron"),
        align!(p::ManifestDns01, "ManifestDNS01"),
        align!(p::Dns01Provider, "DNS01Provider"),
        align!(p::Dns01ProviderLinks, "DNS01ProviderLinks"),
        align!(p::Dns01ProviderForm, "DNS01ProviderForm"),
        align!(p::Dns01ProviderField, "DNS01ProviderField"),
        align!(p::Dns01ProviderMethod, "DNS01ProviderMethod"),
        align!(p::ManifestHttp, "ManifestHTTP"),
        align!(p::SettingsSchema, "SettingsSchema"),
        align!(p::SettingsField, "SettingsField"),
        align!(p::SettingsOption, "SettingsOption"),
        align!(p::ManifestNotify, "ManifestNotify"),
        align!(p::NotifyChannel, "NotifyChannel"),
        align!(p::ManifestProbe, "ManifestProbe"),
        align!(p::ProbeKind, "ProbeKind"),
        align!(p::ConfigurationSchema, "ConfigurationSchema"),
        align!(p::ConfigurationField, "ConfigurationField"),
        align!(p::ManifestMcp, "ManifestMCP"),
        align!(p::McpTool, "MCPTool"),
        align!(p::ManifestStorage, "ManifestStorage"),
        align!(p::StorageBackend, "StorageBackend"),
        align!(p::ManifestDeploy, "ManifestDeploy"),
        align!(p::DeployTarget, "DeployTarget"),
        align!(p::ManifestBlocklist, "ManifestBlocklist"),
        align!(p::BlocklistSource, "BlocklistSource"),
        align!(p::ManifestDiscovery, "ManifestDiscovery"),
        align!(p::DiscoveryProvider, "DiscoveryProvider"),
        align!(p::ManifestLogSink, "ManifestLogSink"),
    ]
}

// Reading the field names of a serde type.

struct Captured(Vec<&'static str>);

#[derive(Debug)]
struct CaptureError(String);

impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CaptureError {}

impl de::Error for CaptureError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        CaptureError(msg.to_string())
    }
}

thread_local! {
    static CAPTURED: std::cell::RefCell<Option<Captured>> = const { std::cell::RefCell::new(None) };
}

/// A deserializer that only listens to `deserialize_struct`.
struct FieldNames;

impl<'de> Deserializer<'de> for FieldNames {
    type Error = CaptureError;

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, CaptureError> {
        CAPTURED.with(|c| *c.borrow_mut() = Some(Captured(fields.to_vec())));
        Err(CaptureError("captured".into()))
    }

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, CaptureError> {
        Err(CaptureError("not a struct".into()))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf
        option unit unit_struct newtype_struct seq tuple tuple_struct map enum identifier
        ignored_any
    }
}

fn fields_of<T: DeserializeOwned>() -> Vec<&'static str> {
    CAPTURED.with(|c| *c.borrow_mut() = None);
    let _ = T::deserialize(FieldNames);
    CAPTURED
        .with(|c| c.borrow_mut().take())
        .unwrap_or_else(|| panic!("{} is not a struct", std::any::type_name::<T>()))
        .0
}

fn round_trip<T: DeserializeOwned + Serialize>(sample: &Value) -> Result<Value, String> {
    let value: T = serde_json::from_value(sample.clone()).map_err(|e| e.to_string())?;
    serde_json::to_value(&value).map_err(|e| e.to_string())
}

// Reading the descriptors.

struct Messages(BTreeMap<String, DescriptorProto>);

fn messages() -> Messages {
    let set =
        FileDescriptorSet::decode(crate::pb::DESCRIPTORS).expect("the descriptor set decodes");
    let mut out = BTreeMap::new();
    for file in set.file {
        assert_eq!(file.package(), PACKAGE);
        for message in file.message_type {
            out.insert(message.name().to_owned(), message);
        }
    }
    Messages(out)
}

impl Messages {
    fn get(&self, name: &str) -> &DescriptorProto {
        self.0
            .get(name)
            .unwrap_or_else(|| panic!("proto message {name} does not exist"))
    }

    /// Builds a document with every field of the message set to a value that
    /// is not the default.
    fn sample(&self, message: &DescriptorProto) -> Value {
        let mut object = serde_json::Map::new();
        for field in &message.field {
            let name = field.name().to_owned();
            let value = if field.label() == Label::Repeated {
                self.repeated_sample(message, field)
            } else {
                self.singular_sample(field)
            };
            object.insert(name, value);
        }
        Value::Object(object)
    }

    fn repeated_sample(
        &self,
        message: &DescriptorProto,
        field: &prost_types::FieldDescriptorProto,
    ) -> Value {
        // A map is a repeated nested `...Entry` message.
        if field.r#type() == Type::Message {
            let type_name = field.type_name();
            if let Some(entry) = message.nested_type.iter().find(|n| {
                type_name.ends_with(&format!(".{}", n.name()))
                    && n.options.as_ref().is_some_and(|o| o.map_entry())
            }) {
                let value_field = entry
                    .field
                    .iter()
                    .find(|f| f.name() == "value")
                    .expect("map value");
                return json!({"k": self.singular_sample(value_field)});
            }
        }
        json!([self.singular_sample(field)])
    }

    fn singular_sample(&self, field: &prost_types::FieldDescriptorProto) -> Value {
        match field.r#type() {
            Type::String => json!("x"),
            Type::Bytes => json!("eA=="),
            Type::Bool => json!(true),
            Type::Double | Type::Float => json!(2),
            Type::Int32
            | Type::Sint32
            | Type::Sfixed32
            | Type::Uint32
            | Type::Fixed32
            | Type::Int64
            | Type::Uint64
            | Type::Sint64
            | Type::Fixed64
            | Type::Sfixed64 => json!(3),
            Type::Message => match field.type_name() {
                ".google.protobuf.Struct" => json!({"k": "x"}),
                ".google.protobuf.Value" => json!("x"),
                ".google.protobuf.ListValue" => json!(["x"]),
                other => {
                    let name = other.rsplit('.').next().unwrap();
                    self.sample(self.get(name))
                }
            },
            other => panic!(
                "field {}: the sample of {other:?} is not defined",
                field.name()
            ),
        }
    }
}

/// Compares two documents, numbers by value.
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w)))
        }
        _ => a == b,
    }
}

#[test]
fn proto_alignment() {
    let messages = messages();
    for a in alignments() {
        let descriptor = messages.get(a.message);
        let proto: BTreeSet<&str> = descriptor.field.iter().map(|f| f.name()).collect();
        let rust: BTreeSet<&str> = (a.fields)().into_iter().collect();

        if let Some(name) = rust.difference(&proto).next() {
            panic!(
                "{}: serde field {name:?} has no field in {}",
                a.rust, a.message
            );
        }
        if let Some(name) = proto.difference(&rust).next() {
            panic!("{}: field {}.{name} has no serde field", a.rust, a.message);
        }

        let sample = messages.sample(descriptor);
        match (a.round_trip)(&sample) {
            Ok(out) => assert!(
                same(&out, &sample),
                "{} does not carry {} unchanged:\n  sample {sample}\n  result {out}",
                a.rust,
                a.message
            ),
            Err(e) => panic!(
                "{} cannot decode a sample of {}: {e}\n  sample {sample}",
                a.rust, a.message
            ),
        }
    }
}

#[test]
fn every_proto_message_is_aligned() {
    let known: BTreeSet<&str> = alignments().iter().map(|a| a.message).collect();
    assert_eq!(
        fields_of::<p::EmptyResult>().len(),
        0,
        "EmptyResult must have no fields"
    );
    for (name, message) in &messages().0 {
        if message.field.is_empty() {
            continue;
        }
        assert!(
            known.contains(name.as_str()),
            "proto message {name} has no hand written counterpart in the alignment table"
        );
    }
}

#[test]
fn method_names_match_proto() {
    let requests = [
        method::INITIALIZE,
        method::CONFIGURE,
        method::PING,
        method::SHUTDOWN,
        method::DNS01_PRESENT,
        method::DNS01_CLEANUP,
        method::DNS01_OPTIONS,
        method::DNS01_CHECK,
        method::DNS01_VALIDATE,
        method::HTTP_HANDLE,
        method::NOTIFY_SEND,
        method::NOTIFY_VALIDATE,
        method::PROBE_CHECK,
        method::MCP_CALL,
        method::STORAGE_VALIDATE,
        method::STORAGE_PUT,
        method::STORAGE_GET,
        method::STORAGE_LIST,
        method::STORAGE_DELETE,
        method::DEPLOY_VALIDATE,
        method::DEPLOY_PUSH,
        method::BLOCKLIST_FETCH,
        method::DISCOVERY_RESOLVE,
        method::HOST_LOG,
        method::HOST_KV_GET,
        method::HOST_KV_SET,
        method::HOST_KV_DELETE,
        method::HOST_KV_LIST,
        method::HOST_SETTINGS_GET,
        method::HOST_I18N_LOCALE,
        method::HOST_CREDENTIALS_GET,
        method::HOST_CRON_REGISTER,
        method::HOST_CRON_UNREGISTER,
        method::HOST_NOTIFY,
        method::HOST_METRICS_SNAPSHOT,
        method::HOST_LOGS_LIST,
        method::HOST_ACTIVITY_SET,
        method::HOST_NGINX_SNIPPET_PUT,
        method::HOST_NGINX_SNIPPET_DELETE,
        method::HOST_NGINX_SNIPPET_LIST,
        method::HOST_NGINX_CONFIG_LIST,
        method::HOST_NGINX_CONFIG_GET,
        method::HOST_SITES_LIST,
        method::HOST_CERTS_LIST,
    ];
    let notifications = [method::INITIALIZED, method::EXIT, method::EVENTS_ON];
    let streams = [method::LOG_PUSH];

    let mut proto_requests = BTreeSet::new();
    let mut proto_notifications = BTreeSet::new();
    let mut proto_streams = BTreeSet::new();
    for rpc in RPCS {
        if rpc.streaming {
            proto_streams.insert(rpc.rpc_name);
        } else if rpc.notification {
            proto_notifications.insert(rpc.rpc_name);
        } else {
            proto_requests.insert(rpc.rpc_name);
        }
    }

    let set = |names: &[&'static str]| names.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(set(&requests), proto_requests, "request rpcs");
    assert_eq!(
        set(&notifications),
        proto_notifications,
        "notification rpcs"
    );
    assert_eq!(set(&streams), proto_streams, "streaming rpcs");
    assert_eq!(
        requests.len(),
        set(&requests).len(),
        "a request constant is listed twice"
    );
}

#[test]
fn error_codes_match_proto() {
    use pb::ErrorCode;
    let codes = [
        (ErrorCode::ParseError, code::PARSE_ERROR),
        (ErrorCode::InvalidRequest, code::INVALID_REQUEST),
        (ErrorCode::MethodNotFound, code::METHOD_NOT_FOUND),
        (ErrorCode::InvalidParams, code::INVALID_PARAMS),
        (ErrorCode::InternalError, code::INTERNAL_ERROR),
        (ErrorCode::PermissionDenied, code::PERMISSION_DENIED),
        (ErrorCode::Unsupported, code::UNSUPPORTED),
        (ErrorCode::InvalidConfig, code::INVALID_CONFIG),
    ];
    for (value, constant) in codes {
        assert_eq!(value as i32, constant, "{value:?}");
    }

    // Every enum value of the descriptor has a constant.
    let set = FileDescriptorSet::decode(crate::pb::DESCRIPTORS).unwrap();
    let mut values: Vec<i32> = set
        .file
        .iter()
        .flat_map(|f| f.enum_type.iter())
        .filter(|e| e.name() == "ErrorCode")
        .flat_map(|e| e.value.iter().map(|v| v.number()))
        .filter(|n| *n != 0)
        .collect();
    values.sort_unstable();
    let mut constants: Vec<i32> = codes.iter().map(|(_, c)| *c).collect();
    constants.sort_unstable();
    assert_eq!(
        values, constants,
        "ErrorCode values and code constants differ"
    );
}

/// Compares the rpc table with `gen/methods.json` when the spec checkout is
/// there, which catches a `pb` that lags behind the spec.
#[test]
fn rpcs_match_the_contract_methods_file() {
    let dir = std::env::var_os("SPEC_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../plugin-spec"));
    let Ok(raw) = std::fs::read_to_string(dir.join("gen/methods.json")) else {
        eprintln!(
            "skipped: {} is not there",
            dir.join("gen/methods.json").display()
        );
        return;
    };
    let file: Vec<Value> = serde_json::from_str(&raw).unwrap();

    let mut from_spec: Vec<(String, String, bool, bool)> = file
        .iter()
        .map(|m| {
            (
                m["rpc_name"].as_str().unwrap().to_owned(),
                m["full_method"].as_str().unwrap().to_owned(),
                m["notification"].as_bool().unwrap_or(false),
                m["streaming"].as_bool().unwrap_or(false),
            )
        })
        .collect();
    let mut from_pb: Vec<(String, String, bool, bool)> = RPCS
        .iter()
        .map(|r| {
            (
                r.rpc_name.to_owned(),
                r.full_method.to_owned(),
                r.notification,
                r.streaming,
            )
        })
        .collect();
    from_spec.sort();
    from_pb.sort();
    assert_eq!(
        from_pb, from_spec,
        "src/pb is stale, run `cargo xtask generate`"
    );
}

/// The table holds only rpcs of the contract package.
#[test]
fn registry_messages_exist() {
    let messages = messages();
    for rpc in RPCS {
        messages.get(rpc.request);
        messages.get(rpc.response);
        assert!(
            rpc.full_method.starts_with(&format!("/{PACKAGE}.")),
            "{}",
            rpc.full_method
        );
    }
}
