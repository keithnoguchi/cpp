//! Async Echo Server with epoll(7)/nix crate
use std::error::Error;
use std::net::{TcpListener, ToSocketAddrs};
use std::result;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub fn server<A: ToSocketAddrs>(a: A, timeout: Duration, _count: usize) -> Result<u16> {
    let l = TcpListener::bind(a)?;
    let addr = l.local_addr()?;
    let port = addr.port();

    sleep(timeout);
    Ok(port)
}
