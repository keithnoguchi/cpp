//! Semaphore
use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    cnt: Mutex<isize>,
    max: isize,
    cvar: Condvar,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Self {
            cnt: Mutex::new(0),
            max,
            cvar: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut cnt = self.cnt.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cvar.wait(cnt).unwrap();
        }
        *cnt += 1;
    }

    pub fn post(&self) {
        let mut cnt = self.cnt.lock().unwrap();
        *cnt -= 1;
        if *cnt <= self.max {
            self.cvar.notify_one();
        }
    }

    #[inline]
    pub fn max(&self) -> isize {
        self.max
    }
}
