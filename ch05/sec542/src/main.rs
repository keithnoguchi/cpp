//! 5.4.2 Cancel Pattern with tokio broadcast Channel
use sec542::worker;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio::time::sleep;
use tracing::{error, info};

const NR_TIMEOUT: Duration = Duration::from_secs(5);
const NR_TOKIO_RUNTIME_WORKER_THREADS: usize = 10;
const NR_TOKIO_BROADCAST_CHANNEL_BOUND: usize = 2;
const NR_WORKERS: usize = 1_000;
const NR_WORKER_JOBS: usize = 1_000;

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
    let nr_tokio_runtime_worker_threads = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TOKIO_RUNTIME_WORKER_THREADS);
    let nr_tokio_broadcast_channel_bound = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TOKIO_BROADCAST_CHANNEL_BOUND);
    let nr_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKERS);
    let nr_worker_jobs = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKER_JOBS);

    info!(
        tokio_runtime_worker_threads = %nr_tokio_runtime_worker_threads,
        tokio_broadcast_channel_bound = %nr_tokio_broadcast_channel_bound,
        workers = %nr_workers,
        worker_jobs = %nr_worker_jobs,
        timeout = ?nr_timeout,
        "{:?}: cancel pattern with tokio broadcast channel",
        progname.file_name().unwrap(),
    );

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(nr_tokio_runtime_worker_threads)
        .enable_time()
        .build()
        .expect("tokio runtime");

    // let's create a main task.
    if let Err(e) = runtime.block_on(async move {
        let (tx, rx) = broadcast::channel(nr_tokio_broadcast_channel_bound);
        let counter0 = Arc::new(Mutex::new(0));

        // fire up the workers.
        let mut workers = vec![];
        let mut failed = 0;
        (0..nr_workers).for_each(|_| {
            let done = tx.subscribe();
            let counter = counter0.clone();
            workers.push(tokio::spawn(worker(done, counter, nr_worker_jobs)));
        });

        // wait for the workers to finish.
        sleep(nr_timeout).await;
        if let Err(e) = tx.send(()) {
            error!(error = %e, "all the tasks were gone");
            Err(e)?;
        }
        drop(rx);

        // make sure all the workers complete the work.
        for worker in workers.drain(..) {
            match worker.await {
                Err(e) => {
                    error!(error = %e, "worker task panic");
                    failed += 1;
                }
                Ok(result) => match result {
                    Err(e) => {
                        error!(error = %e, "worker task failed");
                        failed += 1;
                    }
                    Ok(worked) => assert_eq!(worked, nr_worker_jobs),
                },
            }
        }
        if failed != 0 {
            Err(format!("{failed} worker task(s) failed"))?
        }
        assert_eq!(*counter0.lock().await, nr_workers * nr_worker_jobs);
        Ok::<_, Box<dyn Error>>(())
    }) {
        error!(error = %e, "main task error");
        exit(1);
    }
}
