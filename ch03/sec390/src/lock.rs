//! 3.9.0 Bakery Lock
use crate::NR_THREADS;

pub struct BakeryLock {
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
