//! Asynchronous Chat Client
use async_std::task;
use client::sender;
use client::Result;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let progname = args.next().unwrap();
    let addr = args
        .next()
        .unwrap_or_else(|| panic!("Usage: {progname} ADDR"));

    task::block_on(sender(&addr))
}
