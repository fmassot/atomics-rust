## Test spin locks on a macbook with M1 processor

Run `cargo r --release` and sometimes you will see

```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `985`,
 right: `1000`', src/main.rs:58:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```