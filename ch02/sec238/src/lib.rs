//! 2.3.8 ? Operator and unwrap Function
use std::error::Error;

pub fn handle_option<T>(input: Option<T>) -> Result<T, Box<dyn Error>> {
    Ok(input.ok_or("yep, it's None")?)
}

#[allow(clippy::needless_question_mark)]
pub fn handle_result<T, E>(input: Result<T, E>) -> Result<T, E> {
    Ok(input?)
}
