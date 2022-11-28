//! 5.4.0 Async Server with tokio
use std::error::Error;
use std::net::ToSocketAddrs;
use std::result;
use std::sync::{atomic::AtomicBool, Arc};

pub struct Server {}

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

impl Server {
    pub fn bind<A>(_addr: A) -> Result<Self>
    where
        A: ToSocketAddrs,
    {
        todo!()
    }

    pub async fn run(&self, _done: Arc<AtomicBool>) -> Result<()> {
        todo!()
    }
}
