Async/sync function call bench
==============================

The only purpose of this benchmark is to present that there is a difference between calling async and sync functions.

Development
-----------

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/async_fn_bench`
Async/sync function call benchmark
Async foo
x = 10000000
microseconds = 377538
Sync foo
x = 10000000
microseconds = 147436
```

Production
----------

```
$ cargo build --release
[..]
$ ./target/release/async_fn_bench
Async/sync function call benchmark
Async foo
x = 10000000
microseconds = 12109
Sync foo
x = 10000000
microseconds = 819
```
