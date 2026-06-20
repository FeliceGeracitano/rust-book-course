# 17.6 Futures, Tasks, and Threads

Async is not a replacement for threads — it is a different tool, and they
compose well. The distinction comes in three layers:

* A **future** is a unit of work that can be paused and resumed. It is just a
  value; nothing runs until it is polled.
* A **task** is a top-level future handed to an executor to drive to completion.
  An executor can multiplex thousands of tasks onto a small pool of threads,
  because each task yields (returns `Pending`) instead of blocking.
* A **thread** is an OS-level worker. Threads give you *parallelism*; the kernel
  preempts them whenever it likes.

The trade-off: threads are *preemptive* (any thread can be paused at any
instruction) while tasks are *cooperative* (a task only yields when it `.await`s
or returns `Pending`). A task that loops forever without yielding will starve
its neighbours — there is no `poll` call to hand control back.

```rust
// CPU-bound, no await points -> use a thread.
let handle = std::thread::spawn(|| heavy_computation());
// Many waiting-on-I/O jobs -> use async tasks on one executor.
```

Rule of thumb: reach for threads when work is CPU-bound and parallel; reach for
async tasks when work is I/O-bound and you have *many* of them waiting.

### Exercise
No new code for this section. Make sure your whole chapter is green:

```bash
cargo test -p ch17_async
```
