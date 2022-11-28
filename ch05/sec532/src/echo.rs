//! I/O Selector with epoll(7)
use crate::{Listener, Result, Selector, Spawner};
use std::fmt::Debug;
use std::io::Write;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tracing::{debug, error, instrument, trace};

pub struct Server {
    listener: Listener,
    spawner: Spawner<()>,
}

impl Server {
    #[instrument(name = "Server::new", skip(selector, spawner), err)]
    pub fn new<A>(addr: A, selector: Arc<Selector>, spawner: Spawner<()>) -> Result<Self>
    where
        A: ToSocketAddrs + Debug,
    {
        let listener = Listener::bind(addr, selector)?;
        Ok(Self { listener, spawner })
    }

    #[instrument(name = "Server::run", skip(self), err)]
    pub async fn run(&self) -> Result<()> {
        loop {
            debug!("waiting for the connection");
            let (mut tx, mut rx, addr) = self.listener.accept().await?;
            debug!(remote = %addr, "request received");
            if let Err(e) = self.spawner.spawn(async move {
                while let Some(line) = rx.read_line().await {
                    trace!(remote = %addr, msg = %line, "message received");
                    if let Err(e) = tx.write(line.as_bytes()) {
                        error!(error = %e, "write error");
                    }
                    if let Err(e) = tx.flush() {
                        error!(error = %e, "write flush error");
                    }
                }
            }) {
                error!(error = %e, "spawn error");
            }
        }
    }
}
