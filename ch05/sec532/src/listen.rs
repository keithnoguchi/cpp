//! 5.3.2 I/O Selector with epoll(7)
use crate::{Result, Selector};
use nix::sys::epoll::EpollFlags;
use std::future::Future;
use std::io::ErrorKind::WouldBlock;
use std::net::{TcpListener, ToSocketAddrs};
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct Listener {
    internal: TcpListener,
    selector: Arc<Selector>,
}

impl Listener {
    pub fn bind<A>(addrs: A, selector: Arc<Selector>) -> Result<Self>
    where
        A: ToSocketAddrs,
    {
        let internal = TcpListener::bind(addrs)?;
        internal.set_nonblocking(true)?;
        Ok(Self { internal, selector })
    }

    pub fn accept(&self) -> Acceptor {
        Acceptor { listener: self }
    }
}

pub struct Acceptor<'a> {
    listener: &'a Listener,
}

impl<'a> Future for Acceptor<'a> {
    type Output = Result<String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.internal.accept() {
            Ok(_) => todo!(),
            Err(e) if e.kind() == WouldBlock => {
                match self.listener.selector.register(
                    EpollFlags::EPOLLIN,
                    self.listener.internal.as_raw_fd(),
                    cx.waker().clone(),
                ) {
                    Err(e) => Poll::Ready(Err(e)),
                    Ok(_) => Poll::Pending,
                }
            }
            Err(e) => Poll::Ready(Err(e)?),
        }
    }
}
