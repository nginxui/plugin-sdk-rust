use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The payload of `http.handle`, the RPC fallback of the `http` capability.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpHandleParams {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
    /// Raw query string.
    pub query: String,
    /// Request headers.
    pub headers: HashMap<String, Vec<String>>,
    /// Request body as standard base64.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body_base64: String,
    /// The nginx-ui user behind the request.
    pub user: HttpUser,
}

/// Identifies the nginx-ui user behind a proxied request.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpUser {
    /// User id.
    pub id: String,
    /// User name.
    pub name: String,
}

/// The reply to `http.handle`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpHandleResult {
    /// HTTP status code.
    pub status: i32,
    /// Response headers.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, Vec<String>>,
    /// Response body as standard base64.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body_base64: String,
}
