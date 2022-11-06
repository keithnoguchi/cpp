//! 4.7.0 Spinlock in Rust
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Lock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    inner: &'a Lock<T>,
}

unsafe impl<T> Send for Lock<T> {}
unsafe impl<T> Sync for Lock<T> {}

impl<T> Lock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::<T>::new(data),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        // TTAS
        loop {
            // Test and
            while self.lock.load(Relaxed) {
                std::hint::spin_loop()
            }

            // TAS
            if self
                .lock
                .compare_exchange_weak(
                    false,   // before
                    true,    // after
                    Acquire, // ldar for success
                    Relaxed, // for failure
                )
                .is_ok()
            {
                break;
            }
        }
        Guard { inner: self }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.inner.lock.store(false, Release)
    }
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.data.get() }
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.data.get() }
    }
}
