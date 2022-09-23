//! 2.3.8 ? Operator and unwrap Function
use sec38::{handle_option, handle_result};

fn main() {
    // ? operator on None.
    match handle_option::<f64>(None) {
        Ok(_) => panic!("no way!"),
        Err(e) => println!("{e}"),
    }

    // ? operator on Some(something).
    match handle_option(Some(5)) {
        Ok(r) => println!("{r}"),
        Err(e) => panic!("no way {e}"),
    };

    // ? operator on Err(&str).
    match handle_result::<f64, _>(Err("let's see how it goes")) {
        Ok(r) => panic!("no way!: {r}"),
        Err(e) => println!("{e}"),
    }

    // ? operator on Ok(something).
    match handle_result::<_, &str>(Ok(4.4)) {
        Ok(r) => println!("{r}"),
        Err(e) => panic!("no way!: {e}"),
    }
}
