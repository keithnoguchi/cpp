//! 4.6.1 Signal Handler in Rust
use signal_hook::consts::SIGUSR1;
use signal_hook::iterator::Signals;
use std::error::Error;
use std::result;
use std::thread::{sleep, spawn};
use std::time::Duration;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub fn handler(id: u64, timeout: Duration) -> Result<u64> {
    let mut signals = Signals::new(&[SIGUSR1])?;
    let handle = signals.handle();

    // set the signal handler timeout.
    let cleanup = spawn(move || {
        sleep(timeout);
        handle.close();
    });

    // let's receive the signal!
    for sig in signals.forever() {
        println!("handler{id}: received signal #{sig}");
    }

    match cleanup.join() {
        Err(e) => Err(format!("handler{id}: {e:?}"))?,
        Ok(_) => Ok(id),
    }
}
