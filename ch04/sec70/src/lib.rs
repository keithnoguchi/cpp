//! SPDX-License-Identifier: GPL-2.0
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct SpinLock<T> {
    lock: AtomicBool,
    _data: UnsafeCell<T>,
}

pub struct SpinLockGuard<'a, T> {
    spin_lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(v: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            _data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        loop {
            // spinning for the lock.
            while self.lock.load(Ordering::Relaxed) {}

            if self.lock.compare_exchange_weak(
                false,
                true,
                Ordering::Acquire, // ordering for success
                Ordering::Relaxed, // ordering for failure
            ).is_ok() {
                break;
            }
        }
        SpinLockGuard { spin_lock: self }
    }
}
