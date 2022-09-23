//! 2.3.6 Methods

use sec36::Vec2;

fn main() {
    let mut v = Vec2::new(10.0, 5.0);
    println!("{v} is normalized as {}", v.norm());
    v.set(3.8, 9.1);
    println!("{v} is normalized as {}", v.norm());
}
