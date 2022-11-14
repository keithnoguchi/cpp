//! Coroutine/Task with Future Trait
use futures::future::{BoxFuture, FutureExt};
use futures::task::ArcWake;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

pub struct Task<T> {
    pub inner: Mutex<BoxFuture<'static, T>>,
}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

mod hello;

static NR_TASK_ID: AtomicU64 = AtomicU64::new(0);

impl Default for Task<(u64, String)> {
    fn default() -> Self {
        let hello = hello::Hello::new(NR_TASK_ID.fetch_add(1, Relaxed));
        Self {
            inner: Mutex::new(hello.boxed()),
        }
    }
}

impl Task<(u64, String)> {
    pub fn new() -> Self {
        Self::default()
    }
}
