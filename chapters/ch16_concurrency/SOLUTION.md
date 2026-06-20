# Chapter 16 — Solutions

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// Exercise 1 — Threads + join

pub fn parallel_squares(inputs: Vec<i64>) -> Vec<i64> {
    let mut handles = Vec::with_capacity(inputs.len());
    for n in inputs {
        handles.push(thread::spawn(move || n * n));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.join().expect("thread panicked"));
    }
    results
}

// Exercise 2 — Message passing over an mpsc channel

pub fn channel_sum(values: Vec<i64>, workers: usize) -> i64 {
    let (tx, rx) = mpsc::channel();

    let workers = workers.max(1);
    let chunk_size = values.len().div_ceil(workers).max(1);

    let mut handles = Vec::new();
    for chunk in values.chunks(chunk_size) {
        let tx = tx.clone();
        let chunk: Vec<i64> = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let partial: i64 = chunk.iter().sum();
            tx.send(partial).expect("receiver dropped");
        }));
    }
    drop(tx);

    let total: i64 = rx.iter().sum();

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
    total
}

// Exercise 3 — Shared state with Arc<Mutex<T>>

#[derive(Clone)]
pub struct SharedCounter {
    count: Arc<Mutex<u64>>,
}

impl SharedCounter {
    pub fn new() -> SharedCounter {
        SharedCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut guard = self.count.lock().expect("mutex poisoned");
        *guard += 1;
    }

    pub fn value(&self) -> u64 {
        *self.count.lock().expect("mutex poisoned")
    }
}

impl Default for SharedCounter {
    fn default() -> Self {
        SharedCounter::new()
    }
}

pub fn concurrent_increments(threads: usize, per_thread: usize) -> u64 {
    let counter = SharedCounter::new();

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                counter.increment();
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
    counter.value()
}

// Exercise 4 — Send + Sync marker traits

pub fn assert_send<T: Send>(_value: T) {}

pub fn assert_sync<T: Sync>(_value: T) {}
```
