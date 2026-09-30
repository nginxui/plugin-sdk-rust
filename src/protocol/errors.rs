use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 error codes used on the wire.
pub mod code {
    /// The frame was not valid JSON.
    pub const PARSE_ERROR: i32 = -32700;
    /// The frame was valid JSON but not a valid JSON-RPC 2.0 message.
    pub const INVALID_REQUEST: i32 = -32600;
    /// No handler is registered for the method.
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// The params failed to decode into the request message.
    pub const INVALID_PARAMS: i32 = -32602;
    /// The handler failed for a reason no other code covers.
    pub const INTERNAL_ERROR: i32 = -32000;
    /// A plugin called a `host.*` method without holding the matching
    /// permission.
    pub const PERMISSION_DENIED: i32 = -32001;
    /// The plugin does not implement a capability method it is asked for.
    pub const UNSUPPORTED: i32 = -32002;
    /// Credentials or settings failed validation. `data.field` names the
    /// offending field.
    pub const INVALID_CONFIG: i32 = -32003;
}

/// The JSON-RPC error object.
///
/// It is the error type of every handler: return one to send it verbatim, any
/// other failure becomes an internal error (see the `From` implementations).
/// The constructors in this module build the errors of the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// One of [`code`], or a method specific negative integer.
    pub code: i32,
    /// Short human readable text. It must never contain a secret.
    #[serde(default)]
    pub message: String,
    /// Structured detail, an object when the contract defines one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    /// Builds an error with an arbitrary code and payload.
    pub fn new(code: i32, message: impl Into<String>, data: Option<Value>) -> Self {
        Error {
            code,
            message: message.into(),
            data,
        }
    }

    /// Reports a bad credential or setting. `field` names the offending input
    /// so the host can highlight it in the form.
    pub fn invalid_config(field: impl Into<String>, message: impl Into<String>) -> Self {
        let data = InvalidConfigData {
            field: field.into(),
        };
        Error {
            code: code::INVALID_CONFIG,
            message: message.into(),
            data: serde_json::to_value(data).ok(),
        }
    }

    /// Reports that the plugin does not implement a capability method, which
    /// lets the host fall back to its own implementation.
    pub fn unsupported(method: &str) -> Self {
        Error::new(
            code::UNSUPPORTED,
            format!("unsupported method: {method}"),
            None,
        )
    }

    /// Reports params that could not be decoded.
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Error::new(code::INVALID_PARAMS, message, None)
    }

    /// Reports an unexpected failure.
    pub fn internal(message: impl fmt::Display) -> Self {
        Error::new(code::INTERNAL_ERROR, message.to_string(), None)
    }

    /// Reports a method nobody handles.
    pub fn method_not_found(method: &str) -> Self {
        Error::new(
            code::METHOD_NOT_FOUND,
            format!("unknown method: {method}"),
            None,
        )
    }

    /// Returns `data.field` of an invalid config error.
    pub fn field(&self) -> Option<&str> {
        self.data.as_ref()?.get("field")?.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::internal(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::internal(e)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::internal(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Error::internal(message)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        match e.downcast::<Error>() {
            Ok(e) => *e,
            Err(e) => Error::internal(e),
        }
    }
}

/// The conventional shape of [`Error::data`] for
/// [`code::INVALID_CONFIG`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InvalidConfigData {
    /// Name of the offending credential or setting field.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub field: String,
}
