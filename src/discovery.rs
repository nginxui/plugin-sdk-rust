//! The `upstream.discovery` capability: resolving services into servers.

use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::Registry;
use crate::protocol::{
    method, DiscoveryResolveParams, DiscoveryResolveResult, DiscoveryTarget, Error,
};

/// The payload of `discovery.resolve`.
pub type DiscoveryRequest = DiscoveryResolveParams;

/// The reply to `discovery.resolve`.
pub type DiscoveryResult = DiscoveryResolveResult;

/// Resolves services into their servers through the providers the manifest
/// declares in its `discovery` block. `req.provider` is the provider code,
/// `req.config` holds the values of its form and `req.service` names the
/// service. The manifest must request the `network` permission.
#[async_trait]
pub trait DiscoveryHandler: Send + Sync + 'static {
    /// Returns every server that backs the service right now: an IP address
    /// or a host name without a port, a port from 1 to 65535 and a weight from
    /// 0 to 1000, where 0 means 1.
    ///
    /// When the service cannot be resolved return an error, so the host keeps
    /// the servers it has: [`Error::invalid_config`] for a bad field
    /// ([`unknown_service`] for a service the provider does not know), any
    /// other error for a provider failure.
    async fn resolve(&self, ctx: &Context, req: DiscoveryRequest)
        -> Result<DiscoveryResult, Error>;
}

impl DiscoveryTarget {
    /// Builds a target with weight 1 and no tags.
    pub fn new(address: impl Into<String>, port: i32) -> Self {
        DiscoveryTarget {
            address: address.into(),
            port,
            weight: 1,
            tags: Vec::new(),
        }
    }

    /// Sets the tags of the target.
    #[must_use]
    pub fn with_tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = tags.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the weight of the target, 0 to 1000.
    #[must_use]
    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

/// Reports a service the provider does not know. It is an invalid
/// configuration naming the `service` field, since the person typed it.
pub fn unknown_service(service: &str) -> Error {
    Error::invalid_config("service", format!("unknown service: {service}"))
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: DiscoveryHandler + ?Sized> DiscoveryHandler for Arc<T> {
    async fn resolve(
        &self,
        ctx: &Context,
        req: DiscoveryRequest,
    ) -> Result<DiscoveryResult, Error> {
        (**self).resolve(ctx, req).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn DiscoveryHandler>) {
    reg.tracked(
        method::DISCOVERY_RESOLVE,
        move |ctx, req: DiscoveryRequest| {
            let h = h.clone();
            async move { h.resolve(&ctx, req).await }
        },
    );
}
