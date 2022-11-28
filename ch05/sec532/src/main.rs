//! 5.3.2 I/O Selector with epoll(7)
use sec532::{Executor, Selector, Server};
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;
use tracing::{error, info};

const NR_TIMEOUT: Duration = Duration::from_secs(30);
const NR_LISTEN_ADDR: &str = "127.0.0.1";
const NR_LISTEN_PORT_BASE: u16 = 40_000;
const NR_LISTENERS: usize = 1;
const NR_SPAWNERS: usize = 3;
const NR_RUN_QUEUE_BOUND: usize = 2048;

fn main() {
    let mut args = std::env::args();
    let _progname = args.next().map(PathBuf::from).unwrap();
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
    let nr_run_queue_bound = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RUN_QUEUE_BOUND);

    tracing_subscriber::fmt()
        .with_level(true)
        .with_target(false)
        .without_time()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .init();

    info!(
        listeners = %nr_listeners,
        addr = %nr_addr,
        port_min = %nr_port_base,
        port_max = %(nr_port_base + nr_listeners as u16),
        timeout = ?nr_timeout,
        "echo server",
    );

    // spawn selector.
    let mut workers = vec![];
    let selector0 = match Selector::new() {
        Ok(selector) => Arc::new(selector),
        Err(e) => {
            error!(error = %e, "Selector::new() failure");
            exit(1);
        }
    };
    let selector = selector0.clone();
    workers.push(spawn(move || selector.select(nr_timeout)));

    // spawn executor
    let executor = Executor::new(nr_run_queue_bound);
    let spawner0 = executor.spawner();
    workers.push(spawn(move || executor.run()));

    // spawn listeners
    let counter0 = Arc::new(AtomicUsize::new(0));
    (0..nr_listeners)
        .into_iter()
        .map(|port| SocketAddr::new(nr_addr, nr_port_base + port as u16))
        .collect::<Vec<_>>()
        .chunks(nr_listeners / nr_spawners + 1)
        .map(Vec::<_>::from)
        .for_each(|addrs| {
            let spawner = spawner0.clone();
            let selector1 = selector0.clone();
            let counter1 = counter0.clone();
            workers.push(spawn(move || {
                for addr in addrs {
                    let counter = counter1.clone();
                    let selector = selector1.clone();
                    if let Err(e) = spawner.spawn(async move {
                        let server = match Server::new(addr, selector) {
                            Err(e) => panic!("{e}"),
                            Ok(server) => server,
                        };
                        if let Err(e) = server.run().await {
                            panic!("{e}");
                        }
                        counter.fetch_add(1, Relaxed);
                    }) {
                        panic!("{e}");
                    }
                }
                Ok(())
            }));
        });

    // dropping spawner here to let executor done with the job
    drop(spawner0);
    for worker in workers.drain(..) {
        if let Err(e) = worker.join() {
            error!(error = ?e, "worker thread panic");
        }
    }
    assert_eq!(counter0.load(Relaxed), nr_listeners);
}
