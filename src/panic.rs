//! Turns a panic in a handler into an error instead of a dead task.

use std::any::Any;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Returns the message of a panic payload.
pub(crate) fn panic_message(panic: &(dyn Any + Send)) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        (*s).to_owned()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".to_owned()
    }
}

/// Turns a panic while polling a future into an `Err`.
pub(crate) struct CatchUnwind<F>(pub(crate) F);

impl<F: Future + Unpin> Future for CatchUnwind<F> {
    type Output = Result<F::Output, Box<dyn Any + Send>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = Pin::new(&mut self.0);
        match std::panic::catch_unwind(AssertUnwindSafe(|| inner.poll(cx))) {
            Ok(Poll::Pending) => Poll::Pending,
            Ok(Poll::Ready(v)) => Poll::Ready(Ok(v)),
            Err(panic) => Poll::Ready(Err(panic)),
        }
    }
}

/// Awaits `fut` and returns the message of a panic as the error.
pub(crate) async fn catch_unwind<F: Future + Unpin>(fut: F) -> Result<F::Output, String> {
    CatchUnwind(fut)
        .await
        .map_err(|panic| panic_message(&*panic))
}
