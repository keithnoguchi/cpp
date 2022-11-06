//! Spinlock in Rust
use sec470::Lock;
use std::error::Error;
use std::path::PathBuf;
use std::result;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::{spawn, yield_now};

const NR_WORKERS: usize = 50;
const NR_LOOP: usize = 20_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().as_ref().map(PathBuf::from).unwrap();
    let nr_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKERS);
    let nr_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LOOP);

    println!(
        "{:?}: {} count up total by {} workers",
        progname.file_name().unwrap(),
        nr_loop * nr_workers,
        nr_workers,
    );

    let counter0 = Arc::new(Lock::new(0));
    let mut workers = vec![];
    (0..nr_workers).for_each(|id| {
        let counter = counter0.clone();
        workers.push(spawn(move || worker(id as u64, nr_loop, counter)));
    });
    for (id, worker) in workers.drain(..).enumerate() {
        match worker.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
    assert_eq!(*counter0.lock(), nr_loop * nr_workers);
}

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

fn worker(id: u64, nr_loop: usize, counter: Arc<Lock<usize>>) -> Result<u64> {
    // Just lock and increment the counter.
    (0..nr_loop).for_each(|_| {
        {
            let mut counter = counter.lock();
            *counter += 1;
        }
        yield_now();
    });
    Ok(id)
}
