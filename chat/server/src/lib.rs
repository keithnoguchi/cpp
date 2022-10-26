//! Chat Server
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::stream::StreamExt;
use async_std::task::spawn;
use group::Group;
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
    let db0: Arc<Table<Group>> = Arc::new(Table::new());

    println!("listen on {addr}");

    while let Some(s) = l.incoming().next().await {
        let s = s?;
        let db = db0.clone();
        spawn(serve(s, db));
    }
    Ok(())
}

async fn serve(_s: TcpStream, _db: Arc<Table<Group>>) -> Result<()> {
    todo!();
}
