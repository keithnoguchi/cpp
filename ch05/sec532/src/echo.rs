//! I/O Selector with epoll(7)
use crate::{Listener, Result, Selector};
use std::io::Write;
use std::net::ToSocketAddrs;
use std::sync::Arc;

pub struct Server {
    listener: Listener,
}

impl Server {
    pub fn new<A>(addr: A, selector: Arc<Selector>) -> Result<Self>
    where
        A: ToSocketAddrs,
    {
        let listener = Listener::bind(addr, selector)?;
        Ok(Self { listener })
    }

    pub async fn run(&self) -> Result<()> {
        let (mut tx, mut rx, addr) = self.listener.accept().await?;
        println!("incoming from {addr}");
        while let Some(line) = rx.read_line().await {
            let _n = tx.write(line.as_bytes())?;
            tx.flush()?;
        }
        Ok(())
    }
}
