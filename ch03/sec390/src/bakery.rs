//! 3.9.0 Bakery Lock
use crate::NR_THREADS;
use std::ptr::write_volatile;
use std::sync::atomic::{fence, Ordering};

pub struct Lock {
    // The current version of the BakeryLock has a limitation of
    // the number of the threads sharing this lock statically
    // defined at the compile time.
    entering: [bool; NR_THREADS],
    tickets: [Option<u64>; NR_THREADS],
}

/// LockGuard to protect the lock.
pub struct LockGuard {
    id: usize,
}

// Global mutable variable, woohoo! :)
static mut LOCK: Lock = Lock {
    entering: [false; NR_THREADS],
    tickets: [None; NR_THREADS],
};

macro_rules! write_mem {
    ($addr: expr, $value: expr) => {
        unsafe { write_volatile($addr, $value) }
    };
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.id], None);
    }
}
