//! 3.8.3 Readers/Writer Lock
use sec383::{reader, writer};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread::spawn;

const NR_READERS: u64 = 200;
const NR_WRITERS: u64 = 100;

fn main() {
    let mut args = env::args();
    let progname = args.next().map(PathBuf::from).unwrap();
    let nr_readers = args
        .next()
        .and_then(|v| u64::from_str(&v).ok())
        .unwrap_or(NR_READERS);
    let nr_writers = args
        .next()
        .and_then(|v| u64::from_str(&v).ok())
        .unwrap_or(NR_WRITERS);

    println!(
        "{:?}: {} readers and {} writers.",
        progname.file_name().unwrap(),
        nr_readers,
        nr_writers,
    );

    let lock0 = Arc::new(RwLock::new(0));
    let mut readers = vec![];
    let mut writers = vec![];

    (0..nr_readers).for_each(|id| {
        let lock = lock0.clone();
        readers.push(spawn(move || reader(id, lock)));
    });
    (0..nr_writers).for_each(|id| {
        let lock = lock0.clone();
        writers.push(spawn(move || writer(id, lock)));
    });

    for (id, reader) in readers.drain(..).enumerate() {
        match reader.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }
    for (id, writer) in writers.drain(..).enumerate() {
        match writer.join() {
            Err(e) => panic!("{e:?}"),
            Ok(result) => match result {
                Err(e) => panic!("{e}"),
                Ok(got) => assert_eq!(got, id as u64),
            },
        }
    }

    let lock = lock0.read().unwrap();
    assert_eq!(*lock, NR_WRITERS);
}
