//! Async and Sync Mutex with tokio
use sec541::{async_locker, async_sleeper, sync_locker};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime;
use tokio::time::sleep;
use tracing::{error, info};

const NR_RUNTIME_TIMEOUT: Duration = Duration::from_secs(10);
const NR_RUNTIME_WORKER_THREADS: usize = 4;
const NR_SYNC_LOCKERS: usize = 10_000;
const NR_ASYNC_LOCKERS: usize = 20_000;
const NR_ASYNC_SLEEPERS: usize = 100;
const NR_LOCKER_LOOP: usize = 20;
const NR_SLEEPER_LOOP: usize = 5;
const NR_SLEEPER_SLEEP: Duration = Duration::from_millis(2);

fn main() {
    tracing_subscriber::fmt::init();
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_RUNTIME_TIMEOUT);
    let nr_runtime_worker_threads = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RUNTIME_WORKER_THREADS);
    let nr_sync_lockers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SYNC_LOCKERS);
    let nr_async_lockers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_ASYNC_LOCKERS);
    let nr_async_sleepers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_ASYNC_SLEEPERS);
    let nr_locker_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LOCKER_LOOP);
    let nr_sleeper_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SLEEPER_LOOP);
    let nr_sleeper_sleep = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_millis)
        .unwrap_or(NR_SLEEPER_SLEEP);

    info!(
        runtime_workers = %nr_runtime_worker_threads,
        "{:?}",
        progname.file_name().unwrap(),
    );

    let runtime = runtime::Builder::new_multi_thread()
        .worker_threads(nr_runtime_worker_threads)
        .enable_time()
        .build()
        .expect("tokio runtime");

    let sync_counter0 = Arc::new(std::sync::Mutex::new(0));
    let sync_counter = sync_counter0.clone();
    runtime.block_on(async move {
        let async_counter = Arc::new(tokio::sync::Mutex::new(0));
        let mut workers = vec![];
        (0..nr_sync_lockers).for_each(|_| {
            workers.push(tokio::spawn(sync_locker(
                sync_counter.clone(),
                nr_locker_loop,
            )))
        });
        (0..nr_async_lockers).for_each(|_| {
            workers.push(tokio::spawn(async_locker(
                async_counter.clone(),
                nr_locker_loop,
            )))
        });
        (0..nr_async_sleepers).for_each(|_| {
            workers.push(tokio::spawn(async_sleeper(
                async_counter.clone(),
                nr_sleeper_loop,
                nr_sleeper_sleep,
            )))
        });
        // give sometime to complete the jobs
        sleep(nr_timeout).await;
        for worker in workers.drain(..) {
            // safe to abort the completed tasks
            worker.abort();
            if let Err(e) = worker.await {
                error!(error = %e, "task crash");
            }
        }
        // async mutex is only allowed under async function.
        assert_eq!(
            *async_counter.lock().await,
            nr_async_lockers * nr_locker_loop + nr_async_sleepers * nr_sleeper_loop,
        );
    });
    assert_eq!(
        *sync_counter0.lock().unwrap(),
        nr_sync_lockers * nr_locker_loop,
    );
}
