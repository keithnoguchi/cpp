//! epoll echo server
#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]
use std::io::{self, prelude::*, BufRead, BufReader, BufWriter};
use std::net::{TcpListener, ToSocketAddrs};

/// serves the echo protocol.
pub fn serve(addr: impl ToSocketAddrs) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for result in listener.incoming() {
        let stream = result?;

        let reader = BufReader::new(stream.try_clone()?);
        let mut writer = BufWriter::new(stream);

        for result in reader.lines() {
            let mut line = result?;
            line.push('\n');
            writer.write_all(line.as_bytes())?;
            writer.flush()?;
        }
    }
    Ok(())
}
