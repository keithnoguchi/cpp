//! SPDX-License-Identifier: GPL-2.0
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;

#[derive(Debug)]
pub struct SpinLock<T> {
    _lock: AtomicBool,
    _data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub fn new(v: T) -> Self {
        Self {
            _lock: AtomicBool::new(false),
            _data: UnsafeCell::new(v),
        }
    }
}
