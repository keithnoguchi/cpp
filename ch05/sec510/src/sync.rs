//! Sync Echo Server
use crate::Result;
use std::net::{TcpListener, ToSocketAddrs};

pub fn echo<A: ToSocketAddrs>(a: A, max: usize) -> Result<u16> {
    let l = TcpListener::bind(a)?;
    let port = l.local_addr()?.port();

    let mut count = 0;
    loop {
        let (s, _remote) = l.accept()?;
        println!("{s:?}");
        count += 1;
        if count > max {
            break;
        }
    }
    Ok(port)
}
