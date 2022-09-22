//! 2.3.4 Lifetimes
use sec34::{add, Foo};

fn main() {
    let x = Foo { v: 10 };
    {
        let y = Foo { v: 20 };
        let z = add(&x, &y);
        println!("z = {z}");
        assert_eq!(z, 30);
    }
    assert_eq!(x.v, 10);
}
