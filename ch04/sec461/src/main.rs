//! 4.6.1 Signal Handling in Rust
use sec461::handler;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use std::thread::spawn;
use std::time::Duration;

const NR_HANDLERS: usize = 5;
const NR_TIMEOUT: Duration = Duration::from_secs(10);

fn main() {
    let mut args = std::env::args();
    let progname = args.next().as_ref().map(PathBuf::from).unwrap();
    let nr_handlers = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_HANDLERS);
    let timeout = args
        .next()
        .as_ref()
        .and_then(|v| u64::from_str(v).ok())
        .map(Duration::from_secs)
        .unwrap_or(NR_TIMEOUT);

    println!(
        "{:?}(pid={}): {} signal handlers with {:?} timeout.",
        progname.file_name().unwrap(),
        process::id(),
        nr_handlers,
        timeout,
    );

    let mut handlers = vec![];
    (0..nr_handlers).for_each(|id| {
        handlers.push(spawn(move || handler(id as u64, timeout)));
    });

    for (id, handler) in handlers.drain(..).enumerate() {
        match handler.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
}
