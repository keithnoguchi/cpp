//! Section 2.3.2 Rust Grammer

/// 2.3.2.2 Functions
pub fn hello(n: u32) {
    println!("hello world: {n}");
}

pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

/// 2.3.2.3 If Expressions
pub fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

/// 2.3.2.4 Match Expressions
pub fn print_pred(pred: fn(u32) -> Option<u32>, x: u32) {
    match pred(x) {
        Some(y) => println!("pred({x}) -> {y}"),
        None => println!("pred({x}) is undefined"),
    }
}

/// 2.3.2.5 For Statements
pub fn even_odd(max: u32) {
    for x in 0..max {
        println!("{x} is {}", if is_even(x) { "even" } else { "odd" });
    }
}

/// 2.3.2.6 Loop Statements
pub fn even_odd2(max: u32) {
    let mut x = 0;
    loop {
        println!("{x} is {}", if is_even(x) { "even" } else { "odd" });
        x += 1;
        if x > max {
            break;
        }
    }
}

/// 2.3.2.7 References and Dereferences
pub fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y
}

/// 2.3.2.8 Function Pointers
pub fn app_n(f: fn(u64) -> u64, mut n: usize, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

/// 2.3.2.9 Closures
pub fn mul_x(x: u64) -> Box<dyn Fn(u64) -> u64> {
    Box::new(move |y| x * y)
}
