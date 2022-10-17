//! 3.8.4 Memory Barrier
use std::error::Error;
use std::result;
use std::sync::{Arc, Barrier};

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

pub fn worker(id: u64, barrier: Arc<Barrier>) -> Result<u64> {
    let result = barrier.wait();
    if result.is_leader() {
        println!("worker{id} is the leader");
    }
    Ok(id)
}
