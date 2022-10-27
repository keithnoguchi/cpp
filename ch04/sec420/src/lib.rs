//! 4.2.0 Livelocks
use std::error::Error;
use std::result;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

const NR_EATING: Duration = Duration::from_millis(10);
const NR_TRIES: usize = 100;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub fn philosopher(id: u64, chopsticks: [Arc<Mutex<usize>>; 2], bites: usize) -> Result<u64> {
    let name = format!("philosopher{id}");
    for _ in 0..bites {
        let mut tries = 0;
        loop {
            tries += 1;
            if tries >= NR_TRIES {
                Err(format!("{name} tried {tries} times in vain..."))?;
            }
            let left = match chopsticks[0].try_lock() {
                Err(_e) => {
                    wait(&name);
                    continue;
                }
                Ok(left) => left,
            };
            let right = match chopsticks[1].try_lock() {
                Err(_e) => {
                    wait(&name);
                    continue;
                }
                Ok(right) => right,
            };
            println!("{name} got both chopsticks, #{}, #{}", *left, *right);
            break;
        }
        sleep(NR_EATING);
    }
    Ok(id)
}

fn wait(name: &str) -> Duration {
    println!("{name} let's wait a bit");
    Duration::from_millis(rand::random::<u64>())
}
