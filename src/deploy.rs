//! The `cert.deploy` capability: pushing certificates to external targets.

use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::dispatch::{empty, Registry};
use crate::protocol::{
    method, Config, DeployCertificate, DeployPushParams, DeployPushResult, DeployValidateParams,
    Error,
};

/// The payload of `deploy.push`.
pub type DeployRequest = DeployPushParams;

/// Pushes certificates to the kinds of target the manifest declares in its
/// `deploy` block. `req.kind` is the target kind code and `req.config` holds
/// the values of its form. The manifest must request the `cert.deploy`
/// permission, since the request carries the private key.
#[async_trait]
pub trait DeployHandler: Send + Sync + 'static {
    /// Pushes `req.certificate` to the target and returns a short summary of
    /// what it did.
    ///
    /// When `req.dry_run` is true it must not change anything: it checks what
    /// a real push needs and says what it would do. Pushing a certificate the
    /// target already serves must succeed. Return [`Error::invalid_config`]
    /// for a bad field and any other error for a target failure. Never put
    /// the private key in an error or a log line.
    async fn push(&self, ctx: &Context, req: DeployRequest) -> Result<String, Error>;

    /// Checks a target configuration without contacting the target. The
    /// default answers `-32002` (unsupported).
    async fn validate(&self, ctx: &Context, kind: &str, config: &Config) -> Result<(), Error> {
        let _ = (ctx, kind, config);
        Err(Error::unsupported(method::DEPLOY_VALIDATE))
    }
}

/// Returns the leaf followed by its intermediates, for a target that wants
/// the whole chain in one piece.
pub fn full_chain_pem(cert: &DeployCertificate) -> String {
    let mut leaf = cert.certificate_pem.clone();
    if cert.chain_pem.is_empty() {
        return leaf;
    }
    if !leaf.is_empty() && !leaf.ends_with('\n') {
        leaf.push('\n');
    }
    leaf.push_str(&cert.chain_pem);
    leaf
}

/// A shared handler serves as well as an owned one.
#[async_trait]
impl<T: DeployHandler + ?Sized> DeployHandler for Arc<T> {
    async fn push(&self, ctx: &Context, req: DeployRequest) -> Result<String, Error> {
        (**self).push(ctx, req).await
    }

    async fn validate(&self, ctx: &Context, kind: &str, config: &Config) -> Result<(), Error> {
        (**self).validate(ctx, kind, config).await
    }
}

pub(crate) fn register(reg: &mut Registry, h: Arc<dyn DeployHandler>) {
    let handler = h.clone();
    reg.tracked(method::DEPLOY_PUSH, move |ctx, req: DeployRequest| {
        let h = handler.clone();
        async move {
            let message = h.push(&ctx, req).await?;
            Ok(DeployPushResult { message })
        }
    });
    let handler = h;
    reg.tracked(
        method::DEPLOY_VALIDATE,
        move |ctx, params: DeployValidateParams| {
            let h = handler.clone();
            async move {
                h.validate(&ctx, &params.kind, &params.config).await?;
                empty()
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_leaf_and_chain() {
        let leaf_only = DeployCertificate {
            certificate_pem: "LEAF".into(),
            ..Default::default()
        };
        assert_eq!(full_chain_pem(&leaf_only), "LEAF");

        let with_chain = DeployCertificate {
            certificate_pem: "LEAF\n".into(),
            chain_pem: "ISSUER\n".into(),
            ..Default::default()
        };
        assert_eq!(full_chain_pem(&with_chain), "LEAF\nISSUER\n");

        let unterminated = DeployCertificate {
            certificate_pem: "LEAF".into(),
            chain_pem: "ISSUER\n".into(),
            ..Default::default()
        };
        assert_eq!(full_chain_pem(&unterminated), "LEAF\nISSUER\n");
    }
}
