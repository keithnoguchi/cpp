//! async-task based executor example
//!
//! As in the [async-task] crate documentation.
//!
//! [async-task]: https://lib.rs/async-task
#![forbid(unsafe_code, missing_docs, missing_debug_implementations)]
use std::sync::Arc;

use concurrent_queue::ConcurrentQueue;
use tracing::{debug, instrument};

fn main() {
    tracing_subscriber::fmt::init();

    let queue = Arc::new(ConcurrentQueue::unbounded());
    let sender = queue.clone();

    // scheduler function.
    let schedule = move |runnable| sender.push(runnable).unwrap();

    let (runnable, _task) = async_task::spawn(additioner(1, 2), schedule);

    let result = runnable.schedule();
    debug!("{result:?}");

    while let Ok(runnable) = queue.pop() {
        let result = runnable.run();
        debug!("{result}");
    }
}

#[instrument(ret)]
async fn additioner(x: usize, y: usize) -> usize {
    x + y
}
