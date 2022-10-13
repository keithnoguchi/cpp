//! Chapter 20 Asynchronous Programming Packet
use async_std::io::{prelude::BufReadExt, BufRead, Write, WriteExt};
use async_std::stream::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn send<S, P>(mut s: S, p: P) -> Result<()>
where
    S: Write + Unpin,
    P: Serialize,
{
    let mut packet = serde_json::to_string(&p)?;
    packet.push('\n');
    s.write_all(packet.as_bytes()).await?;
    Ok(())
}

pub fn recv<S, P>(s: S) -> impl Stream<Item = Result<P>>
where
    S: BufRead + Unpin,
    P: DeserializeOwned,
{
    s.lines().map(|packet| -> Result<P> {
        let packet = packet?;
        let parsed = serde_json::from_str::<P>(&packet)?;
        Ok(parsed)
    })
}
