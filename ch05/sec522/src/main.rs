//! 5.2.2 Async Task Executor
use sec522::{Executor, Hello};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::spawn;

const NR_TASKS: usize = 1_024;
const NR_TASK_SPAWN_CHUNK: usize = 10;
const NR_RUN_QUEUE_BOUND: usize = 2_048;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_tasks = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TASKS);
    let nr_run_queue_bound = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_RUN_QUEUE_BOUND);

    println!(
        "{:?} with {} tasks on {} run queue bound",
        progname.file_name().unwrap(),
        nr_tasks,
        nr_run_queue_bound,
    );

    // Simple single thread executor.
    let ex = Executor::<String>::new(nr_run_queue_bound);

    // spawn tasks NR_TASK_SPAWN_CHUNK each from multiple threads.
    let mut spawners = vec![];
    let task_ids: Vec<Vec<_>> = (0..nr_tasks)
        .collect::<Vec<_>>()
        .chunks(NR_TASK_SPAWN_CHUNK)
        .map(|v| v.into())
        .collect();
    for ids in task_ids {
        let spawner = ex.spawner();
        spawners.push(spawn(move || {
            for id in ids {
                if let Err(e) = spawner.spawn(Hello::new(id as u64)) {
                    panic!("{e}");
                }
            }
        }));
    }
    if let Err(e) = ex.run() {
        panic!("{e}");
    }
    for spawner in spawners.drain(..) {
        if let Err(e) = spawner.join() {
            panic!("{e:?}");
        }
    }
}
