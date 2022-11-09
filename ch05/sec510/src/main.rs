//! Sync and Async Echo Server
use sec510::server;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::thread::{sleep, spawn};
use std::time::Duration;

const NR_ADDR: &str = "127.0.0.1";
const NR_TIMEOUT: Duration = Duration::from_secs(10);
const NR_SERVERS: usize = 2;
const NR_PORT_BASE: u16 = 5000;
const NR_ECHO_COUNT: usize = 1;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let addr = args
        .next()
        .as_ref()
        .and_then(|v| IpAddr::from_str(v).ok())
        .unwrap_or_else(|| IpAddr::from_str(NR_ADDR).unwrap());
    let timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_TIMEOUT);
    let nr_servers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_SERVERS);
    let nr_port_base = args
        .next()
        .as_ref()
        .and_then(|v| u16::from_str(v).ok())
        .unwrap_or(NR_PORT_BASE);
    let nr_echo_count = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_ECHO_COUNT);

    println!(
        "{:?}: Sync echo servers on {}:({}..{}) with {:?} timeout",
        progname.file_name().unwrap(),
        addr,
        nr_port_base,
        nr_port_base + nr_servers as u16,
        timeout,
    );

    let _ = spawn(move || {
        sleep(timeout);
        eprintln!(
            "{:?}: timing out after {timeout:?}",
            progname.file_name().unwrap(),
        );
        exit(1);
    });

    let mut servers = HashMap::new();
    (0..nr_servers).for_each(|i| {
        let port = nr_port_base + i as u16;
        let addr = SocketAddr::new(addr, port);
        let count = nr_echo_count;
        servers.insert(port, spawn(move || server(addr, count)));
    });
    for (port, server) in servers.drain() {
        match server.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, port),
            },
        }
    }
}
