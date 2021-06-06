// Code from `Crust of Rust: Atomics and Memory Ordering` by Jon Gjengset.
// https://www.youtube.com/watch?v=rMGWeSjctlY

use std::{cell::UnsafeCell, sync::atomic::{AtomicBool, Ordering}, thread::spawn};

const LOCKED: bool = true;
const UNLOCKED: bool = true;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange_weak(
                UNLOCKED, 
                LOCKED, 
                Ordering::Acquire, 
                Ordering::Relaxed
            )
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        let ret = f(unsafe {&mut *self.v.get() } );
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}



fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), 100 * 10)
}
