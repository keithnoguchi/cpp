//! 3.9.0 Bakery Lock
use sec390::{worker, COUNTER, NR_THREADS};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::spawn;

// Number of loop by the workers to lock and update the counter.
const NR_LOOP: usize = 1_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LOOP);

    println!(
        "{:?} {} threads with {} loop",
        progname.file_name().unwrap(),
        NR_THREADS,
        nr_loop,
    );

    let mut workers = vec![];
    (0..NR_THREADS).for_each(|id| {
        workers.push(spawn(move || worker(id as u64, nr_loop)));
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
    let counter = unsafe { COUNTER };
    println!("{:?}: counter = {counter}", progname.file_name().unwrap());
    assert_eq!(counter, (NR_THREADS * nr_loop) as u64);
}
