//! I/O Selector with epoll(7)
use crate::{Listener, Result, Selector};
use std::fmt::Debug;
use std::io::Write;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tracing::{info, instrument};

pub struct Server {
    listener: Listener,
}

impl Server {
    #[instrument(name = "Server::new", skip(selector), err)]
    pub fn new<A>(addr: A, selector: Arc<Selector>) -> Result<Self>
    where
        A: ToSocketAddrs + Debug,
    {
        let listener = Listener::bind(addr, selector)?;
        Ok(Self { listener })
    }

    #[instrument(name = "Server::run", skip(self), err)]
    pub async fn run(&self) -> Result<()> {
        let (mut tx, mut rx, addr) = self.listener.accept().await?;
        info!(remote = %addr, "request received");
        while let Some(line) = rx.read_line().await {
            info!(remote = %addr, msg = %line, "message received");
            let _n = tx.write(line.as_bytes())?;
            tx.flush()?;
        }
        Ok(())
    }
}
