//! 2.3.9 Threads
use sec39::hello;

use std::thread::spawn;

fn main() {
    // spawn function.
    match spawn(hello).join() {
        Ok(r) => println!("success: {r:?}"),
        Err(e) => {
            let ret = e.downcast_ref::<&str>();
            eprintln!("error: {ret:?}");
        }
    }

    // spawn closure.
    let v = 10;
    let f = move || v * 2;
    match spawn(f).join() {
        Ok(r) => println!("success: {r}"),
        Err(e) => {
            let ret = e.downcast_ref::<&str>();
            eprintln!("error: {ret:?}");
        }
    }

    // thread crash handling.
    match spawn(|| panic!("oops")).join() {
        Ok(r) => println!("success: {r:?}"),
        Err(e) => {
            let ret = e.downcast_ref::<&str>();
            // ret type is Option<&&str>...
            eprintln!("error: {ret:?}");
        }
    }
}
