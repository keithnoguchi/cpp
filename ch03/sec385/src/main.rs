//! 3.8.5 Semaphore and Channel
use sec385::{channel, consumer, producer, worker, Semaphore};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;

// For the semaphore module example.
const NR_WORKERS: usize = 10_000;
const NR_SEMAPHORES: usize = NR_WORKERS / 2; // half the worker is allowed.
const NR_WORKER_JOBS: usize = 100;

// For the channel module example.
const NR_PRODUCERS: usize = 2;
const NR_CONSUMERS: usize = 3;
const NR_PRODUCER_JOBS: usize = 60;
const NR_CHANNEL_SIZE: usize = 10;

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
    let nr_producers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PRODUCERS);
    let nr_consumers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_CONSUMERS);
    let nr_producer_jobs = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PRODUCER_JOBS);
    let nr_channel_size = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_CHANNEL_SIZE);

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

    println!(
        "{:?}: {} jobs on {} bounded channel with {}/{} producer/consumer.",
        progname.file_name().unwrap(),
        nr_producer_jobs,
        nr_channel_size,
        nr_producers,
        nr_consumers,
    );
    let (tx0, rx0) = channel(nr_channel_size as isize);
    let jobs = nr_producer_jobs / nr_producers;
    let mut producers = vec![];
    (0..nr_producers).for_each(|id| {
        let tx = tx0.clone();
        let data = format!("{id}"); // simple string data.
        producers.push(spawn(move || producer(id as u64, data, jobs, tx)));
    });
    let jobs = nr_producer_jobs / nr_consumers;
    let mut consumers = vec![];
    (0..nr_consumers).for_each(|id| {
        let rx = rx0.clone();
        consumers.push(spawn(move || consumer(id as u64, jobs, rx)));
    });
    for (id, producer) in producers.drain(..).enumerate() {
        match producer.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
    for (id, producer) in producers.drain(..).enumerate() {
        match producer.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
}
