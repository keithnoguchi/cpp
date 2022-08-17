//! Mutex in Rust
use std::sync::{Arc, Mutex};
use std::thread::spawn;

// The goal of the counster.
const WANT: u64 = u16::MAX as u64;

fn main() {
    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();
    let lock2 = lock0.clone();

    // run workers.
    let th1 = spawn(move || {
        run(lock1);
    });
    let th2 = spawn(move || {
        run(lock2);
    });

    // wait for the completion.
    if let Err(e) = th1.join() {
        eprintln!("thread0: {e:?}");
    }
    if let Err(e) = th2.join() {
        eprintln!("thread1: {e:?}");
    }

    // get the counter value.
    let v = lock0.lock().unwrap();
    println!("counter = {}", *v);

    // which should be equal to WANT.
    assert_eq!(*v, WANT);
}

fn run(lock: Arc<Mutex<u64>>) {
    loop {
        let mut v = lock.lock().unwrap();
        if *v >= WANT {
            return;
        }
        *v += 1;
    }
}
