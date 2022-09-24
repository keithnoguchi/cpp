//! 2.3.5 Borrow Checker
pub struct Foo {
    pub v: u32,
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}
