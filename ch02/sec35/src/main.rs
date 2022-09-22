//! 2.3.5 Borrow Checker
use sec35::Foo;

fn main() {
    let mut x = Foo { v: 25 };
    {
        let a = &mut x;
        println!("a = {a}");
        // You can't reference x here, as it's mutably borrowed
        // to a.
        //
        // Note that you can if you don't use 'a' below.
        //println!("x = {x}");

        let b = &a;
        // You can't assign/reference a here, as it's immutably
        // borrowed to b above.
        //
        // Note that you can if you don't use 'b' below.
        //a.v = 20;
        println!("b = {b}");

        // Now you can assign to a.
        a.v = 20;
        println!("a = {a}");
    }
    x.v = 30;
    println!("x = {x}");

    {
        let c = &x;
        println!("c = {c}");
        println!("x = {x}");

        // You can't do that, as it's immutably bollowed by c.
        // Note that you can if you don't use 'c' below.
        //
        // Note also that you can in case the use is 'x', though!
        //let d = &mut x;
        //d.v = 40;
        //println!("d = {d}");

        println!("c = {c}");
        println!("x = {x}");
    }
    println!("x = {x}");
}
