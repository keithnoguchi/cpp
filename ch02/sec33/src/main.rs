//! 2.3.3 Ownership
#[allow(unused_imports)]
use sec33::{get_full_stomach, get_gold, Apple};

fn main() {
    let apple = Apple {};
    let gold = get_gold(apple);
    println!("got {gold:?}");
    // apple is gone here.
    //let full_stomach = get_full_stomach(apple);
    //println!("I'm {full_stomach:?}");
}
