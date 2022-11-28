//! 5.3.2 I/O Selector with epoll(7)
use crate::Selector;
use nix::sys::epoll::EpollFlags;
use std::future::Future;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tracing::{debug, instrument};

pub struct Reader<R: Read + AsRawFd> {
    fd: RawFd,
    reader: BufReader<R>,
    selector: Arc<Selector>,
}

impl<R: Read + AsRawFd> Drop for Reader<R> {
    fn drop(&mut self) {
        if let Err(e) = self.selector.unregister(self.fd) {
            panic!("{e}");
        }
    }
}

impl<R: Read + AsRawFd> Reader<R> {
    #[instrument(name = "Reader::new", skip(s, selector))]
    pub fn new(s: R, selector: Arc<Selector>) -> Self {
        let fd = s.as_raw_fd();
        let reader = BufReader::new(s);
        Self {
            fd,
            reader,
            selector,
        }
    }

    #[instrument(name = "Reaer::read_line", skip(self))]
    pub fn read_line(&mut self) -> ReadLiner<R> {
        ReadLiner { internal: self }
    }
}

pub struct ReadLiner<'a, R: Read + AsRawFd> {
    internal: &'a mut Reader<R>,
}

impl<'a, R: Read + AsRawFd> Future for ReadLiner<'a, R> {
    type Output = Option<String>;

    #[instrument(name = "ReadLiner::poll", skip(self, cx))]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        debug!("polling...");
        let mut line = String::new();
        match self.internal.reader.read_line(&mut line) {
            Ok(0) => Poll::Ready(None),
            Ok(_) => Poll::Ready(Some(line)),
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                debug!("blocking");
                if let Err(e) = self.internal.selector.register(
                    EpollFlags::EPOLLIN,
                    self.internal.fd,
                    cx.waker().clone(),
                ) {
                    panic!("{e}");
                }
                Poll::Pending
            }
            Err(e) => panic!("{e}"),
        }
    }
}
