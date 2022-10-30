//! 4.3.0 Dijkstra Banker Algorithm
use sec430::{philosopher, Banker};
use std::path::PathBuf;
use std::thread::spawn;

const NR_PHILOSOPHERS: usize = 5;
const NR_CHOPSTICKS: usize = NR_PHILOSOPHERS;
const NR_CHOPSTICKS_PER_RESOURCE: usize = 1;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();

    println!(
        "{:?}: Dining {} philosophers with banker algorithm",
        progname.file_name().unwrap(),
        NR_PHILOSOPHERS,
    );

    let chopsticks = [NR_CHOPSTICKS_PER_RESOURCE; NR_CHOPSTICKS];
    let chopsticks_for_philosophers = [
        [1, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0],
        [0, 0, 0, 1, 1],
        [1, 0, 0, 0, 1],
    ];
    let banker0 = Banker::new(chopsticks, chopsticks_for_philosophers);

    let mut philosophers = vec![];
    (0..NR_PHILOSOPHERS).for_each(|id| {
        let chopsticks = if id == NR_PHILOSOPHERS - 1 {
            [id, 0]
        } else {
            [id, id + 1]
        };
        let banker = banker0.clone();
        philosophers.push(spawn(move || philosopher(id as u64, chopsticks, banker)));
    });
    for (id, philosopher) in philosophers.drain(..).enumerate() {
        let progname = progname.file_name().unwrap();
        match philosopher.join() {
            Err(e) => panic!("{:?}: {e:?}", progname),
            Ok(result) => match result {
                Err(e) => panic!("{:?}: {e}", progname),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
}
