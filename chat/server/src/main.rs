//! Chat Server
use async_std::task::block_on;
use server::server;
use std::path::PathBuf;

const ADDR: &str = "localhost:8080";

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let addr = args.next().unwrap_or_else(|| String::from(ADDR));

    if let Err(e) = block_on(server(addr)) {
        eprintln!("{:?}: {e}", progname.file_name().unwrap());
    }
}
