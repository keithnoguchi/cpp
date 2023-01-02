//! Let's write an executor.
//!
//! This is a copy of the simple executor demonstrated in `smol` crate.
use std::future::Future;
use std::panic;
use std::thread;

use futures_lite::future::pending;
use tracing::{debug, instrument};

use async_executor::{Executor, Task};
use async_io::block_on;
use async_lock::OnceCell;

#[instrument(skip(future))]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    static GLOBAL: OnceCell<Executor<'_>> = OnceCell::new();

    // Mutable static variable in action!
    fn global() -> &'static Executor<'static> {
        GLOBAL.get_or_init_blocking(|| {
            debug!("global executor");
            let num_workers = 4; // just for fun

            for n in 1..=num_workers {
                thread::Builder::new()
                    .name(format!("sec512-{n}"))
                    .spawn(move || loop {
                        debug!(%n, "in executor thread");
                        panic::catch_unwind(|| block_on(global().run(pending::<()>()))).ok();
                    })
                    .expect("cannot spawn executor thread");
            }

            Executor::new()
        })
    }

    global().spawn(future)
}
