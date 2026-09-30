//! The `notify` capability: delivering notifications through the channels
//! the manifest declares.

use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::{empty, Registry};
use crate::protocol::{method, Config, Error, NotifySendParams, NotifyValidateParams};

/// The payload of `notify.send`.
pub type NotifyRequest = NotifySendParams;

/// Delivers notifications through the channels the manifest declares in its
/// `notify` block.
#[async_trait]
pub trait NotifyHandler: Send + Sync + 'static {
    /// Delivers one notification. `req.channel` is the channel code and
    /// `req.config` holds the values of its form. Return
    /// [`Error::invalid_config`] for a bad field and any other error for a
    /// vendor failure.
    async fn send(&self, ctx: &Context, req: NotifyRequest) -> Result<(), Error>;

    /// Checks a channel configuration without sending anything. The default
    /// answers `-32002` (unsupported).
    async fn validate(&self, ctx: &Context, channel: &str, config: &Config) -> Result<(), Error> {
        let _ = (ctx, channel, config);
        Err(Error::unsupported(method::NOTIFY_VALIDATE))
    }
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: NotifyHandler + ?Sized> NotifyHandler for Arc<T> {
    async fn send(&self, ctx: &Context, req: NotifyRequest) -> Result<(), Error> {
        (**self).send(ctx, req).await
    }

    async fn validate(&self, ctx: &Context, channel: &str, config: &Config) -> Result<(), Error> {
        (**self).validate(ctx, channel, config).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn NotifyHandler>) {
    let handler = h.clone();
    reg.tracked(method::NOTIFY_SEND, move |ctx, req: NotifyRequest| {
        let h = handler.clone();
        async move {
            h.send(&ctx, req).await?;
            empty()
        }
    });
    let handler = h;
    reg.tracked(
        method::NOTIFY_VALIDATE,
        move |ctx, params: NotifyValidateParams| {
            let h = handler.clone();
            async move {
                h.validate(&ctx, &params.channel, &params.config).await?;
                empty()
            }
        },
    );
}
