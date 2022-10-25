//! Deadlock with Rust
use sec410::{mutex, rwlock};
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;

const NR_PHILOSOPHERS: usize = 2;
const NR_PHILOSOPHERS_LOOP: usize = 3;
const NR_RW_WORKERS: usize = 100;

fn main() {
    let mut args = std::env::args();
    let progname0 = args.next().map(PathBuf::from).unwrap();
    let nr_philosophers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PHILOSOPHERS);
    let nr_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PHILOSOPHERS_LOOP);
    let nr_rw_workers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RW_WORKERS);
    let is_deadlock = args.next().is_some();

    // running multiple task without impacting each other.
    let mut tasks = vec![];
    let progname = progname0.clone();
    tasks.push(spawn(move || {
        philosophers(progname, nr_philosophers, nr_loop)
    }));
    tasks.push(spawn(move || {
        deadlockers(progname0, nr_rw_workers, is_deadlock)
    }));

    for task in tasks.drain(..) {
        match task.join() {
            Err(e) => eprintln!("{e:?}"),
            Ok(result) => {
                if let Err(e) = result {
                    eprintln!("{e}");
                }
            }
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error + Send>>;

fn philosophers(progname: PathBuf, nr_philosophers: usize, nr_loop: usize) -> Result<()> {
    let taskname = "philosophers";
    println!(
        "{:?}::{taskname}: {} philosophers with {} loop",
        progname.file_name().unwrap(),
        nr_philosophers,
        nr_loop,
    );

    // nr_philosophers * 2 - 1 chopsticks.
    let mut chopsticks0 = vec![];
    (0..nr_philosophers).for_each(|id| {
        chopsticks0.push(Arc::new(Mutex::new(id as u64)));
        if id != nr_philosophers - 1 {
            chopsticks0.push(Arc::new(Mutex::new((id + 1) as u64)));
        }
    });

    // Mutex based philosophers.
    let mut philosophers = vec![];
    (0..nr_philosophers).for_each(|id| {
        let chopsticks = if id != nr_philosophers - 1 {
            [chopsticks0[id].clone(), chopsticks0[id + 1].clone()]
        } else {
            [chopsticks0[id].clone(), chopsticks0[0].clone()]
        };
        philosophers.push(spawn(move || {
            mutex::philosopher(id as u64, chopsticks, nr_loop)
        }));
    });
    for (id, philosopher) in philosophers.drain(..).enumerate() {
        match philosopher.join() {
            Err(e) => panic!("{taskname}: {e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{taskname}: {e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
    Ok(())
}

fn deadlockers(progname: PathBuf, nr_workers: usize, is_deadlock: bool) -> Result<()> {
    let taskname = "deadlock";
    println!(
        "{:?}::{taskname}: {} deadlock workers with deadlock is {}",
        progname.file_name().unwrap(),
        nr_workers,
        is_deadlock,
    );

    let val0 = Arc::new(RwLock::new(true));
    let mut workers = vec![];
    (0..nr_workers).for_each(|id| {
        let val = val0.clone();
        workers.push(spawn(move || {
            rwlock::deadlocker(id as u64, val, is_deadlock)
        }));
    });
    for (id, worker) in workers.drain(..).enumerate() {
        match worker.join() {
            Err(e) => panic!("{taskname}: {e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{taskname}: {e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
    Ok(())
}
