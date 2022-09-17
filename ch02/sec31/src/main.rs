//! Section 2.3.1 Rust Types, p.28
use sec31::{Pair, Person, Role, Sex};

fn main() {
    let mut person = Person {
        age: 18,
        sex: Sex::NotApplicable,
        role: Role::Player(1, 10000),
    };
    println!("I'm {} years old.", person.age);

    let pair = Pair {
        first: 1.0,
        second: 5.2,
    };
    println!(
        "Here is the generic pair, first value is {} and second value is {}.",
        pair.first, pair.second,
    );

    change_age(&mut person, 15);
    println!("Now I'm {} years old.", person.age);
}

fn change_age(person: &mut Person, age: u16) {
    person.age = age;
}
