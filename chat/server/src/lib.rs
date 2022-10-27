//! Chat Server
use async_std::io::{prelude::BufReadExt, BufReader};
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::spawn;
use connection::Outbound;
use group::Group;
use protocol::{Request, Response};
use state::Table;
use std::error::Error;
use std::fmt::Display;
use std::result;
use std::sync::Arc;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn server<A>(addr: A) -> Result<()>
where
    A: ToSocketAddrs + Display,
{
    let l = TcpListener::bind(&addr).await?;
    let kv0: Arc<Table<Group>> = Arc::new(Table::new());

    println!("listen on {addr}");

    while let Some(s) = l.incoming().next().await {
        let s = s?;
        let kv = kv0.clone();
        spawn(async {
            if let Err(e) = serve(s, kv).await {
                eprintln!("server: {e}");
            }
        });
    }
    Ok(())
}

async fn serve(s: TcpStream, kv: Arc<Table<Group>>) -> Result<()> {
    let tx = Arc::new(Outbound::new(s.clone()));

    let mut rx = BufReader::new(s).lines();
    while let Some(line) = rx.next().await {
        let result = match serde_json::from_str::<Request>(&line?)? {
            Request::Join { group_name } => {
                let group = kv.get_or_create(group_name);
                group.join(tx.clone());
                Ok(())
            }
            Request::Post {
                group_name,
                message,
            } => match kv.get(&group_name) {
                None => Err(format!("{group_name:?} doesn't exist")),
                Some(group) => {
                    group.post(message);
                    Ok(())
                }
            },
        };
        if let Err(e) = result {
            let resp = Response::Error(e);
            tx.send(resp).await?;
        }
    }
    Ok(())
}
