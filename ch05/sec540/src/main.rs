//! 5.4.0 Async Echo server with tokio
use sec540::Server;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

const NR_TIMEOUT: Duration = Duration::from_secs(10);
const NR_LISTEN_ADDR: &str = "127.0.0.1";
const NR_LISTEN_PORT_BASE: u16 = 60000;
const NR_LISTENERS: usize = 10;

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

    info!(
        "{:?}: listen on {}:{}..{} with {:?} timeout",
        progname.file_name().unwrap(),
        nr_addr,
        nr_port_base,
        nr_port_base + nr_listeners as u16,
        nr_timeout,
    );

    // tokio runtime
    let rt = match tokio::runtime::Runtime::new() {
        Err(e) => {
            error!(error = %e, "tokio runtime");
            exit(1);
        }
        Ok(rt) => rt,
    };

    let success0 = Arc::new(AtomicUsize::new(0));
    let success1 = success0.clone();
    rt.block_on(async move {
        // spawn async server tasks
        let done0 = Arc::new(AtomicBool::new(false));
        (0..nr_listeners)
            .map(|n| nr_port_base as usize + n)
            .for_each(|port| {
                let success = success1.clone();
                let done = done0.clone();
                tokio::spawn(async move {
                    let addr = SocketAddr::new(nr_addr, port as u16);
                    let server = match Server::bind(addr).await {
                        Err(e) => {
                            error!(error = %e, "server bind");
                            return;
                        }
                        Ok(server) => server,
                    };
                    if let Err(e) = server.run(done).await {
                        error!(error = %e, "server run");
                        return;
                    }
                    success.fetch_add(1, Relaxed);
                });
            });

        // timeout handling
        tokio::time::sleep(nr_timeout).await;
        done0.store(false, Relaxed);
    });
    assert_eq!(success0.load(Relaxed), nr_listeners);
}
