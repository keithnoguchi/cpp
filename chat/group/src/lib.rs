//! Chat group crate
use async_std::task::spawn;
use connection::Outbound;
use protocol::Response;
use state::Entry;
use std::sync::Arc;
use tokio::sync::broadcast::error::{RecvError, SendError};
use tokio::sync::broadcast::{channel, Receiver, Sender};

const NR_CHANNEL_BOUNDED_SIZE: usize = 1_000;

type Message = Arc<String>;

#[derive(Clone)]
pub struct Group {
    name: Arc<String>,
    tx: Sender<Message>,
}

impl Entry for Group {
    fn new(name: Arc<String>) -> Self {
        let (tx, _rx) = channel(NR_CHANNEL_BOUNDED_SIZE);
        Self { name, tx }
    }
}

impl Group {
    pub fn join(&self, tx: Arc<Outbound>) {
        spawn(receiver(self.name.clone(), tx, self.tx.subscribe()));
    }

    pub fn post(&self, msg: Message) {
        match self.tx.send(msg) {
            Err(SendError(_)) => eprintln!("{}: no active friends", self.name),
            Ok(n) => println!("{}: sent to {n} friends", self.name),
        }
    }
}

async fn receiver(group_name: Arc<String>, tx: Arc<Outbound>, mut rx: Receiver<Message>) {
    loop {
        let msg = match rx.recv().await {
            Err(RecvError::Closed) => break,
            Err(RecvError::Lagged(n)) => panic!("{group_name}: lagged {n}"),
            Ok(msg) => Response::Message {
                group_name: group_name.clone(),
                message: msg,
            },
        };
        if let Err(e) = tx.send(msg).await {
            // let's just panic for now.
            panic!("{group_name}: {e}");
        }
    }
}
