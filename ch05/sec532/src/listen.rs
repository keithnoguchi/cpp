//! 5.3.2 I/O Selector with epoll(7)
use crate::{Result, Selector};
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;

pub struct Listener {
    _internal: TcpListener,
    _selector: Arc<Selector>,
}

impl Listener {
    pub fn bind<A>(addrs: A, selector: Arc<Selector>) -> Result<Self>
    where
        A: ToSocketAddrs,
    {
        let internal = TcpListener::bind(addrs)?;
        internal.set_nonblocking(true)?;
        Ok(Self {
            _internal: internal,
            _selector: selector,
        })
    }

    pub fn accept(&self) -> Acceptor {
        Acceptor { _listener: self }
    }
}

pub struct Acceptor<'a> {
    _listener: &'a Listener,
}
