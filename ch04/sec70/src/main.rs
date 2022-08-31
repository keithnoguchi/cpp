//! 4.7 Memory Barrier
//!
//! SPDX-License-Identifier: GPL-2.0
use sec70::SpinLock;

use std::sync::Arc;
use std::thread;

const NUM_THREADS: usize = 10;
const NUM_LOOP: usize = 1_000_000;

fn main() {
    let counter = Arc::new(SpinLock::new(0));
    let mut workers = Vec::new();

    // create workers.
    for _ in 0..NUM_THREADS {
        let counter0 = counter.clone();
        workers.push(thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut counter = counter0.lock();
                *counter += 1;
            }
        }));
    }

    for worker in workers {
        if let Err(e) = worker.join() {
            eprintln!("thread crash: {e:?}");
        }
    }

    let result = *counter.lock();
    println!("counter holds {:#}", result);
    assert_eq!(result, NUM_THREADS * NUM_LOOP);
}
