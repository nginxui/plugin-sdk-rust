//! The smallest useful plugin: a DNS-01 provider that publishes nothing.
//!
//! Run it under the host, or drive it by hand:
//!
//! ```sh
//! cargo build --example dns01
//! echo '{"jsonrpc":"2.0","id":1,"method":"plugin.initialize","params":{}}' | target/debug/examples/dns01
//! ```

use nginxui_plugin_sdk::protocol::{Dns01OptionsParams, Dns01OptionsResult};
use nginxui_plugin_sdk::{redact, Context, Dns01Handler, Dns01Request, Error, Plugin};

struct Provider;

#[async_trait::async_trait]
impl Dns01Handler for Provider {
    // Publishes the challenge TXT record.
    async fn present(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        let token = req.config.get("MY_API_TOKEN").cloned().unwrap_or_default();
        if token.is_empty() {
            return Err(Error::invalid_config(
                "MY_API_TOKEN",
                "the API token is required",
            ));
        }

        nginxui_plugin_sdk::info!(
            "publishing {} for {} (token {})",
            req.effective_fqdn,
            req.domain,
            redact(&token)
        );

        // Talk to the vendor API here.
        Ok(())
    }

    // Removes what `present` published.
    async fn clean_up(&self, _ctx: &Context, req: Dns01Request) -> Result<(), Error> {
        nginxui_plugin_sdk::info!("removing {}", req.effective_fqdn);
        Ok(())
    }

    // Optional: report the propagation timings to the host.
    async fn options(
        &self,
        _ctx: &Context,
        _params: Dns01OptionsParams,
    ) -> Result<Dns01OptionsResult, Error> {
        Ok(Dns01OptionsResult {
            propagation_timeout_seconds: 120,
            polling_interval_seconds: 2,
            ..Default::default()
        })
    }
}

fn main() {
    let plugin = Plugin::new()
        .dns01(Provider)
        // Called on plugin.configure and whenever the user saves settings.
        .configure(|ctx, settings| async move {
            ctx.host()
                .notify(
                    "info",
                    "Reconfigured",
                    &format!("{} settings", settings.len()),
                    (),
                )
                .await?;
            Ok(())
        });

    nginxui_plugin_sdk::serve_blocking(plugin);
}
