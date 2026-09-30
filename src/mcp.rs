//! The `mcp` capability: Model Context Protocol tools.

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::Registry;
use crate::jsonrpc::BoxFuture;
use crate::protocol::{method, Error, McpCallParams, McpCallResult, McpContent, Settings};

/// The payload of `mcp.call`.
pub type McpRequest = McpCallParams;

/// The reply to `mcp.call`.
pub type McpResult = McpCallResult;

/// Runs the Model Context Protocol tools the manifest declares in its `mcp`
/// block. The host publishes them under a prefixed name and forwards every
/// call with the unprefixed name in `req.tool`.
#[async_trait]
pub trait McpHandler: Send + Sync + 'static {
    /// Runs one tool. A tool that ran and failed returns
    /// [`McpCallResult::error`], an unknown tool [`unknown_tool`].
    /// `req.arguments` come from an AI assistant and must be validated before
    /// use.
    async fn call(&self, ctx: &Context, req: McpRequest) -> Result<McpResult, Error>;
}

type ToolFn = Arc<dyn Fn(Context, Settings) -> BoxFuture<Result<McpResult, Error>> + Send + Sync>;

/// An [`McpHandler`] that dispatches a call by tool name.
///
/// ```
/// use nginxui_plugin_sdk::{McpResult, McpTools, Plugin};
///
/// let tools = McpTools::new().tool("purge_cache", |_ctx, args| async move {
///     match args.get("zone").and_then(|z| z.as_str()) {
///         Some(zone) => Ok(McpResult::text(format!("purged {zone}"))),
///         None => Ok(McpResult::error("zone is required")),
///     }
/// });
/// let plugin = Plugin::new().mcp(tools);
/// # drop(plugin);
/// ```
#[derive(Clone, Default)]
pub struct McpTools {
    tools: HashMap<String, ToolFn>,
}

impl std::fmt::Debug for McpTools {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut names: Vec<&String> = self.tools.keys().collect();
        names.sort();
        f.debug_struct("McpTools").field("tools", &names).finish()
    }
}

impl McpTools {
    /// Builds an empty tool set.
    pub fn new() -> McpTools {
        McpTools::default()
    }

    /// Adds a tool. It gets the arguments object of the call, empty when the
    /// call has none.
    #[must_use]
    pub fn tool<F, Fut>(mut self, name: impl Into<String>, f: F) -> McpTools
    where
        F: Fn(Context, Settings) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<McpResult, Error>> + Send + 'static,
    {
        self.tools.insert(
            name.into(),
            Arc::new(move |ctx, args| Box::pin(f(ctx, args))),
        );
        self
    }
}

#[async_trait]
impl McpHandler for McpTools {
    async fn call(&self, ctx: &Context, req: McpRequest) -> Result<McpResult, Error> {
        match self.tools.get(&req.tool) {
            Some(tool) => tool(ctx.clone(), req.arguments).await,
            None => Err(unknown_tool(&req.tool)),
        }
    }
}

impl McpCallResult {
    /// A successful tool result with one text block.
    pub fn text(text: impl Into<String>) -> Self {
        McpCallResult {
            content: vec![McpContent::text(text)],
            is_error: false,
        }
    }

    /// The result of a tool that ran and failed. The text tells the AI
    /// assistant what went wrong so it can correct itself.
    pub fn error(text: impl Into<String>) -> Self {
        McpCallResult {
            content: vec![McpContent::text(text)],
            is_error: true,
        }
    }
}

/// Reports a tool the plugin does not serve, with the -32602 code the Model
/// Context Protocol uses for it.
pub fn unknown_tool(name: &str) -> Error {
    Error::invalid_params(format!("unknown tool: {name}"))
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: McpHandler + ?Sized> McpHandler for Arc<T> {
    async fn call(&self, ctx: &Context, req: McpRequest) -> Result<McpResult, Error> {
        (**self).call(ctx, req).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn McpHandler>) {
    reg.tracked(method::MCP_CALL, move |ctx, req: McpRequest| {
        let h = h.clone();
        async move { h.call(&ctx, req).await }
    });
}
