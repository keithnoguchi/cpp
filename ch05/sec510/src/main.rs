fn main() {
    if let Err(e) = sec510::server("127.0.0.1:8080") {
        eprintln!("{e}");
    }
}
