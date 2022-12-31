//! epoll echo server
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
use smol::io::{BufReader, BufWriter};
use smol::net::AsyncToSocketAddrs as ToSocketAddrs;
use smol::net::TcpListener;
use smol::prelude::*;
use std::fmt::Debug;
use std::io;
use tracing::instrument;

/// asynchronously serves the echo protocol.
#[instrument]
pub async fn serve(addr: impl ToSocketAddrs + Debug) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let mut workers = vec![];
    while let Some(result) = listener.incoming().next().await {
        workers.push(smol::spawn(async move {
            let stream = result?;
            let mut reader = BufReader::new(stream.clone()).lines();
            let mut writer = BufWriter::new(stream);

            while let Some(result) = reader.next().await {
                let mut line = result?;
                line.push('\n');
                writer.write_all(line.as_bytes()).await?;
                writer.flush().await?;
            }
            Ok::<_, io::Error>(())
        }));
    }
    // this won't happen unless the tcp connection is closed somehow
    // above.
    for worker in workers {
        worker.await?;
    }
    Ok(())
}
