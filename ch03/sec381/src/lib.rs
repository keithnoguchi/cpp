//! 3.8.1 Mutex in Rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn worker(counter: Arc<Mutex<u64>>, max: u64) {
    let id = thread::current().id();
    let result;
    loop {
        let mut count = counter.lock().unwrap();
        if *count >= max {
            result = *count;
            break;
        }
        *count += 1;
    }
    println!("worker({id:?}): {result}");
}
