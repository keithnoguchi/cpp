//! 5.2.2 Async Task Executor
use crate::Result;
use futures::future::{BoxFuture, FutureExt};
use futures::task::ArcWake;
use std::future::Future;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

pub struct Task<T> {
    pub(crate) future: Mutex<BoxFuture<'static, T>>,
    tx: SyncSender<Arc<Self>>,
}

pub struct Spawner<T: 'static> {
    tx: SyncSender<Arc<Task<T>>>,
}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        // This will crash in case of the run queue full.
        arc_self.tx.send(self0).unwrap();
    }
}

impl<T: 'static> Spawner<T> {
    pub fn new(tx: SyncSender<Arc<Task<T>>>) -> Self {
        Self { tx }
    }

    pub fn spawn<F>(&self, f: F) -> Result<()>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(f.boxed()),
            tx: self.tx.clone(),
        });
        self.tx.send(task)?;
        Ok(())
    }
}
