//! 5.2.1 Coroutine with Future Trait
use futures::future::{BoxFuture, FutureExt};
use sec521::Hello;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Mutex;

const NR_TASKS: usize = 10_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_tasks = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TASKS);

    println!("{:?}: {} tasks", progname.file_name().unwrap(), nr_tasks);

    let mut tasks = vec![];
    (0..nr_tasks).for_each(|_| tasks.push(Task::new()));
}

struct Task {
    _coroutine: Mutex<BoxFuture<'static, ()>>,
}

static NR_TASK_ID: AtomicU64 = AtomicU64::new(0);

impl Task {
    fn new() -> Self {
        let hello = Hello::new(NR_TASK_ID.fetch_add(1, Relaxed));
        Self {
            _coroutine: Mutex::new(hello.boxed()),
        }
    }
}
