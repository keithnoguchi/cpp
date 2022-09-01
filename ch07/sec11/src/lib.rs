//! SPDX-License-Identifier: GPL-2.0
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize};

/// Maximum number of threads to race for the lock.
pub const NUM_LOCK: usize = 8;

#[derive(Debug)]
pub struct FairLock<T> {
    _waiting: Vec<AtomicBool>,
    _turn: AtomicUsize,
    _lock: AtomicBool,
    _data: UnsafeCell<T>,
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self {
        let mut waiting = Vec::new();
        for _ in 0..NUM_LOCK {
            waiting.push(AtomicBool::new(false));
        }
        Self {
            _waiting: waiting,
            _turn: AtomicUsize::new(0),
            _lock: AtomicBool::new(false),
            _data: UnsafeCell::new(v),
        }
    }
}
