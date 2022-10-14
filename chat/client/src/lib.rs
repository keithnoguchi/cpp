//! Asynchronous Chat Client
use async_std::io::prelude::BufReadExt;
use async_std::io::{self, BufReader};
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use protocol::Request;
use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub async fn sender(addr: &str) -> Result<()> {
    println!(
        "join <GROUP>\n\
         post <GROUP> MSG...\n\
         type Control-D to close the connection.",
    );
    let s = TcpStream::connect(addr).await?;
    let mut reader = BufReader::new(io::stdin()).lines();

    while let Some(line) = reader.next().await {
        let req = Request::try_from(line?)?;
        packet::send(s.clone(), req).await?;
    }
    Ok(())
}
