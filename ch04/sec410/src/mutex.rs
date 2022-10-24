//! 4.1.0 Deadlock with Mutex
use crate::Result;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

const NR_TRIES: usize = 1_000;
const NR_TRY_SLEEP: Duration = Duration::from_millis(10);
const NR_PICK_SLEEP: Duration = Duration::from_millis(100);
const NR_EAT_SLEEP: Duration = Duration::from_millis(500);

pub fn philosopher(id: u64, chopsticks: [Arc<Mutex<u64>>; 2], nr_loop: usize) -> Result<u64> {
    for _ in 0..nr_loop {
        let mut tries = 0;
        let left = loop {
            tries += 1;
            match chopsticks[0].try_lock() {
                Ok(chopstick) => break chopstick,
                Err(e) if tries == NR_TRIES => {
                    panic!("philosopher{id}: tried left chopstick {tries} times: {e}");
                }
                Err(_) => sleep(NR_TRY_SLEEP),
            }
        };
        sleep(NR_PICK_SLEEP);
        tries = 0;
        let right = loop {
            tries += 1;
            match chopsticks[1].try_lock() {
                Ok(chopstick) => break chopstick,
                Err(e) if tries == NR_TRIES => {
                    panic!("philosopher{id}: tried right chopstick {tries} times: {e}");
                }
                Err(_) => sleep(NR_TRY_SLEEP),
            }
        };
        println!(
            "philosoper{id}: let's eat with #{} and #{} chopsticks",
            left, right,
        );
        sleep(NR_EAT_SLEEP);
    }
    Ok(id)
}
