//! Asynchronous Chat Client
use async_std::task;
use client::sender;
use client::Result;

const ADDR: &str = "::1:8080";

fn main() -> Result<()> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    task::block_on(sender(&addr))
}
