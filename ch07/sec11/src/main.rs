//! FairLock example
use sec11::FairLock;

fn main() {
    let lock = FairLock::new(0);
    println!("{:?}", lock);
}
