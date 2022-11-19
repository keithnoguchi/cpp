//! 5.3.2 I/O Selector with epoll(7)
mod task;

pub use crate::task::Executor;
use std::error::Error;
use std::result;
use std::sync::Arc;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub struct Selector {}

impl Selector {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}
