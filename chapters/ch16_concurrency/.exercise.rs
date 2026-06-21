//! Chapter 16 — Fearless Concurrency
//!
//! These exercises drill the chapter's core ideas: spawning threads and
//! collecting their results with `join`, passing data between threads over an
//! `mpsc` channel, sharing a counter across threads with `Arc<Mutex<T>>`, and
//! reasoning about the `Send` / `Sync` marker traits.
//!
//! Every exercise is deterministic: each one joins all of its threads and
//! asserts on the final result, so there are no sleeps, no timing, and no
//! randomness.
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch16_concurrency
//! ```

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

// ---------------------------------------------------------------------------
// Exercise 1 — Threads + join: run work in parallel and collect results
// ---------------------------------------------------------------------------

/// Squares every number in `inputs` by giving each one its own thread.
///
/// Each value is `move`d into a freshly spawned thread that computes its
/// square. The returned `JoinHandle`s are then joined *in order*, so the output
/// vector lines up with the input vector — joining is what makes the result
/// deterministic.
///
/// # Examples
///
/// ```
/// use ch16_concurrency::parallel_squares;
///
/// assert_eq!(parallel_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
/// ```
pub fn parallel_squares(inputs: Vec<i64>) -> Vec<i64> {
    // TODO: For each `n` in `inputs`, `thread::spawn(move || n * n)` and collect
    // the `JoinHandle`s into a Vec. Then `join()` each handle *in order* and
    // push its result into the output Vec so the output lines up with the input.
    todo!("spawn one thread per input, then join them in order")
}

// ---------------------------------------------------------------------------
// Exercise 2 — Message passing: an mpsc channel with multiple producers
// ---------------------------------------------------------------------------

/// Sums `values` by fanning them out across `workers` threads over a channel.
///
/// The transmitter is cloned once per worker (mpsc = *multiple producer, single
/// consumer*) and each worker `move`s its slice in, sends a partial sum, and
/// drops its sender. The main thread owns the single receiver and iterates it
/// until every sender has dropped, accumulating the partial sums into a total.
///
/// Because addition is commutative and we wait for the channel to close, the
/// total is the same no matter what order the partial sums arrive in.
///
/// # Examples
///
/// ```
/// use ch16_concurrency::channel_sum;
///
/// assert_eq!(channel_sum(vec![1, 2, 3, 4, 5, 6], 3), 21);
/// ```
pub fn channel_sum(values: Vec<i64>, workers: usize) -> i64 {
    // TODO:
    // 1. Create a channel with `mpsc::channel()`.
    // 2. Split `values` into chunks (use `workers.max(1)` producers; a chunk
    //    size of `values.len().div_ceil(workers).max(1)` works).
    // 3. For each chunk: clone `tx`, `move` the chunk into a spawned thread,
    //    `send` the chunk's partial sum, and keep the `JoinHandle`.
    // 4. `drop(tx)` (the original sender) so the receiver can stop once every
    //    cloned sender has dropped.
    // 5. Sum everything from `rx.iter()` into the total, then join all handles.
    todo!("fan the work out over a channel and accumulate the partial sums")
}

// ---------------------------------------------------------------------------
// Exercise 3 — Shared state: an Arc<Mutex<T>> counter
// ---------------------------------------------------------------------------

/// A counter that many threads can share and increment safely.
///
/// `Arc<T>` gives every thread a cheap, atomically reference-counted handle to
/// the *same* value, and `Mutex<T>` ensures only one thread mutates the count
/// at a time. Cloning a [`SharedCounter`] clones the `Arc`, so all clones point
/// at one underlying count.
///
/// # Examples
///
/// ```
/// use ch16_concurrency::SharedCounter;
///
/// let counter = SharedCounter::new();
/// let clone = counter.clone();
/// counter.increment();
/// clone.increment();
/// assert_eq!(counter.value(), 2); // both handles share one count
/// ```
#[derive(Clone)]
pub struct SharedCounter {
    count: Arc<Mutex<u64>>,
}

impl SharedCounter {
    /// Creates a new counter starting at `0`.
    pub fn new() -> SharedCounter {
        // TODO: build a `SharedCounter` whose `count` is `Arc::new(Mutex::new(0))`.
        todo!("create a shared counter starting at 0")
    }

    /// Adds one to the shared count, locking the mutex for the update.
    pub fn increment(&self) {
        // TODO: `lock()` the mutex (it returns a guard), then `*guard += 1`.
        todo!("lock the mutex and add one")
    }

    /// Reads the current count.
    pub fn value(&self) -> u64 {
        // TODO: `lock()` the mutex and return the value it guards (`*guard`).
        todo!("lock the mutex and read the count")
    }
}

impl Default for SharedCounter {
    fn default() -> Self {
        SharedCounter::new()
    }
}

/// Spawns `threads` threads that each call `increment` `per_thread` times on a
/// shared [`SharedCounter`], then joins them all and returns the final count.
///
/// Each thread gets its own clone of the counter (a cloned `Arc`). After every
/// thread has been joined, exactly `threads * per_thread` increments have
/// happened, so the result is deterministic with no lost updates.
///
/// # Examples
///
/// ```
/// use ch16_concurrency::concurrent_increments;
///
/// assert_eq!(concurrent_increments(8, 1000), 8000);
/// ```
pub fn concurrent_increments(threads: usize, per_thread: usize) -> u64 {
    // TODO:
    // 1. Make one `SharedCounter`.
    // 2. Spawn `threads` threads; give each its own `counter.clone()` (moved in)
    //    and have it call `increment()` `per_thread` times.
    // 3. Join every handle, then return `counter.value()`.
    todo!("share one counter across threads, then join and read the total")
}

// ---------------------------------------------------------------------------
// Exercise 4 — Send + Sync: which types may cross thread boundaries?
// ---------------------------------------------------------------------------

/// Compile-time proof that a type is `Send` (safe to *move* to another thread).
///
/// This generic function has a `T: Send` bound but never uses its argument, so
/// it does nothing at runtime. Its value is in the *type checker*: it only
/// compiles when called with a `Send` type, which lets a test assert that, say,
/// `Arc<Mutex<u64>>` is `Send` while `std::rc::Rc<u64>` is not.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use ch16_concurrency::assert_send;
///
/// assert_send(Arc::new(Mutex::new(0u64)));
/// ```
pub fn assert_send<T: Send>(_value: T) {
    // TODO: this body stays empty — the whole point is the `T: Send` bound in
    // the signature above. Delete the `todo!` line below to "complete" it; the
    // tests then prove (at compile time) that the types they pass are `Send`.
    todo!("nothing to do at runtime — remove this line once you understand the Send bound")
}

/// Compile-time proof that a type is `Sync` (safe to *share* by reference
/// across threads, i.e. `&T: Send`).
///
/// Like [`assert_send`], this is a type-level assertion with no runtime effect.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
/// use ch16_concurrency::assert_sync;
///
/// assert_sync(Arc::new(Mutex::new(0u64)));
/// ```
pub fn assert_sync<T: Sync>(_value: T) {
    // TODO: like `assert_send`, the body stays empty — the `T: Sync` bound in
    // the signature is the assertion. Delete the `todo!` line below.
    todo!("nothing to do at runtime — remove this line once you understand the Sync bound")
}
