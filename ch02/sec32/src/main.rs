//! Section 2.3.2 Rust Grammer
use sec32::{add, app_n, even_odd, even_odd2, hello, is_even, mul, mul_x, print_pred};

fn main() {
    // Section 2.3.2.2 Functions
    let (x, y) = (9, 2);
    let r = add(x, y);
    hello(r);
    assert_eq!(11, r);

    // Section 2.3.2.3 If expressions
    let (x, y) = (11, 2);
    let r = is_even(x);
    assert!(!r);
    let r = is_even(y);
    assert!(r);

    // Section 2.3.2.4 Match Expressions
    let (x, y) = (10000, 0);
    print_pred(pred, x);
    print_pred(pred, y);

    // Section 2.3.2.5 For Statements
    let n = 10;
    even_odd(n);

    // Section 2.3.2.6 Loop Statements
    let n = 10;
    even_odd2(n);

    // Section 2.3.2.7 References and Dereferences
    let mut x = 3;
    let y = 5;
    let previous_x = x;
    mul(&mut x, &y);
    println!("mul(&mut {}, &{}) is {}", previous_x, y, x);
    assert_eq!(45, x);

    // Section 2.3.2.8 Function Pointers
    let n = 4;
    let x = 3;
    let r = app_n(mul2, n, x);
    println!("{n} times call of mul2({x}) is {r}");
    assert_eq!(48, r);

    // Section 2.3.2.9 Closures
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
    assert_eq!(15, f(5));
}

fn pred(v: u32) -> Option<u32> {
    if v == 0 {
        None
    } else {
        Some(v - 1)
    }
}

fn mul2(x: u64) -> u64 {
    x * 2
}
