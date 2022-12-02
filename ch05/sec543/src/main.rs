//! Aync and Async Workers on tokio runtime
use sec543::{async_worker, sync_worker};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex as SyncMutex;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::sleep;
use tracing::{error, info};

const NR_TIMEOUT: Duration = Duration::from_secs(5);
const NR_RUNTIME_WORKER_THREADS: usize = 16;
const NR_SYNC_WORKERS: usize = 50;
const NR_ASYNC_WORKERS: usize = 1_000;
const NR_WORKER_JOBS: usize = 500;

fn main() {
    tracing_subscriber::fmt::init();
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_TIMEOUT);
    let nr_runtime_worker_threads = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RUNTIME_WORKER_THREADS);
    let nr_sync_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SYNC_WORKERS);
    let nr_async_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_ASYNC_WORKERS);
    let nr_worker_jobs = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKER_JOBS);

    info!(
        progname = ?progname.file_name().unwrap(),
        timeout = ?nr_timeout,
        runtime_worker_threads = %nr_runtime_worker_threads,
        sync_workers = %nr_sync_workers,
        async_workers = %nr_async_workers,
        worker_jobs = %nr_worker_jobs,
        "start",
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(nr_runtime_worker_threads)
        .enable_time()
        .build()
        .expect("tokio runtime");

    if let Err(e) = runtime.block_on(async move {
        let sync_counter0 = Arc::new(SyncMutex::new(0));
        let async_counter0 = Arc::new(AsyncMutex::new(0));

        let mut workers = vec![];
        (0..nr_sync_workers).for_each(|_| {
            let counter = sync_counter0.clone();
            workers.push(tokio::task::spawn_blocking(move || {
                // This make compiler happy to be in sync with
                // the async_worker signature below.
                //
                // I'll come back to see if I can avoid the closure
                // in the next iteration.
                sync_worker(counter, nr_worker_jobs)
            }));
        });
        (0..nr_async_workers).for_each(|_| {
            let counter = async_counter0.clone();
            workers.push(tokio::spawn(async_worker(counter, nr_worker_jobs)));
        });
        sleep(nr_timeout).await;
        for worker in &workers {
            worker.abort();
        }
        for worker in workers.drain(..) {
            match worker.await {
                Err(e) => error!(error = %e, "worker panic"),
                Ok(result) => {
                    if let Err(e) = result {
                        error!(error = %e, "worker error");
                    }
                }
            }
        }
        let sync_counter = *sync_counter0.lock().unwrap();
        let async_counter = *async_counter0.lock().await;
        info!(
            %sync_counter,
            %async_counter,
            "finish",
        );
        assert_eq!(sync_counter, nr_sync_workers * nr_worker_jobs);
        assert_eq!(async_counter, nr_async_workers * nr_worker_jobs);
        Ok::<_, Box<dyn Error>>(())
    }) {
        error!(error = %e, "runtime error");
    }
}
