//! Decentralized Chat Client with libp2p
use async_std::task::block_on;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();

    if let Err(e) = block_on(dclient::run()) {
        eprintln!("{:?}: {e}", progname.file_name().unwrap());
    }
}
