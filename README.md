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
- [Benchmark](ch03/sec373/main.c)

## Locks

Here is the list of example locks explained in the book:

- [SpinLock](ch04/sec70/src/lib.rs)
- [FairLock](ch07/sec11/src/lib.rs)

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
