//! 5.4.2 Cancel Pattern with tokio broadcast Channel
use std::error::Error;
use std::result;
use std::sync::Arc;
use tokio::sync::{broadcast::Receiver, Mutex};
use tracing::{instrument, trace, warn};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[instrument(level = "trace", name = "worker", skip(cancel, counter), ret)]
pub async fn worker(
    mut cancel: Receiver<()>,
    counter: Arc<Mutex<usize>>,
    nr_loop: usize,
) -> Result<usize> {
    trace!("worker started");
    let mut worked = 0;
    loop {
        tokio::select! {
            _ = cancel.recv() => {
                warn!("worker canceled");
                break;
            }
            mut counter = counter.lock() => {
                *counter += 1;
                worked += 1;
                trace!("worker worked");
                if worked == nr_loop {
                    trace!("worker done");
                    break;
                }
            }
        }
    }
    Ok(worked)
}
