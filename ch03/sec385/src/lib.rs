//! 3.8.5 Semaphore and Channel
mod semaphore;

pub use semaphore::Semaphore;

use std::error::Error;
use std::result;
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
