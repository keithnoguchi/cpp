//! 5.3.1 Future by async/await
use sec531::{Executor, Hello};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread::spawn;

const NR_TASKS: usize = 1_024;
const NR_SPAWNERS: usize = 20;
const NR_RUN_QUEUE_BOUND: usize = 2_048;

static COMPLETED: AtomicU64 = AtomicU64::new(0);

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_tasks = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TASKS);
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

    println!(
        "{:?}: {} tasks on the {} run queue bound executor",
        progname.file_name().unwrap(),
        nr_tasks,
        nr_run_queue_bound,
    );

    let ex = Executor::new(nr_run_queue_bound);
    let mut spawners = vec![];
    (0..nr_tasks)
        .collect::<Vec<_>>()
        .chunks(nr_tasks / nr_spawners)
        .map(Vec::<_>::from)
        .for_each(|ids| {
            let spawner = ex.spawner();
            spawners.push(spawn(move || {
                ids.into_iter().for_each(|id| {
                    if let Err(e) = spawner.spawn(async move {
                        let hello = Hello::new(id as u64);
                        match hello.await {
                            Ok(_) => COMPLETED.fetch_add(1, Relaxed),
                            Err(e) => panic!("{e}"),
                        };
                    }) {
                        panic!("{e}");
                    }
                });
            }));
        });
    for spawner in spawners.drain(..) {
        if let Err(e) = spawner.join() {
            panic!("{e:?}");
        }
    }
    if let Err(e) = ex.run() {
        panic!("{e}");
    }
    assert_eq!(COMPLETED.load(Relaxed), nr_tasks as u64);
}
