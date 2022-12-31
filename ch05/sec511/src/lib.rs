//! epoll echo server
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
use smol::io::{BufReader, BufWriter};
use smol::net::AsyncToSocketAddrs as ToSocketAddrs;
use smol::net::TcpListener;
use smol::prelude::*;
use std::io;

/// asynchronously serves the echo protocol.
pub async fn serve(addr: impl ToSocketAddrs) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    while let Some(result) = listener.incoming().next().await {
        let stream = result?;

        let mut reader = BufReader::new(stream.clone()).lines();
        let mut writer = BufWriter::new(stream);

        while let Some(result) = reader.next().await {
            let mut line = result?;
            line.push('\n');
            writer.write_all(line.as_bytes()).await?;
            writer.flush().await?;
        }
    }
    Ok(())
}
