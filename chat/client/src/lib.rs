//! Asynchronous Chat Client
use async_std::io::prelude::BufReadExt;
use async_std::io::{stdin, BufReader};
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use async_std::task::sleep;
use futures_lite::FutureExt;
use protocol::Request;
use std::error::Error;
use std::result;
use std::time::Duration;

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub async fn run(addr: &str) -> Result<()> {
    let s = TcpStream::connect(addr).await?;

    sender(s.clone()).race(receiver(s)).await
}

async fn receiver(_s: TcpStream) -> Result<()> {
    loop {
        sleep(Duration::from_millis(100)).await;
    }
}

async fn sender(s: TcpStream) -> Result<()> {
    println!(
        "join <GROUP>\n\
         post <GROUP> MSG...\n\
         type Control-D to close the connection.",
    );

    let mut rx = BufReader::new(stdin()).lines();
    while let Some(line) = rx.next().await {
        let req = Request::try_from(line?)?;
        packet::send(&s, req).await?;
    }
    Ok(())
}
