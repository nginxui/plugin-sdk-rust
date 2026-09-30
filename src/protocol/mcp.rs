use serde::{Deserialize, Serialize};

use super::{is_false, Settings};

/// The payload of `mcp.call`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct McpCallParams {
    /// Tool name declared in the manifest, without the host prefix.
    pub tool: String,
    /// The arguments object the MCP client sent.
    #[serde(skip_serializing_if = "Settings::is_empty")]
    pub arguments: Settings,
}

/// The reply to `mcp.call`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct McpCallResult {
    /// Content blocks of the result.
    pub content: Vec<McpContent>,
    /// The tool ran and failed, `content` explains why.
    #[serde(skip_serializing_if = "is_false")]
    pub is_error: bool,
}

/// One content block of a tool result.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct McpContent {
    /// [`MCP_CONTENT_TYPE_TEXT`], the only type the contract defines.
    pub r#type: String,
    /// The text of the block.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
}

/// The type of a text content block.
pub const MCP_CONTENT_TYPE_TEXT: &str = "text";

impl McpContent {
    /// Builds a text block.
    pub fn text(text: impl Into<String>) -> Self {
        McpContent {
            r#type: MCP_CONTENT_TYPE_TEXT.to_owned(),
            text: text.into(),
        }
    }
}
