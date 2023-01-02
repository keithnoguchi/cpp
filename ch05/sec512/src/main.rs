//! Async Echo Server
use std::fmt::Debug;
use std::{env, io};

use futures_lite::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use futures_lite::stream::StreamExt;
use tracing::{error, instrument};

use async_net::AsyncToSocketAddrs as ToSocketAddrs;
use async_net::{TcpListener, TcpStream};

const ADDR: &str = "localhost:8080";

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();

    let addr = env::args().nth(1).unwrap_or_else(|| ADDR.to_string());
    if let Err(e) = async_io::block_on(listener(addr)) {
        error!("{e}");
    }
}

#[instrument]
async fn listener(addr: impl ToSocketAddrs + Debug) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut workers = vec![];

    while let Some(result) = listener.incoming().next().await {
        workers.push(sec512::spawn(async move {
            let stream = result?;
            copy(stream).await
        }));
    }
    for worker in workers {
        if let Err(e) = worker.await {
            error!("{e}");
        }
    }
    Ok(())
}

#[instrument]
async fn copy(stream: TcpStream) -> io::Result<()> {
    let mut writer = BufWriter::new(stream.clone());

    let mut stream = BufReader::new(stream).lines();
    while let Some(result) = stream.next().await {
        let mut line = result?;
        line.push('\n');
        writer.write_all(line.as_bytes()).await?;
        writer.flush().await?;
    }
    Ok(())
}
