//! 5.3.2 I/O Selector with epoll(7)
use crate::{Reader, Result, Selector};
use nix::sys::epoll::EpollFlags;
use std::fmt::Debug;
use std::future::Future;
use std::io::{BufWriter, ErrorKind::WouldBlock};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tracing::{debug, instrument};

pub struct Listener {
    internal: TcpListener,
    selector: Arc<Selector>,
}

impl Listener {
    #[instrument(name = "Listener::bind", skip(selector), err)]
    pub fn bind<A>(addrs: A, selector: Arc<Selector>) -> Result<Self>
    where
        A: ToSocketAddrs + Debug,
    {
        let internal = TcpListener::bind(addrs)?;
        internal.set_nonblocking(true)?;
        Ok(Self { internal, selector })
    }

    #[instrument(name = "Listener::accept", skip(self))]
    pub fn accept(&self) -> Acceptor {
        Acceptor { listener: self }
    }
}

pub struct Acceptor<'a> {
    listener: &'a Listener,
}

impl<'a> Future for Acceptor<'a> {
    type Output = Result<(BufWriter<TcpStream>, Reader<TcpStream>, SocketAddr)>;

    #[instrument(name = "Acceptor::poll", skip(self, cx))]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        debug!("polling...");
        match self.listener.internal.accept() {
            Ok((s, addr)) => {
                let tx = match s.try_clone() {
                    Err(e) => return Poll::Ready(Err(e)?),
                    Ok(s) => BufWriter::new(s),
                };
                let rx = Reader::new(s, self.listener.selector.clone());
                Poll::Ready(Ok((tx, rx, addr)))
            }
            Err(e) if e.kind() == WouldBlock => {
                debug!("would block");
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
