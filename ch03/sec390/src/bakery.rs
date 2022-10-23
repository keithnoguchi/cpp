//! 3.9.0 Bakery Lock
use crate::NR_THREADS;
use std::ptr::{read_volatile, write_volatile};
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
pub static mut LOCK: Lock = Lock {
    entering: [false; NR_THREADS],
    tickets: [None; NR_THREADS],
};

macro_rules! read_mem {
    ($addr: expr) => {
        unsafe { read_volatile($addr) }
    };
}

macro_rules! write_mem {
    ($addr: expr, $value: expr) => {
        unsafe { write_volatile($addr, $value) }
    };
}

impl Lock {
    pub fn lock(&mut self, id: usize) -> LockGuard {
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[id], true);
        fence(Ordering::SeqCst);

        // get the next available ticket.
        let mut max = 0;
        self.tickets.iter().for_each(|t| {
            if let Some(t) = t {
                max = max.max(*t);
            }
        });

        let ticket = max + 1;
        write_mem!(&mut self.tickets[id], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[id], false);
        fence(Ordering::SeqCst);

        // waiting for your turn.
        for i in 0..NR_THREADS {
            if i == id {
                continue;
            }

            // waiting for the other one in case it's taking a ticket.
            while read_mem!(&self.entering[i]) {}

            // wait until your turn is higher than others.
            while let Some(t) = self.tickets[i] {
                if ticket < t || (ticket == t && id < i) {
                    // Either you have a smaller ticket number or
                    // you're in the higher precedence.
                    break;
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { id }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.id], None);
    }
}
