//! Sync Echo Server
use std::error::Error;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, ToSocketAddrs};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub fn server<A: ToSocketAddrs>(a: A, max: usize) -> Result<u16> {
    let l = TcpListener::bind(a)?;
    let port = l.local_addr()?.port();

    // takes max connections.
    for _ in 0..max {
        let (s, _remote) = l.accept()?;
        let mut tx = BufWriter::new(s.try_clone()?);
        for line in BufReader::new(s).lines() {
            let mut buf = line?;
            // lines() drops the line delimiter
            buf.push('\n');
            tx.write_all(buf.as_bytes())?;
            tx.flush()?;
        }
    }
    Ok(port)
}
