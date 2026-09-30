//! The `dns01` capability: solving the DNS-01 challenge for one or more
//! vendors.

use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::{empty, Registry};
use crate::protocol::{
    method, Config, Dns01ChallengeParams, Dns01CheckParams, Dns01CheckResult, Dns01OptionsParams,
    Dns01OptionsResult, Dns01ValidateParams, Error,
};

/// The payload of `dns01.present` and `dns01.cleanup`.
pub type Dns01Request = Dns01ChallengeParams;

/// Solves the DNS-01 challenge for one or more vendors.
///
/// `validate`, `options` and `check` are optional. Their defaults answer
/// `-32002` (unsupported), which the host takes as "use my own code".
#[async_trait]
pub trait Dns01Handler: Send + Sync + 'static {
    /// Publishes the TXT record for the challenge.
    async fn present(&self, ctx: &Context, req: Dns01Request) -> Result<(), Error>;

    /// Removes the record `present` published.
    async fn clean_up(&self, ctx: &Context, req: Dns01Request) -> Result<(), Error>;

    /// Checks credentials without touching a certificate. Return
    /// [`Error::invalid_config`] for a bad field.
    async fn validate(&self, ctx: &Context, provider: &str, config: &Config) -> Result<(), Error> {
        let _ = (ctx, provider, config);
        Err(Error::unsupported(method::DNS01_VALIDATE))
    }

    /// Reports propagation timings. The host falls back to its own defaults
    /// when this is not implemented.
    async fn options(
        &self,
        ctx: &Context,
        params: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        let _ = (ctx, params);
        Err(Error::unsupported(method::DNS01_OPTIONS))
    }

    /// Runs the propagation check of the plugin itself.
    async fn check(
        &self,
        ctx: &Context,
        params: Dns01CheckParams,
    ) -> Result<Dns01CheckResult, Error> {
        let _ = (ctx, params);
        Err(Error::unsupported(method::DNS01_CHECK))
    }
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: Dns01Handler + ?Sized> Dns01Handler for Arc<T> {
    async fn present(&self, ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        (**self).present(ctx, req).await
    }

    async fn clean_up(&self, ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        (**self).clean_up(ctx, req).await
    }

    async fn validate(&self, ctx: &Context, provider: &str, config: &Config) -> Result<(), Error> {
        (**self).validate(ctx, provider, config).await
    }

    async fn options(
        &self,
        ctx: &Context,
        params: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        (**self).options(ctx, params).await
    }

    async fn check(
        &self,
        ctx: &Context,
        params: Dns01CheckParams,
    ) -> Result<Dns01CheckResult, Error> {
        (**self).check(ctx, params).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn Dns01Handler>) {
    let handler = h.clone();
    reg.tracked(method::DNS01_PRESENT, move |ctx, req: Dns01Request| {
        let h = handler.clone();
        async move {
            h.present(&ctx, req).await?;
            empty()
        }
    });
    let handler = h.clone();
    reg.tracked(method::DNS01_CLEANUP, move |ctx, req: Dns01Request| {
        let h = handler.clone();
        async move {
            h.clean_up(&ctx, req).await?;
            empty()
        }
    });
    let handler = h.clone();
    reg.tracked(
        method::DNS01_VALIDATE,
        move |ctx, params: Dns01ValidateParams| {
            let h = handler.clone();
            async move {
                h.validate(&ctx, &params.provider, &params.config).await?;
                empty()
            }
        },
    );
    let handler = h.clone();
    reg.tracked(
        method::DNS01_OPTIONS,
        move |ctx, params: Dns01OptionsParams| {
            let h = handler.clone();
            async move { h.options(&ctx, params).await }
        },
    );
    let handler = h;
    reg.tracked(method::DNS01_CHECK, move |ctx, params: Dns01CheckParams| {
        let h = handler.clone();
        async move { h.check(&ctx, params).await }
    });
}
