//! Async Echo Server with epoll(7)/nix crate
use nix::sys::epoll::epoll_ctl;
use nix::sys::epoll::EpollOp::{EpollCtlAdd, EpollCtlDel};
use nix::sys::epoll::{epoll_create1, EpollCreateFlags};
use nix::sys::epoll::{epoll_wait, EpollEvent, EpollFlags};
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, ToSocketAddrs};
use std::os::unix::io::{AsRawFd, RawFd};
use std::result;
use std::time::Duration;

const NR_CONCURRENT_EVENTS: usize = 1024;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub fn server<A: ToSocketAddrs>(a: A, timeout: Duration, _count: usize) -> Result<u16> {
    let l = TcpListener::bind(a)?;
    let addr = l.local_addr()?;
    let port = addr.port();

    // register the accept events
    let efd = epoll_create1(EpollCreateFlags::empty())?;
    let lfd = l.as_raw_fd();
    let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, lfd as u64);
    epoll_ctl(efd, EpollCtlAdd, lfd, &mut ev)?;

    // event loop
    let mut clients = HashMap::new();
    let mut events = vec![EpollEvent::empty(); NR_CONCURRENT_EVENTS];
    let timeout = timeout.as_millis() as isize;
    loop {
        let nr = match epoll_wait(efd, &mut events, timeout)? {
            0 => break, // timed out
            nr => nr,
        };
        for ev in events.iter().take(nr) {
            match ev.data() as RawFd {
                fd if fd == lfd => {
                    let (s, _remote) = l.accept()?;

                    // register the events to epoll(7)
                    let fd = s.as_raw_fd();
                    let mut ev =
                        EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLET, fd as u64);
                    epoll_ctl(efd, EpollCtlAdd, fd, &mut ev)?;

                    // remember the client
                    let tx = BufWriter::new(s.try_clone()?);
                    let rx = BufReader::new(s);
                    clients.insert(fd, (tx, rx));
                }
                fd => {
                    let (tx, rx) = clients.get_mut(&fd).ok_or(format!("unexpected fd: {fd}"))?;

                    // XXX nonblocking support
                    let mut buf = String::new();
                    let n = rx.read_line(&mut buf)?;
                    if n != 0 {
                        tx.write_all(buf.as_bytes())?;
                        tx.flush()?;
                    } else {
                        epoll_ctl(efd, EpollCtlDel, fd, None)?;
                        clients.remove(&fd);
                    }
                }
            }
        }
    }
    Ok(port)
}
