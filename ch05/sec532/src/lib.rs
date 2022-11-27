//! 5.3.2 I/O Selector with epoll(7)
mod echo;
mod listen;
mod read;
mod task;

pub use crate::echo::Server;
pub use crate::task::Executor;

pub(crate) use crate::listen::Listener;
pub(crate) use crate::read::Reader;

use nix::errno::Errno;
use nix::sys::epoll::{epoll_create1, EpollCreateFlags};
use nix::sys::epoll::{epoll_ctl, EpollOp};
use nix::sys::epoll::{epoll_wait, EpollEvent, EpollFlags};
use nix::sys::eventfd::{eventfd, EfdFlags};
use nix::unistd::{read, write};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::os::unix::io::RawFd;
use std::result;
use std::sync::Mutex;
use std::task::Waker;
use std::time::Duration;
use tracing::instrument;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

// Maximum number of monitoring events in a single epoll_wait(2) call.
const NR_MAX_MONITORING_EVENTS: usize = 24;

pub struct Selector {
    efd: RawFd,
    epfd: RawFd,
    queue: Mutex<VecDeque<Ops>>,
    wakers: Mutex<HashMap<RawFd, Waker>>,
}

enum Ops {
    Add(EpollFlags, RawFd, Waker),
    Del(RawFd),
}

impl Selector {
    #[instrument(name = "Selector::new", err)]
    pub fn new() -> Result<Self> {
        let efd = eventfd(0, EfdFlags::empty())?;
        let epfd = epoll_create1(EpollCreateFlags::empty())?;
        let queue = Mutex::new(VecDeque::new());
        let wakers = Mutex::new(HashMap::new());
        Ok(Self {
            efd,
            epfd,
            queue,
            wakers,
        })
    }

    #[instrument(name = "Selector::select", skip(self), err)]
    pub fn select(&self, timeout: Duration) -> Result<()> {
        // register eventfd
        let mut e = EpollEvent::new(EpollFlags::EPOLLIN, self.efd as u64);
        epoll_ctl(self.epfd, EpollOp::EpollCtlAdd, self.efd, &mut e)?;

        // event loop
        let timeout = timeout.as_millis() as isize;
        let mut events = [EpollEvent::empty(); NR_MAX_MONITORING_EVENTS];
        loop {
            let nfd = epoll_wait(self.epfd, &mut events, timeout)?;
            // time out
            if nfd == 0 {
                break;
            }
            for e in events.iter().take(nfd) {
                if e.data() == self.efd as u64 {
                    // eventfd event
                    let mut q = self.queue.lock().unwrap();
                    while let Some(e) = q.pop_front() {
                        match e {
                            Ops::Add(flags, fd, waker) => self.add_event(flags, fd, waker)?,
                            Ops::Del(fd) => self.del_event(fd)?,
                        }
                    }
                    // flush the eventfd buffer to avoid the periodic
                    // wake ups.
                    let mut buf = [0u8; 1];
                    Self::read_event(self.efd, &mut buf)?;
                } else {
                    // i/o event
                    let fd = e.data() as i32;
                    let mut wakers = self.wakers.lock().unwrap();
                    if let Some(waker) = wakers.remove(&fd) {
                        waker.wake_by_ref()
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(name = "Selector::register", skip(self), err)]
    pub fn register(&self, flags: EpollFlags, fd: RawFd, waker: Waker) -> Result<()> {
        let mut q = self.queue.lock().unwrap();
        q.push_back(Ops::Add(flags, fd, waker));
        Self::write_event(self.efd, 1)?;
        Ok(())
    }

    #[instrument(name = "Selector::unregister", skip(self), err)]
    pub fn unregister(&self, fd: RawFd) -> Result<()> {
        let mut q = self.queue.lock().unwrap();
        q.push_back(Ops::Del(fd));
        Self::write_event(self.efd, 1)?;
        Ok(())
    }

    #[instrument(name = "Selector::add_event", skip(self), err)]
    fn add_event(&self, mut flags: EpollFlags, fd: RawFd, waker: Waker) -> Result<()> {
        flags |= EpollFlags::EPOLLONESHOT;
        let mut e = EpollEvent::new(flags, fd as u64);
        match epoll_ctl(self.epfd, EpollOp::EpollCtlAdd, fd, &mut e) {
            // update the event in case of the the event is there.
            Err(Errno::EEXIST) => epoll_ctl(self.epfd, EpollOp::EpollCtlMod, fd, &mut e)?,
            Err(e) => Err(e)?,
            Ok(_) => (),
        }
        let mut wakers = self.wakers.lock().unwrap();
        if wakers.insert(fd, waker).is_some() {
            Err(format!("duplicate event addition: fd({fd})"))?;
        }
        Ok(())
    }

    #[instrument(name = "Selector::del_event", skip(self), err)]
    fn del_event(&self, fd: RawFd) -> Result<()> {
        let mut e = EpollEvent::new(EpollFlags::empty(), fd as u64);
        epoll_ctl(self.epfd, EpollOp::EpollCtlDel, fd, &mut e)?;
        let mut wakers = self.wakers.lock().unwrap();
        wakers
            .remove(&fd)
            .ok_or(format!("missing event deletion: fd({fd})"))?;
        Ok(())
    }

    #[instrument(name = "Selector::read_event", err)]
    fn read_event(fd: RawFd, buf: &mut [u8; 1]) -> Result<()> {
        read(fd, buf)?;
        Ok(())
    }

    #[instrument(name = "Selector::write_event", err)]
    fn write_event(fd: RawFd, n: usize) -> Result<()> {
        let ptr = &n as *const usize as *const u8;
        let val = unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of_val(&n)) };
        write(fd, val)?;
        Ok(())
    }
}
