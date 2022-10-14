//! Outbound: A safe concurrent access to the connection.
use async_std::io::prelude::WriteExt;
use async_std::net::TcpStream;
use async_std::sync::Mutex;
use serde::Serialize;
use std::error::Error;
use std::result;

pub struct Outbound(Mutex<TcpStream>);

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

impl Outbound {
    pub fn new(s: TcpStream) -> Self {
        Self(Mutex::new(s))
    }

    pub async fn send<P: Serialize>(&self, packet: P) -> Result<()> {
        let mut s = self.0.lock().await;
        packet::send(&mut *s, packet).await?;
        s.flush().await?;
        Ok(())
    }
}
