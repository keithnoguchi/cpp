//! 4.2.0 Livelocks
use std::error::Error;
use std::result;
use std::sync::{Arc, Mutex};

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

pub fn philosopher(id: u64, chopsticks: [Arc<Mutex<usize>>; 2]) -> Result<u64> {
    let left = chopsticks[0].lock().unwrap();
    println!("philosopher{id}: got left chopstick, #{}", *left);
    Ok(id)
}
