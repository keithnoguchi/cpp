# Concurrent Programming in Rust

Concurrent programming in Rust, demonstrated by [Yuuki Takano] in
[Concurrent Programming].

## Primitives in C

- [CAS: Compare And Swap](ch03/sec321/compare_and_swap.s)
- [TAS: Test And Set](ch03/sec322/test_and_set.s)
- [Mutex by TAS](ch03/sec330/main.c)
- [Spinlock by TTAS](ch03/sec331/lib.c)
- [Posix Mutex](ch03/sec332/main.c)
- [Semaphore](ch03/sec340/lib.c)
- [Posix Semaphore](ch03/sec342/lib.c)
- [Posix Condition Variable](ch03/sec350/lib.c)
- [Memory Barrier by Spinlock](ch03/sec361/lib.c)
- [Memory Barrier by Posix Condition Variable](ch03/sec362/lib.c)
- [R/W Lock by Spinlock](ch03/sec371/lib.c)
- [Posix R/W Lock](ch03/sec372/lib.c)
- [Recursive Lock](ch04/sec440/lib.c)
- [Signal Handler](ch04/sec460/lib.c)
- [Benchmark](ch03/sec373/main.c)

## Bugs in C

- [Spurious Wakeup](ch04/sec450/lib.c)

## Primitives in Rust

- [Mutex](ch03/sec381/src/lib.rs)
- [Condition Variable](ch03/sec382/src/lib.rs)
- [R/W Lock](ch03/sec383/src/lib.rs)
- [Memory Barrier](ch03/sec384/src/lib.rs)
- [Semaphore](ch03/sec385/src/semaphore.rs)
- [Channel](ch03/sec385/src/channel.rs)
- [Signal Handler](ch04/sec461/src/lib.rs)

## Bugs in Rust

- [Deadlocks](ch04/sec410/src/main.rs)
  - [Dining Philosophers Problem with Mutex](ch04/sec410/src/mutex.rs)
  - [Deadlock with RwLock](ch04/sec410/src/rwlock.rs)
- [Livelocks](ch04/sec420/src/lib.rs)

## Algorithms in Rust

- [Bakery Lock](ch03/sec390/src/bakery.rs) by [Leslie Lamport](https://en.wikipedia.org/wiki/Lamport%27s_bakery_algorithm).
- [Banker/Detection Algorithm](ch04/sec430/src/banker.rs) by [Edsger Dijkstra](https://en.wikipedia.org/wiki/Banker%27s_algorithm)
- [SpinLock](ch04/sec470/src/lib.rs)
- [FairLock](ch07/sec11/src/lib.rs)

## Async Primitives in Rust

- [Coroutine/Task with Future Trait](ch05/src521/src/hello.rs)
  - [Simple Coroutine/Task](ch05/src521/src/lib.rs)
- [Single Threaded Task Executor](ch05/sec522/src/lib.rs)
  - [Task Spawner & Waker](ch05/sec522/src/task.rs)
- [Future by async/await](ch05/sec531/src/main.rs)
  - [Single Threaded Executor, Spawner, and Waker](ch05/sec531/src/lib.rs)

## Services in Rust

- [Sync Echo Server](ch05/sec510/src/lib.rs)
- [Async Echo Server with epoll(7)](ch05/sec511/src/lib.rs)

## Asynchronous Chat Client and Server

This is the example chat client and server application demonstrated
in Chapter 20, Asynchronous Programming, of the wonderful [Programming Rust],
2nd Edition:

- [protocol](chat/protocol/src/lib.rs) crate
- [packet](chat/packet/src/lib.rs) crate
- [connection](chat/connection/src/lib.rs) crate
- [group](chat/group/Cargo.toml) crate
  - [Group](chat/group/src/lib.rs) type
- [state](chat/state/Cargo.toml) crate
  - [Table](chat/state/src/lib.rs) type
- [client](chat/client/src/lib.rs) crate
- [server](chat/server/src/lib.rs) crate

## Decentralized Chat Client

Let's implement the decentralized chat client powered by [libp2p],
as in [libp2p example].

- [dclient](dchat/dclient/src/lib.rs) crate

## Run

Each sections are cargo crate to be self contained.  And since
we use [cargo workspace] to organize those crate, you can't
run `cargo run` in the top directory.  You either go into
each sub directory to run `cargo run`, or use one of the
cargo sub command, e.g. `cargo check` in the top directory:

```
cpr$ cargo check
```

```
cpr$ $(cd ch02/sec239 && cargo run)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/cpr/target/debug/sec239`
Hello world
success: ()
success: 20
thread '<unnamed>' panicked at 'oops', ch02/sec239/src/main.rs:28:20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: Some("oops")
```

Happy Hacking!

[yuuki takano]: https://scholar.google.co.jp/citations?user=RiH0Kt0AAAAJ&hl
[concurrent programming]: https://www.oreilly.co.jp/books/9784873119595/
[cargo workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[programming rust]: https://github.com/ProgrammingRust
[libp2p]: https://lib.rs/crates/libp2p
[libp2p example]: https://github.com/libp2p/rust-libp2p/blob/master/examples/chat.rs
