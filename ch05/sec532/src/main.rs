//! 5.3.2 I/O Selector with epoll(7)
use sec532::{Executor, Selector};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::thread::spawn;

const NR_BASE_PORT: usize = 40_000;
const NR_LISTENERS: usize = 16;
const NR_SPAWNERS: usize = 3;
const NR_RUN_QUEUE_BOUND: usize = 2048;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
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
        "{:?}: {} listeners on {}..{} with {} run queue executor",
        progname.file_name().unwrap(),
        nr_listeners,
        nr_base_port,
        nr_base_port + nr_listeners,
        nr_run_queue_bound,
    );

    let executor = Executor::new(nr_run_queue_bound);
    let _selector = Selector::new();
    let spawner0 = executor.spawner();

    // spawn executor
    let mut workers = vec![];
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
            let counter1 = counter0.clone();
            workers.push(spawn(move || {
                for port in ports {
                    let counter = counter1.clone();
                    if let Err(e) = spawner.spawn(async move {
                        println!("{port}");
                        counter.fetch_add(1, Relaxed);
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
