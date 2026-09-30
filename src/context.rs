use std::time::{Duration, Instant};

use tokio::sync::watch;
use tokio::time::Instant as TokioInstant;

use crate::host::{current_host, Host};

/// The context of one call a handler serves.
///
/// It carries the [`Host`] client, the deadline of the call and a signal that
/// the plugin is stopping. It is cheap to clone.
///
/// Rust futures are cancelled by dropping them, so a handler does not have to
/// watch the context to stop: when the caller gives up or the connection ends
/// the future is dropped. Use [`Context::cancelled`] to release resources in
/// an orderly way, or [`Context::remaining`] to bound work on the deadline.
#[derive(Clone, Default)]
pub struct Context {
    host: Option<Host>,
    deadline: Option<TokioInstant>,
    stopping: Option<watch::Receiver<bool>>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("deadline", &self.deadline())
            .finish_non_exhaustive()
    }
}

impl Context {
    /// Builds a context that has no host of its own, no deadline and is
    /// never cancelled. [`Context::host`] falls back to [`current_host`].
    pub fn background() -> Context {
        Context::default()
    }

    pub(crate) fn new(host: Host, stopping: watch::Receiver<bool>) -> Context {
        Context {
            host: Some(host),
            deadline: None,
            stopping: Some(stopping),
        }
    }

    /// Returns the host client.
    ///
    /// It falls back to the process wide client, so a handler always gets a
    /// usable value. Outside a running plugin its calls fail with
    /// [`HostError::NotReady`](crate::HostError::NotReady).
    pub fn host(&self) -> Host {
        self.host
            .clone()
            .or_else(current_host)
            .unwrap_or_else(Host::detached)
    }

    /// Returns a copy that also expires after `timeout`, or at the current
    /// deadline when that is sooner.
    #[must_use]
    pub fn with_timeout(&self, timeout: Duration) -> Context {
        let candidate = TokioInstant::now() + timeout;
        let mut ctx = self.clone();
        ctx.deadline = Some(match ctx.deadline {
            Some(existing) => existing.min(candidate),
            None => candidate,
        });
        ctx
    }

    /// Returns a copy without the deadline of the call, for work that has to
    /// outlive it, such as clearing state after the call gave up.
    #[must_use]
    pub fn detached(&self) -> Context {
        let mut ctx = self.clone();
        ctx.deadline = None;
        ctx
    }

    /// The moment the call expires, if it has a deadline.
    pub fn deadline(&self) -> Option<Instant> {
        self.deadline.map(TokioInstant::into_std)
    }

    /// The time left until the deadline, if it has one. Zero once passed.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|d| d.saturating_duration_since(TokioInstant::now()))
    }

    /// Reports whether the plugin is stopping or the deadline passed.
    pub fn is_cancelled(&self) -> bool {
        if self.remaining() == Some(Duration::ZERO) {
            return true;
        }
        self.stopping.as_ref().is_some_and(|rx| *rx.borrow())
    }

    /// Completes when the plugin is stopping or the deadline passes. It
    /// stays pending forever for a background context.
    pub async fn cancelled(&self) {
        let stopping = async {
            match self.stopping.clone() {
                Some(mut rx) => {
                    let _ = rx.wait_for(|stop| *stop).await;
                }
                None => std::future::pending::<()>().await,
            }
        };
        let expired = async {
            match self.deadline {
                Some(deadline) => tokio::time::sleep_until(deadline).await,
                None => std::future::pending::<()>().await,
            }
        };
        tokio::select! {
            () = stopping => {}
            () = expired => {}
        }
    }
}
