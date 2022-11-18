//! 5.3.1 Future by async/await
mod hello;

pub use crate::hello::Hello;
use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use std::error::Error;
use std::future::Future;
use std::result;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::Context;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub struct Executor<T> {
    tx: SyncSender<Arc<Task<T>>>,
    rx: Receiver<Arc<Task<T>>>,
}

impl<T> Executor<T> {
    pub fn new(nr_run_queue_bound: usize) -> Self {
        let (tx, rx) = sync_channel(nr_run_queue_bound);
        Self { tx, rx }
    }

    pub fn spawner(&self) -> Spawner<T> {
        let tx = self.tx.clone();
        Spawner { tx }
    }

    pub fn run(self) -> Result<()> {
        drop(self.tx);
        while let Ok(task) = self.rx.recv() {
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            let mut fut = task.fut.lock().unwrap();
            // deadlock in case of the run queue full.
            fut.as_mut()
                .poll(&mut ctx)
                .is_pending()
                .then(|| waker.wake_by_ref());
        }
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
            fut: Mutex::new(Box::pin(fut)),
            tx: self.tx.clone(),
        });
        self.tx.send(task)?;
        Ok(())
    }
}

struct Task<T> {
    fut: Mutex<BoxFuture<'static, T>>,
    tx: SyncSender<Arc<Self>>,
}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let task = arc_self.clone();
        // this blocks forever in case of run queue full,
        // or crash in case the channel is dropped.
        arc_self.tx.send(task).unwrap();
    }
}
