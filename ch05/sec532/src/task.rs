//! 5.3.2 I/O Selector with epoll(7)
use crate::Result;
use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::Context;

pub struct Executor<T: Send + 'static> {
    tx: SyncSender<Arc<Task<T>>>,
    rx: Receiver<Arc<Task<T>>>,
}

#[derive(Clone)]
pub struct Spawner<T: Send + 'static> {
    tx: SyncSender<Arc<Task<T>>>,
}

pub struct Task<T: Send + 'static> {
    fut: Mutex<BoxFuture<'static, T>>,
    tx: SyncSender<Arc<Self>>,
}

impl<T: Send + 'static> Executor<T> {
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
            if fut.as_mut().poll(&mut ctx).is_pending() {
                println!("task is pending...");
            }
        }
        Ok(())
    }
}

impl<T: Send + 'static> Spawner<T> {
    pub fn spawn<F>(&self, fut: F) -> Result<()>
    where
        F: Future<Output = T> + Send + 'static,
    {
        let task = Arc::new(Task {
            fut: Mutex::new(Box::pin(fut)),
            tx: self.tx.clone(),
        });
        self.tx
            .send(task)
            .map_err(|e| format!("spawn error: {e}"))?;
        Ok(())
    }
}

impl<T: Send + 'static> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let task = arc_self.clone();
        arc_self.tx.send(task).unwrap();
    }
}
