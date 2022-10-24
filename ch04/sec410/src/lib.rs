//! 4.1.0 Deadlock
pub mod mutex;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error + Send>>;
