//! 5.3.1 Future by async/await
mod hello;

pub use crate::hello::Hello;
use futures::future::BoxFuture;
use std::error::Error;
use std::future::Future;
use std::result;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub struct Executor<T> {
    tx: SyncSender<Arc<Task<T>>>,
    _rx: Receiver<Arc<Task<T>>>,
}

impl<T> Executor<T> {
    pub fn new(nr_run_queue_bound: usize) -> Self {
        let (tx, _rx) = sync_channel(nr_run_queue_bound);
        Self { tx, _rx }
    }

    pub fn spawner(&self) -> Spawner<T> {
        let tx = self.tx.clone();
        Spawner { tx }
    }

    pub fn run(self) -> Result<()> {
        Ok(())
    }
}

pub struct Spawner<T: 'static> {
    tx: SyncSender<Arc<Task<T>>>,
}

impl<T: 'static> Spawner<T> {
    pub fn spawn<F>(&self, fut: F) -> Result<()>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let task = Arc::new(Task {
            _fut: Mutex::new(Box::pin(fut)),
            _tx: self.tx.clone(),
        });
        self.tx.send(task)?;
        Ok(())
    }
}

struct Task<T> {
    _fut: Mutex<BoxFuture<'static, T>>,
    _tx: SyncSender<Arc<Self>>,
}
