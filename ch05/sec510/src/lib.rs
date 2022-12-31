//! Sync echo server
#![forbid(unsafe_code, unstable_features)]
#![warn(missing_debug_implementations)]
use std::fmt::Debug;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, ToSocketAddrs};
use std::thread;
use tracing::{error, instrument};

/// serves the echo protocol through thread.
#[instrument]
pub fn serve(addr: impl ToSocketAddrs + Debug) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    let mut workers = vec![];
    for result in listener.incoming() {
        workers.push(thread::spawn(|| -> io::Result<()> {
            let socket = result?;
            let mut writer = BufWriter::new(socket.try_clone()?);
            let reader = BufReader::new(socket);

            for result in reader.lines() {
                let mut line = result?;
                line.push('\n');
                writer.write_all(line.as_bytes())?;
                writer.flush()?;
            }
            Ok(())
        }));
    }
    // it won't come here unless TcpListener connection closed somehow.
    for worker in workers {
        match worker.join() {
            Err(e) => error!("worker panic: {e:?}"),
            Ok(result) => if let Err(e) = result {
                error!("worker error: {e}");
            },
        }
    }
    Ok(())
}
