//! 5.2.1 Coroutine/Task with Future Trait
use futures::task::waker_ref;
use sec521::Task;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};

const NR_TASKS: usize = 1_000;

fn main() {
    let mut args = std::env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_tasks = args
        .next()
        .as_ref()
        .and_then(|v| usize::from_str(v).ok())
        .unwrap_or(NR_TASKS);

    println!("{:?}: {} tasks", progname.file_name().unwrap(), nr_tasks);

    // prepping the coroutines/tasks.
    let mut tasks = vec![];
    (0..nr_tasks).for_each(|_| tasks.push(Arc::new(Task::new())));

    // a simple executor.
    let mut done = HashSet::new();
    loop {
        if done.len() == tasks.len() {
            break;
        }
        for task in &tasks {
            let waker = waker_ref(task);
            let mut ctx = Context::from_waker(&waker);
            let coroutine = &mut task.inner.lock().unwrap();
            if let Poll::Ready((id, msg)) = coroutine.as_mut().poll(&mut ctx) {
                println!("{id}: {msg}");
                done.insert(id);
            }
        }
    }
}
