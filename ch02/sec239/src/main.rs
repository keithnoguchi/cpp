use std::thread::spawn;

fn main() {
    if let Err(e) = spawn(hello).join() {
        eprintln!("function thread join error: {e:?}");
    }

    let h = || println!("hello world from closure");
    if let Err(e) = spawn(h).join() {
        eprintln!("closure thread join error: {e:?}");
    }
}

fn hello() {
    println!("hello world from function");
}
