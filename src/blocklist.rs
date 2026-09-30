//! The `security.blocklist` capability: lists of addresses to deny.

use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::Registry;
use crate::protocol::{method, BlocklistEntry, BlocklistFetchParams, BlocklistFetchResult, Error};

/// The payload of `blocklist.fetch`.
pub type BlocklistRequest = BlocklistFetchParams;

/// The reply to `blocklist.fetch`.
pub type BlocklistResult = BlocklistFetchResult;

/// Fetches lists of addresses to deny from the kinds of source the manifest
/// declares in its `blocklist` block. `req.source` is the source kind code
/// and `req.config` holds the values of its form. The manifest must request
/// the `network` permission.
#[async_trait]
pub trait BlocklistHandler: Send + Sync + 'static {
    /// Returns the complete current list of the source, not the changes since
    /// the last call.
    ///
    /// Each entry is an IPv4 or IPv6 address or a CIDR network, the host drops
    /// anything else. An empty list denies nothing, so when the list cannot be
    /// fetched return an error instead: [`Error::invalid_config`] for a bad
    /// field, any other error for a source failure. The host then keeps the
    /// list it has.
    async fn fetch(&self, ctx: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error>;
}

impl BlocklistEntry {
    /// Builds an entry for an address or a CIDR network.
    pub fn deny(cidr: impl Into<String>, reason: impl Into<String>) -> Self {
        BlocklistEntry {
            cidr: cidr.into(),
            reason: reason.into(),
        }
    }
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: BlocklistHandler + ?Sized> BlocklistHandler for Arc<T> {
    async fn fetch(&self, ctx: &Context, req: BlocklistRequest) -> Result<BlocklistResult, Error> {
        (**self).fetch(ctx, req).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn BlocklistHandler>) {
    reg.tracked(
        method::BLOCKLIST_FETCH,
        move |ctx, req: BlocklistRequest| {
            let h = h.clone();
            async move { h.fetch(&ctx, req).await }
        },
    );
}
