# Concurrent Programming in Rust

Concurrent programming in Rust, demonstrated by [Yuuki Takano] in
[Concurrent Programming].

## Atomic Primitives

- [CAS: Compare And Swap](ch03/sec321/compare_and_swap.s)
- [TAS: Test And Set](ch03/sec322/test_and_set.s)
- [Mutex by TAS](ch03/sec330/main.c)
- [Spinlock by TTAS](ch03/sec331/lib.c)

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
cpr$ cd ch02/sec239
ch02/sec239
cpr$ cargo run
```

Happy Hacking!

[yuuki takano]: https://scholar.google.co.jp/citations?user=RiH0Kt0AAAAJ&hl
[concurrent programming]: https://www.oreilly.co.jp/books/9784873119595/
[cargo workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
