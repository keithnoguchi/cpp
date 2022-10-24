//! Deadlock with Rust
use sec410::mutex;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

const NR_PHILOSOPHERS: usize = 2;
const NR_LOOP: usize = 3;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_philosophers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PHILOSOPHERS);
    let nr_loop = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LOOP);

    println!(
        "{:?} {} philosophers with {} loop",
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
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
}
