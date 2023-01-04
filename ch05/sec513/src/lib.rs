//! `atomic-waker` example
#![forbid(unsafe_code, missing_docs, missing_debug_implementations)]
extern crate alloc;

use alloc::sync::Arc;

use core::future::Future;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};
use core::pin::Pin;
use core::task::{Context, Poll};

use atomic_waker::AtomicWaker;

/// Flag is an asynchronous flag protected by the `AtomicWaker`,
/// demonstrated in the `atomic-waker` crate example.
#[derive(Debug, Clone)]
pub struct Flag(Arc<Inner>);

#[derive(Debug)]
struct Inner {
    waker: AtomicWaker,
    set: AtomicBool,
}

impl Default for Flag {
    fn default() -> Self {
        Self(Arc::new(Inner {
            waker: AtomicWaker::new(),
            set: AtomicBool::default(),
        }))
    }
}

impl Future for Flag {
    type Output = ();

    /// Check the waker status, in case if the flag is not set,
    /// it registers the waker.  This ensures the single waker
    /// is registered for this particular event.
    ///
    /// This ensures a single call to the waker by the consumer,
    /// e.g. async runtime reacter.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // quick check to avoid registration if it's already done.
        if self.0.set.load(Relaxed) {
            return Poll::Ready(());
        }

        self.0.waker.register(cx.waker());

        if self.0.set.load(Relaxed) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Flag {
    /// Create a new asynchronous flag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Signals the change of the flag through the `Waker`.
    pub fn signal(&self) {
        self.0.set.store(true, Relaxed);
        self.0.waker.wake();
    }
}
