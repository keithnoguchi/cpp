//! 3.8.3 R/W Lock
use std::error::Error;
use std::result;
use std::sync::{Arc, RwLock};

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

pub fn reader(id: u64, lock: Arc<RwLock<u64>>) -> Result<u64> {
    sleep();
    let counter = lock.read().unwrap();
    println!("reader{id:03}: {:-3}", *counter);
    Ok(id)
}

pub fn writer(id: u64, lock: Arc<RwLock<u64>>) -> Result<u64> {
    sleep();
    let mut counter = lock.write().unwrap();
    *counter += 1;
    println!("writer{id:03}: {:-3}", *counter);
    Ok(id)
}

#[inline]
fn sleep() {
    const MAX_DELAY_IN_MILLIS: u16 = 500;
    let delay = rand::random::<u16>() % MAX_DELAY_IN_MILLIS;
    std::thread::sleep(std::time::Duration::from_millis(delay as u64));
}
