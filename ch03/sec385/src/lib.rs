//! 3.8.5 Semaphore and Channel
mod channel;
mod semaphore;

pub use channel::{channel, Receiver, Sender};
pub use semaphore::Semaphore;

use std::error::Error;
use std::result;
use std::str::FromStr;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

static CNT: AtomicIsize = AtomicIsize::new(0);

pub fn worker(id: u64, jobs: usize, sem: Arc<Semaphore>) -> Result<u64> {
    (0..jobs).for_each(|_| {
        sem.wait();
        CNT.fetch_add(1, Ordering::SeqCst);
        let cnt = CNT.load(Ordering::SeqCst);
        CNT.fetch_sub(1, Ordering::SeqCst);
        sem.post();
        assert!(cnt <= sem.max());
        assert!(cnt > 0);
    });
    Ok(id)
}

pub fn producer<T: Send + Clone>(id: u64, data: T, n: usize, tx: Sender<T>) -> Result<u64> {
    (0..n).for_each(|_| tx.send(data.clone()));
    Ok(id)
}

// I tried T: FromStr but that was not enough to bound the type.
// Instead, set the type to String to work around atm.
pub fn consumer(id: u64, n: usize, rx: Receiver<String>) -> Result<u64> {
    (0..n).for_each(|_| {
        let data = rx.recv();
        if let Err(e) = usize::from_str(&data) {
            panic!("{e}");
        }
    });
    Ok(id)
}
