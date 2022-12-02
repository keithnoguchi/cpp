//! 5.4.3 Sync and Async Workers on tokio runtime
use std::error::Error;
use std::result;
use std::sync::Arc;
use tracing::{instrument, trace};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;
type SyncCounter = Arc<std::sync::Mutex<usize>>;
type AsyncCounter = Arc<tokio::sync::Mutex<usize>>;

#[instrument(level = "debug", name = "sync_worker", ret, err)]
pub fn sync_worker(counter: SyncCounter, nr_jobs: usize) -> Result<usize> {
    trace!("start");
    let mut worked = 0;
    loop {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        worked += 1;
        if worked == nr_jobs {
            break;
        }
    }
    trace!("finish");
    Ok(worked)
}

#[instrument(level = "debug", name = "async_worker", ret, err)]
pub async fn async_worker(counter: AsyncCounter, nr_jobs: usize) -> Result<usize> {
    trace!("start");
    let mut worked = 0;
    loop {
        let mut counter = counter.lock().await;
        *counter += 1;
        worked += 1;
        if worked == nr_jobs {
            break;
        }
    }
    trace!("finish");
    Ok(worked)
}
