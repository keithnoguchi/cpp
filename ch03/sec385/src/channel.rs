//! 3.8.5 Channel
use crate::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone)]
pub struct Sender<T: Send> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cvar: Arc<Condvar>,
}

#[derive(Clone)]
pub struct Receiver<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cvar: Arc<Condvar>,
}

impl<T: Send> Sender<T> {
    pub fn send(&self, data: T) {
        self.sem.wait();
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data);
        // Notify in case someone waiting in Receiver::recv().
        self.cvar.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        let data = loop {
            match buf.pop_front() {
                Some(data) => break data,
                None => buf = self.cvar.wait(buf).unwrap(),
            }
        };
        self.sem.post();
        data
    }
}

pub fn channel<T: Send>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cvar = Arc::new(Condvar::new());
    let tx = Sender {
        sem: Arc::clone(&sem),
        buf: Arc::clone(&buf),
        cvar: Arc::clone(&cvar),
    };
    let rx = Receiver { sem, buf, cvar };
    (tx, rx)
}
