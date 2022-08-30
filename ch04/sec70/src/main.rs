//! 4.7 Memory Barrier
//!
//! SPDX-License-Identifier: GPL-2.0
use sec70::SpinLock;

fn main() {
    let lock = SpinLock::new(10);
    println!("Let's have fun with UnsafeCell!: {:?}", lock);
}
