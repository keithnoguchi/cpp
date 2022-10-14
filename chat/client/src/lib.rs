//! Asynchronous Chat Client
use async_std::io::prelude::BufReadExt;
use async_std::io::{self, BufReader};
use async_std::stream::StreamExt;
use protocol::Request;
use std::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub async fn sender(_addr: &str) -> Result<()> {
    println!(
        "join <GROUP>\n\
         post <GROUP> MSG...\n\
         type Control-D to close the connection.",
    );
    let mut reader = BufReader::new(io::stdin()).lines();

    while let Some(line) = reader.next().await {
        let line = line?;
        let req = match Request::try_from(&line[..]) {
            Err(e) => {
                eprintln!("{e}");
                continue;
            },
            Ok(req) => req,
        };
        println!("{req:?}");
    }
    Ok(())
}
