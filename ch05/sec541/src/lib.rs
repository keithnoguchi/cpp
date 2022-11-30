//! 5.4.1 Sync and Async Mutex with tokio
use std::error::Error;
use std::result;
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::sleep;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub async fn sync_locker(counter: Arc<SyncMutex<usize>>, nr_loop: usize) -> Result<()> {
    (0..nr_loop).for_each(|_| {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
    });
    Ok(())
}

pub async fn async_locker(counter: Arc<AsyncMutex<usize>>, nr_loop: usize) -> Result<()> {
    for _ in 0..nr_loop {
        let mut counter = counter.lock().await;
        *counter += 1;
    }
    Ok(())
}

pub async fn async_sleeper(
    counter: Arc<AsyncMutex<usize>>,
    nr_loop: usize,
    nr_sleep: Duration,
) -> Result<()> {
    for _ in 0..nr_loop {
        let mut counter = counter.lock().await;
        *counter += 1;
        sleep(nr_sleep).await;
    }
    Ok(())
}
