const ADDR: &str = "127.0.0.1:8080";

fn main() {
    tracing_subscriber::fmt::init();
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = smol::block_on(sec511::serve(addr)) {
        eprintln!("{e}");
    }
}
