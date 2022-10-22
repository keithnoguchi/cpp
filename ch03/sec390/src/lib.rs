//! 3.9.0 Bakery Lock
mod lock;

pub use lock::{BakeryLock, LockGuard};

use std::error::Error;
use std::result;

/// Number of threads supported by the BakeryLock.
///
/// This is a limitation of the current BakeryLock, that the
/// number of threads are pre-defined during the compile time.
///
/// Let's see if we can make it to be dynamic in the future
/// iteration.
pub const NR_THREADS: usize = 10;

/// Counter updated by each workers protected by the BakeryLock.
pub static mut COUNTER: u64 = 0;

type Result<T> = result::Result<T, Box<dyn Error + Send>>;

pub fn worker(id: u64, loops: usize) -> Result<u64> {
    (0..loops).for_each(|_| {
        unsafe {
            COUNTER += 1;
        };
    });
    Ok(id)
}
