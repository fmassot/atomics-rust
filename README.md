## Test mutex on a macbook with M1 processor

### Intro
In one of his [great video](https://www.youtube.com/watch?v=rMGWeSjctlY), Jon Gjengset implements a mutex 
to notably understand the effect of `std::sync::atomic::Ordering`.
The code which runs is very simple : create a mutex that holds an integer and start many threads to add 1
concurrently to this mutex and see the results.

When using correct ordering, we expect the program to make additions atomically and check the result as the sum of all
added values. Unfortunately it does not seem to work on linux/x86_64 nor on macbook/arm64.

### Results

Run `cargo r --release` and sometimes you will see

```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `985`,
 right: `1000`', src/main.rs:58:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
