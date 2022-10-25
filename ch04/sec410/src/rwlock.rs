//! 4.1.0 Deadlock
use crate::Result;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

const NR_TRY_WRITE: usize = 100;
const NR_TRY_WRITE_SLEEP: Duration = Duration::from_millis(10);

pub fn deadlocker(id: u64, val: Arc<RwLock<bool>>, deadlock: bool) -> Result<u64> {
    if !deadlock {
        // dereference below, '*', save the world.
        let v = *val.read().unwrap();
        if v {
            let mut v = val.write().unwrap();
            *v = false;
        }
    } else {
        let v = val.read().unwrap();
        if *v {
            let mut tried = 0;
            let mut v = loop {
                tried += 1;
                if tried >= NR_TRY_WRITE {
                    panic!("deadlocker{id}: deadlocked");
                }
                match val.try_write() {
                    Err(_e) => {
                        sleep(NR_TRY_WRITE_SLEEP);
                        continue;
                    }
                    Ok(v) => break v,
                }
            };
            *v = false;
        }
    }
    Ok(id)
}
