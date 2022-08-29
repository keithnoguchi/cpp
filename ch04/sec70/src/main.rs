//! 4.7 Memory Barrier
//!
//! SPDX-License-Identifier: GPL-2.0
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    fn new(v: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
        }
    }
}

fn main() {
    let lock = SpinLock::new(10);
    println!("Let's have fun with UnsafeCell!: {:?}", lock);
}
