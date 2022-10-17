//! 3.8.5 Semaphore and Channel
use sec385::{worker, Semaphore};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;

const NR_WORKERS: usize = 10_000;
const NR_SEMAPHORES: usize = NR_WORKERS / 2; // half the worker is allowed.
const NR_WORKER_JOBS: usize = 100;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKERS);
    let nr_sems = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SEMAPHORES);
    let nr_jobs = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKER_JOBS);

    println!(
        "{:?}: {} workers with {}/{} jobs/semaphores",
        progname.file_name().unwrap(),
        nr_workers,
        nr_jobs,
        nr_sems,
    );
    let sem0 = Arc::new(Semaphore::new(nr_sems as isize));
    let mut workers = vec![];
    (0..nr_workers).for_each(|id| {
        let sem = Arc::clone(&sem0);
        workers.push(spawn(move || worker(id as u64, nr_jobs, sem)));
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
}
