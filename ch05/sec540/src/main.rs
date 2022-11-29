//! 5.4.0 Async Echo server with tokio
use sec540::Server;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread::spawn;
use std::time::Duration;
use tracing::{error, info};

const NR_TIMEOUT: Duration = Duration::from_secs(10);
const NR_LISTEN_ADDR: &str = "127.0.0.1";
const NR_LISTEN_PORT_BASE: u16 = 60000;
const NR_LISTENERS: usize = 10;
const NR_SPAWNERS: usize = 3;

fn main() {
    tracing_subscriber::fmt::init();
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_TIMEOUT);
    let nr_addr = args
        .next()
        .as_ref()
        .and_then(|v| IpAddr::from_str(v).ok())
        .unwrap_or_else(|| IpAddr::from_str(NR_LISTEN_ADDR).unwrap());
    let nr_port_base = args
        .next()
        .as_ref()
        .and_then(|v| u16::from_str(v).ok())
        .unwrap_or(NR_LISTEN_PORT_BASE);
    let nr_listeners = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_LISTENERS);
    let nr_spawners = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SPAWNERS);

    info!(
        "{:?}: listen on {}:{}..{} with {:?} timeout",
        progname.file_name().unwrap(),
        nr_addr,
        nr_port_base,
        nr_port_base + nr_listeners as u16,
        nr_timeout,
    );

    let done0 = Arc::new(AtomicBool::new(false));
    let mut spawners = vec![];
    (0..nr_listeners)
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(nr_listeners / nr_spawners + 1)
        .map(Vec::<_>::from)
        .for_each(|ports| {
            let done1 = done0.clone();
            spawners.push(spawn(move || {
                for port in ports {
                    let addr = SocketAddr::new(nr_addr, port as u16);
                    let server = match Server::bind(addr) {
                        Err(e) => {
                            error!(error = %e, "server creation");
                            return;
                        }
                        Ok(server) => server,
                    };
                    let done = done1.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.run(done).await {
                            error!(error = %e, "server execution");
                        }
                    });
                }
            }));
        });

    for spawner in spawners.drain(..) {
        if let Err(e) = spawner.join() {
            error!(error = ?e, "spawner crash");
        }
    }
}
