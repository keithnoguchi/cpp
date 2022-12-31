const ADDR: &str = "127.0.0.1:8080";

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = sec510::serve(addr) {
        eprintln!("{e}");
    }
}
