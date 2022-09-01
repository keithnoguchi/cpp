# Concurrent Programming in Rust

Concurrent programming in Rust, demonstrated by [Yuuki Takano] in
[Concurrent Programming].

## Locks

Here is the list of example locks explained in the book:

- [SpinLock](ch04/sec70/src/lib.rs)
- [FairLock](ch07/sec11/src/lib.rs)

## Run

It's organized by the different executable crate per each sections
and managed by the [cargo workspace].  Hence, you can run all the
executable crates with the `cargo run` command from the top directory.


```
$ cargo run
```

You can also run the individual sections by moving into it and run
`cargo run`:

```
$ cd ch02/sec239
$ cargo run
```

Happy Hacking!

[yuuki takano]: https://scholar.google.co.jp/citations?user=RiH0Kt0AAAAJ&hl
[concurrent programming]: https://www.oreilly.co.jp/books/9784873119595/
[cargo workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
