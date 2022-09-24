//! 2.3.7 Traits and Trait Bounds
use sec237::{add_3times, Vec2};

fn main() {
    let v1 = Vec2::new(10.0, 5.0);
    let v2 = Vec2::new(3.1, 8.7);
    println!("v1={v1}, v2={v2}");
    let v = v1 + v2;
    println!("v={v}");
    assert_eq!(v.x, 13.1);
    assert_eq!(v.y, 13.7);
    let v = add_3times(v);
    println!("v={v}");
    assert_eq!(v.x, 39.3);
    assert_eq!(v.y, 41.1);
}
