//! Asynchronous Echo Server
const ADDR: &str = "localhost:8080";

fn main() {
    tracing_subscriber::fmt::init();
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = async_io::block_on(sec511::serve(addr)) {
        eprintln!("{e}");
    }
}
