//! Asynchronous Chat Client
use async_std::io::prelude::BufReadExt;
use async_std::io::{self, BufReader};
use async_std::stream::StreamExt;
use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub async fn sender(_addr: &str) -> Result<()> {
    let mut reader = BufReader::new(io::stdin()).lines();

    while let Some(line) = reader.next().await {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}
