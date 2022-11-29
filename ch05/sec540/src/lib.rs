//! 5.4.0 Async Server with tokio
use std::error::Error;
use std::fmt::Debug;
use std::result;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, ToSocketAddrs};
use tracing::{debug, error, instrument};

pub struct Server {
    listener: TcpListener,
}

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

impl Server {
    #[instrument(name = "Server::bind", err)]
    pub async fn bind<A>(addr: A) -> Result<Self>
    where
        A: ToSocketAddrs + Debug,
    {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }

    #[instrument(name = "Server::run", skip(self), err)]
    pub async fn run(&self, done: Arc<AtomicBool>) -> Result<()> {
        loop {
            if done.load(Relaxed) {
                break;
            }
            let (mut s, remote) = self.listener.accept().await?;
            debug!(%remote, "accepted");
            tokio::spawn(async move {
                let (rx, tx) = s.split();
                let mut rx = BufReader::new(rx);
                let mut tx = BufWriter::new(tx);
                let mut buf = String::new();
                loop {
                    buf.clear();
                    if let Err(e) = rx.read_line(&mut buf).await {
                        error!(error = %e, "read_line");
                        break;
                    };
                    debug!(%buf, "receive line");
                    if let Err(e) = tx.write_all(buf.as_bytes()).await {
                        error!(error = %e, "write_all");
                        break;
                    }
                    if let Err(e) = tx.flush().await {
                        error!(error = %e, "flush");
                        break;
                    }
                }
            });
        }
        Ok(())
    }
}
