//! 4.3.0 Dijkstra Banker Algorithm
use sec430::Banker;
use std::path::PathBuf;

const NR_PHILOSOPHERS: usize = 4;
const NR_TOTAL_CHOPSTICKS: usize = NR_PHILOSOPHERS;
const NR_RESOURCE_FOR_CHOPSTICK: usize = 1;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();

    println!(
        "{:?}: Dining problem with {} philosophers",
        progname.file_name().unwrap(),
        NR_PHILOSOPHERS,
    );

    let total_chopsticks = [NR_RESOURCE_FOR_CHOPSTICK; NR_TOTAL_CHOPSTICKS];
    let for_each_philosophers = [[NR_RESOURCE_FOR_CHOPSTICK; NR_TOTAL_CHOPSTICKS]; NR_PHILOSOPHERS];
    let _banker = Banker::new(total_chopsticks, for_each_philosophers);
}
