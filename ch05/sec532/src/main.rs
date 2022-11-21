//! 5.3.2 I/O Selector with epoll(7)
use sec532::{Executor, Reader, Selector};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;

const NR_TIMEOUT: Duration = Duration::from_secs(5);
const NR_BASE_PORT: usize = 40_000;
const NR_LISTENERS: usize = 1;
const NR_SPAWNERS: usize = 3;
const NR_RUN_QUEUE_BOUND: usize = 2048;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_TIMEOUT);
    let nr_base_port = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_BASE_PORT);
    let nr_listeners = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LISTENERS);
    let nr_spawners = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SPAWNERS);
    let nr_run_queue_bound = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RUN_QUEUE_BOUND);

    println!(
        "{:?}: {} listin on {}..{} with {:?} timeout",
        progname.file_name().unwrap(),
        nr_listeners,
        nr_base_port,
        nr_base_port + nr_listeners,
        nr_timeout,
    );

    // spawn selector.
    let mut workers = vec![];
    let selector0 = match Selector::new() {
        Ok(selector) => Arc::new(selector),
        Err(e) => panic!("{e}"),
    };
    let selector = selector0.clone();
    workers.push(spawn(move || selector.select(nr_timeout)));

    // spawn executor
    let executor = Executor::new(nr_run_queue_bound);
    let spawner0 = executor.spawner();
    workers.push(spawn(move || executor.run()));

    // spawn listeners
    let counter0 = Arc::new(AtomicUsize::new(0));
    (0..nr_listeners)
        .into_iter()
        .map(|port| nr_base_port + port)
        .collect::<Vec<_>>()
        .chunks(nr_listeners / nr_spawners + 1)
        .map(Vec::<_>::from)
        .for_each(|ports| {
            let spawner = spawner0.clone();
            let selector1 = selector0.clone();
            let counter1 = counter0.clone();
            workers.push(spawn(move || {
                for port in ports {
                    let selector = selector1.clone();
                    let counter = counter1.clone();
                    if let Err(e) = spawner.spawn(async move {
                        println!("{port}");
                        counter.fetch_add(1, Relaxed);
                        let stdin = io::stdin();
                        let mut reader = Reader::new(stdin, selector);
                        // This blocks forever...
                        while let Some(line) = reader.read_line().await {
                            print!("{line}");
                        }
                    }) {
                        panic!("{e}");
                    }
                }
                Ok(())
            }));
        });

    // dropping spawner here to let executor done with the job
    drop(spawner0);
    for worker in workers.drain(..) {
        if let Err(e) = worker.join() {
            panic!("{e:?}");
        }
    }
    assert_eq!(counter0.load(Relaxed), nr_listeners);
}
