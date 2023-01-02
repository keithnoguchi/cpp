//! Asynchronous Echo Service
#![forbid(unsafe_code, missing_docs, missing_debug_implementations)]
#![warn(missing_copy_implementations)]
use async_net::AsyncToSocketAddrs as ToSocketAddrs;
use async_net::TcpListener;
use futures_lite::io::{BufReader, BufWriter};
use futures_lite::{AsyncBufReadExt, AsyncWriteExt, StreamExt};
use std::fmt::Debug;
use std::io;
use tracing::{debug, instrument};

const MAX_CONNECTIONS: usize = 5; // for the cleanup handling.

/// Asynchronously serve the echo service.
#[instrument]
pub async fn serve(addr: impl ToSocketAddrs + Debug) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let mut workers = vec![];
    while let Some(result) = listener.incoming().next().await {
        debug!(
            workers.len = %workers.len(),
            workers.max = %MAX_CONNECTIONS,
            "incoming connection",
        );
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
        if workers.len() >= MAX_CONNECTIONS {
            break;
        }
    }
    // cleanup.
    for worker in workers {
        worker.await?;
    }
    Ok(())
}
