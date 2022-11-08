//! Sync and Async Echo Server
use std::error::Error;
use std::result;

pub mod sync;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;
