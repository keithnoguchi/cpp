//! Asynchronous Chat Client
use async_std::task;
use std::path::PathBuf;

const ADDR: &str = "localhost:8080";

fn main() -> client::Result<()> {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let addr = args.next().unwrap_or_else(|| ADDR.to_string());

    println!("{:?}: connect to {addr}", progname.file_name().unwrap());

    task::block_on(client::run(&addr))
}
