//! Section 2.3.1 Rust Types, p28
#[derive(Debug)]
pub enum Sex {
    Unknown,
    Male,
    Female,
    NotApplicable,
}

#[derive(Debug)]
pub enum Role {
    Player(u32, u64),
    Supporter(u32),
}

#[derive(Debug)]
pub struct Person {
    pub age: u16,
    pub sex: Sex,
    pub role: Role,
}

#[derive(Debug)]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}
