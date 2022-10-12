//! 3.8.1 Mutex in Rust
use sec381::worker;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

const NR_WORKERS: usize = 1_000;
const NR_MAX_COUNT: u64 = 1_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_workers = args
        .next()
        .map_or(NR_WORKERS, |arg| usize::from_str(&arg).unwrap());
    let nr_max_count = args
        .next()
        .map_or(NR_MAX_COUNT, |arg| u64::from_str(&arg).unwrap());

    let mut workers = vec![];
    let counter0 = Arc::new(Mutex::new(0));
    (0..nr_workers).for_each(|_| {
        let counter = counter0.clone();
        workers.push(spawn(move || worker(counter, nr_max_count)))
    });

    for worker in workers.drain(..) {
        let id = worker.thread().id();
        if let Err(e) = worker.join() {
            eprintln!("worker{id:?}: {e:?}");
        }
    }

    let result = counter0.lock().unwrap();
    println!("{:?}: counter={}", progname.file_name().unwrap(), *result);
    assert_eq!(*result, nr_max_count);
}
