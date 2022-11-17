//! 5.3.1 Future by async/await
use sec531::{Executor, Hello};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::spawn;

const NR_TASKS: usize = 1_024;
const NR_TASK_CHUNK_SIZE: usize = 100;
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
        "{:?}: {} tasks with {} run queue bound executor",
        progname.file_name().unwrap(),
        nr_tasks,
        nr_run_queue_bound,
    );

    let ex = Executor::<String>::new(nr_run_queue_bound);
    let mut spawners = vec![];
    (0..nr_tasks)
        .collect::<Vec<_>>()
        .chunks(NR_TASK_CHUNK_SIZE)
        .map(|v| v.into())
        .for_each(|ids: Vec<_>| {
            let spawner = ex.spawner();
            spawners.push(spawn(move || {
                ids.into_iter().for_each(|id| {
                    if let Err(e) = spawner.spawn(async move {
                        let hello = Hello::new(id as u64);
                        hello.await
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
}
