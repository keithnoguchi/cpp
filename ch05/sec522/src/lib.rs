//! 5.2.2 Async Task Executor
mod hello;
mod task;

pub use crate::hello::Hello;
use crate::task::{Spawner, Task};
use futures::task::waker_ref;
use std::error::Error;
use std::fmt::Display;
use std::result;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct Executor<T: 'static> {
    tx: SyncSender<Arc<Task<T>>>,
    rx: Receiver<Arc<Task<T>>>,
}

type Result<T> = result::Result<T, Box<dyn Error>>;

impl<T: 'static + Display> Executor<T> {
    pub fn new(nr_run_queue_bound: usize) -> Self {
        let (tx, rx) = sync_channel(nr_run_queue_bound);
        Self { tx, rx }
    }

    pub fn spawner(&self) -> Spawner<T> {
        Spawner::new(self.tx.clone())
    }

    pub fn run(self) -> Result<()> {
        // This tx drop makes the completion of the executor
        // once all the tasks are reach to completion.
        drop(self.tx);
        while let Ok(task) = self.rx.recv() {
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            let mut fut = task.future.lock().map_err(|e| format!("{e:?}"))?;

            // Run the coroutine/future.
            //
            // Note that this will block in case the run queue is already
            // full.
            match fut.as_mut().poll(&mut ctx) {
                Poll::Pending => waker.wake_by_ref(),
                Poll::Ready(result) => println!("{result}"),
            }
        }
        Ok(())
    }
}
