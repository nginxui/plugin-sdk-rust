use crate::logger;
use crate::options::Options;
use crate::plugin::Plugin;
use crate::runtime::run_until;

/// Runs the plugin on stdin and stdout and never returns: it exits the
/// process once the host asked it to stop, stdin ended or a signal arrived.
///
/// It registers the lifecycle methods, wires the capability methods the
/// handlers implement and exits with status 0, or 1 when the connection
/// failed.
pub async fn serve(plugin: Plugin) -> ! {
    serve_with(plugin, Options::default()).await
}

/// Like [`serve`], with options.
pub async fn serve_with(plugin: Plugin, options: Options) -> ! {
    let signals = shutdown_signal();
    let result = run_until(
        plugin,
        tokio::io::stdin(),
        tokio::io::stdout(),
        options,
        signals,
    )
    .await;
    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            logger::error(format!("plugin stopped: {e}"));
            std::process::exit(1)
        }
    }
}

/// Builds a tokio runtime and runs [`serve`] on it, so `main` needs no
/// async runtime of its own.
pub fn serve_blocking(plugin: Plugin) -> ! {
    serve_blocking_with(plugin, Options::default())
}

/// Like [`serve_blocking`], with options.
pub fn serve_blocking_with(plugin: Plugin, options: Options) -> ! {
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(e) => {
            logger::error(format!("plugin stopped: cannot build the runtime: {e}"));
            std::process::exit(1);
        }
    };
    runtime.block_on(serve_with(plugin, options))
}

/// Completes on SIGINT or SIGTERM. The handlers are installed when this is
/// called, before the future is polled.
fn shutdown_signal() -> impl std::future::Future<Output = ()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let handlers: std::io::Result<_> = (|| {
            Ok((
                signal(SignalKind::interrupt())?,
                signal(SignalKind::terminate())?,
            ))
        })();
        async move {
            match handlers {
                Ok((mut interrupt, mut terminate)) => {
                    tokio::select! {
                        _ = interrupt.recv() => {}
                        _ = terminate.recv() => {}
                    }
                }
                Err(e) => {
                    logger::warn(format!("cannot listen for signals: {e}"));
                    std::future::pending::<()>().await;
                }
            }
        }
    }
    #[cfg(not(unix))]
    {
        async {
            if let Err(e) = tokio::signal::ctrl_c().await {
                logger::warn(format!("cannot listen for signals: {e}"));
                std::future::pending::<()>().await;
            }
        }
    }
}
