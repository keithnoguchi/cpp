// SPDX-License-Identifier: GPL-2.0
use std::thread::spawn;

fn main() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result is {:?}", result);

    match spawn(|| panic!("I'm paniced")).join() {
        Ok(_) => println!("successed"),
        Err(e) => {
            let s = e.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}
