//! 2.3.3 Ownership
#[derive(Debug)]
pub struct Apple {}

#[derive(Debug)]
pub struct Gold {}

#[derive(Debug)]
pub struct FullStomach {}

pub fn get_gold(_apple: Apple) -> Gold {
    Gold {}
}

pub fn get_full_stomach(_apple: Apple) -> FullStomach {
    FullStomach {}
}
