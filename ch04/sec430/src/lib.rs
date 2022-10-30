//! Dijkstra Banker Algorithm
mod banker;

pub use banker::Banker;

use std::error::Error;
use std::result;
use std::thread::sleep;
use std::time::Duration;

// 2 chopsticks to eat.
const NR_CHOPSTICKS: usize = 2;
const NR_BITES: usize = 20;
const NR_BITE_DURATION: Duration = Duration::from_millis(100);
const NR_TRIES: usize = 100;
const NR_TRY_DELAY: Duration = Duration::from_millis(100);

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

pub fn philosopher<const NR_RESOURCES: usize, const NR_PHILOSOPHERS: usize>(
    id: u64,
    chopsticks: [usize; NR_CHOPSTICKS],
    banker: Banker<NR_RESOURCES, NR_PHILOSOPHERS>,
) -> Result<u64> {
    let name = format!("philosopher{id}");
    for _ in 0..NR_BITES {
        let id = id as usize;
        let left = chopsticks[0];
        if !pick_chopstick(id, left, &banker) {
            panic!("{name} can't take left chopstick #{left}");
        }
        let right = chopsticks[1];
        if !pick_chopstick(id, right, &banker) {
            panic!("{name} can't take right chopstick #{right}");
        }
        sleep(NR_BITE_DURATION);
        banker.release(id, right);
        banker.release(id, left);
    }
    Ok(id)
}

fn pick_chopstick<const R: usize, const N: usize>(
    id: usize,
    chopstick_id: usize,
    banker: &Banker<R, N>,
) -> bool {
    let mut tries = 0;
    loop {
        if banker.take(id, chopstick_id) {
            break true;
        }
        tries += 1;
        if tries == NR_TRIES {
            break false;
        }
        sleep(NR_TRY_DELAY);
    }
}
