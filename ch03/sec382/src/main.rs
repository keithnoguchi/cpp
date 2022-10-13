//! 3.8.2 Condition Variable
use sec382::{waker, worker};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::time::Duration;

const NR_WORKERS: u64 = 10_000;
const NR_DELAY: Duration = Duration::from_millis(100); // 100ms.

fn main() {
    let mut args = std::env::args();
    let progname = args
        .next()
        .map(|v| PathBuf::from(v).file_name().unwrap().to_os_string())
        .unwrap();
    let nr_workers = args
        .next()
        .map(|v| u64::from_str(&v).expect("number of workers"))
        .unwrap_or(NR_WORKERS);
    let nr_delay = args
        .next()
        .and_then(|v| u64::from_str(&v).ok())
        .map(Duration::from_millis)
        .unwrap_or(NR_DELAY);

    // A conditional variable, used for the workers to wait for waker.
    let cvar0 = Arc::new((Mutex::new(false), Condvar::new()));

    // wakers first.
    let mut workers = vec![];
    (0..nr_workers).for_each(|id| {
        let cvar = cvar0.clone();
        workers.push(spawn(move || worker(id, cvar)));
    });

    // fire the waker to wake all the worker up.
    let waker = spawn(move || waker(nr_delay, cvar0));

    // let's wait for all the workes to finish.
    for (id, worker) in workers.drain(..).enumerate() {
        match worker.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => debug_assert_eq!(got, id as u64),
            },
        }
    }
    if let Err(e) = waker.join() {
        panic!("{e:?}");
    }
    println!("{progname:?}: {nr_workers} workers with {nr_delay:?} delay");
}
