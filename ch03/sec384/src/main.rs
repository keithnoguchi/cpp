//! 3.8.4 Memory Barrier
use sec384::worker;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Barrier};
use std::thread::spawn;

// 20,000 workers crashes it on my 8Gi MBA.
const NR_WORKERS: usize = 15_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_WORKERS);

    println!("{:?}: {nr_workers} workers", progname.file_name().unwrap());

    let barrier0 = Arc::new(Barrier::new(nr_workers));
    let mut workers = vec![];
    (0..nr_workers).for_each(|id| {
        let barrier = barrier0.clone();
        workers.push(spawn(move || worker(id as u64, barrier)));
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
