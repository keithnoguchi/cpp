//! 4.2.0 Livelocks
use sec420::philosopher;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

const NR_PHILOSOPHERS: usize = 2;
const NR_BITES: usize = 50;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_philosophers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_PHILOSOPHERS);
    let nr_bites = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_BITES);

    println!(
        "{:?}: {} philosophers eat {} bites each",
        progname.file_name().unwrap(),
        nr_philosophers,
        nr_bites,
    );

    let nr_chopsticks = nr_philosophers * 2 - 1;
    let mut chopsticks0 = Vec::with_capacity(nr_chopsticks);
    (0..nr_chopsticks).for_each(|id| {
        chopsticks0.push(Arc::new(Mutex::new(id)));
    });

    let mut philosophers = vec![];
    (0..nr_philosophers).for_each(|id| {
        let chopsticks = if id == nr_philosophers - 1 {
            [chopsticks0[id].clone(), chopsticks0[0].clone()]
        } else {
            [chopsticks0[id].clone(), chopsticks0[id + 1].clone()]
        };
        philosophers.push(spawn(move || philosopher(id as u64, chopsticks, nr_bites)));
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
