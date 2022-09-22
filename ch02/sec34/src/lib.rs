//! 2.3.4 Lifetimes
pub struct Foo {
    pub v: u32,
}

pub fn add<'a>(a: &'a Foo, b: &'a Foo) -> u32 {
    a.v + b.v
}
