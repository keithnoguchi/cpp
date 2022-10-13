//! 3.8.2 Conditino Variable
use std::error::Error;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn Error + Send>>;

pub fn worker(id: u64, cvar: Arc<(Mutex<bool>, Condvar)>) -> Result<u64> {
    let &(ref lock, ref cvar) = &*cvar;

    let mut wakeup = lock.lock().unwrap();
    while !*wakeup {
        // let it panic and propagate to the spawner.
        wakeup = cvar.wait(wakeup).unwrap();
    }
    Ok(id)
}

pub fn waker(delay: Duration, cvar: Arc<(Mutex<bool>, Condvar)>) -> Result<()> {
    sleep(delay);

    // okay, let's wake everyone up.
    let &(ref lock, ref cvar) = &*cvar;
    let mut wakeup = lock.lock().unwrap();
    *wakeup = true;
    cvar.notify_all();

    Ok(())
}
