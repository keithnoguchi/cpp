//! Sync echo server
#![forbid(unsafe_code, unstable_features)]
#![warn(missing_debug_implementations)]
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, ToSocketAddrs};

pub fn server(addr: impl ToSocketAddrs) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for result in listener.incoming() {
        let socket = result?;
        let mut writer = BufWriter::new(socket.try_clone()?);
        let reader = BufReader::new(socket);

        for result in reader.lines() {
            let mut line = result?;
            line.push('\n');
            writer.write_all(line.as_bytes())?;
            writer.flush()?;
        }
    }

    Ok(())
}
