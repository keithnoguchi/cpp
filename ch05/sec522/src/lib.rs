//! 5.2.2 Async Task Executor
mod task;

use crate::task::{Spawner, Task};
use std::error::Error;
use std::result;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

pub struct Executor<T: 'static> {
    tx: SyncSender<Arc<Task<T>>>,
    rx: Receiver<Arc<Task<T>>>,
}

type Result<T> = result::Result<T, Box<dyn Error>>;

impl<T: 'static> Executor<T> {
    pub fn new(nr_run_queue: usize) -> Self {
        let (tx, rx) = sync_channel(nr_run_queue);
        Self { tx, rx }
    }

    pub fn spawner(&self) -> Spawner<T> {
        Spawner::new(self.tx.clone())
    }

    pub fn run(&self) -> Result<()> {
        loop {
            let task = self.rx.recv()?;
            // XXX This unwrap() will be removed soon;
            let mut _future = task.future.lock().unwrap();
        }
    }
}
