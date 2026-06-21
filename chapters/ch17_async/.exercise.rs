//! Chapter 17 — Fundamentals of Asynchronous Programming
//!
//! Real async Rust needs a *runtime* (an executor like Tokio) to drive futures
//! to completion. This crate has no dependencies and no runtime, so instead of
//! `async`/`await` we model the machinery by hand: a future is just a value
//! with a `poll` method that returns [`Poll::Pending`] (not done yet, call me
//! again) or [`Poll::Ready`] (here is the value). That is exactly what the
//! `Future` trait does under the hood, minus the `Context`/`Waker` plumbing.
//!
//! These exercises drill the core ideas of the chapter without any I/O,
//! threads, timers, or randomness:
//!
//! * **Futures and polling** — a state machine that yields `Pending` until it is
//!   `Ready` (§17.1).
//! * **An executor** — a `block_on` loop that polls a future to completion
//!   (§17.1, §17.5).
//! * **Joining** — driving two futures concurrently and collecting both
//!   results (§17.2).
//! * **Racing** — returning whichever future finishes first (§17.3).
//! * **Streams** — a sequence of values produced one `poll_next` at a time
//!   (§17.4).
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch17_async
//! ```

// ---------------------------------------------------------------------------
// A minimal future abstraction (no Waker/Context, no async runtime).
// ---------------------------------------------------------------------------

/// The result of polling a [`SimpleFuture`].
///
/// This mirrors the standard library's `Poll`, trimmed down for teaching:
/// either the future still has work to do (`Pending`) or it has produced its
/// final value (`Ready`).
#[derive(Debug, PartialEq, Eq)]
pub enum Poll<T> {
    /// The future is not finished; poll it again later.
    Pending,
    /// The future has completed and produced this value.
    Ready(T),
}

/// A hand-rolled stand-in for `std::future::Future`.
///
/// An executor repeatedly calls [`poll`](SimpleFuture::poll). Each call does a
/// little work and returns [`Poll::Pending`] until the final value is
/// available, at which point it returns [`Poll::Ready`]. Real futures also
/// receive a `Context` carrying a `Waker`; we omit it because our executor
/// simply polls in a loop.
pub trait SimpleFuture {
    /// The value produced when the future completes.
    type Output;

    /// Advances the future, returning whether it is `Pending` or `Ready`.
    fn poll(&mut self) -> Poll<Self::Output>;
}

// ---------------------------------------------------------------------------
// Exercise 1 — Futures and polling: a countdown state machine
// ---------------------------------------------------------------------------

/// A future that becomes ready after it has been polled a fixed number of times.
///
/// It models the lifecycle of any async operation: each `poll` represents one
/// turn on the executor. Until enough turns have passed the future reports
/// [`Poll::Pending`]; on the final turn it yields its [`label`](Countdown) as
/// the output. This is the simplest possible state machine — just a counter.
///
/// # Examples
///
/// ```
/// use ch17_async::{Countdown, Poll, SimpleFuture};
///
/// let mut fut = Countdown::new("done", 1);
/// assert_eq!(fut.poll(), Poll::Pending);          // 1 turn used up
/// assert_eq!(fut.poll(), Poll::Ready("done"));    // ready
/// ```
pub struct Countdown {
    label: &'static str,
    remaining: u32,
}

impl Countdown {
    /// Creates a countdown that is ready after `polls_needed` polls.
    ///
    /// A `polls_needed` of `0` means the future is ready on its very first poll.
    pub fn new(label: &'static str, polls_needed: u32) -> Countdown {
        // TODO: build a `Countdown` storing `label` and `polls_needed` as
        // `remaining`.
        todo!("construct a Countdown with the given label and poll count")
    }
}

impl SimpleFuture for Countdown {
    type Output = &'static str;

    /// Each call uses up one "turn". While turns remain, return `Pending` and
    /// decrement the counter; once none remain, return `Ready(label)`.
    fn poll(&mut self) -> Poll<&'static str> {
        // TODO: if `self.remaining == 0`, return `Poll::Ready(self.label)`.
        // Otherwise decrement `self.remaining` and return `Poll::Pending`.
        todo!("return Pending until the countdown reaches zero, then Ready(label)")
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 — An executor: block_on
// ---------------------------------------------------------------------------

/// Drives `future` to completion by polling it in a loop, returning its output.
///
/// This is the essence of an async runtime's `block_on`: keep polling until the
/// future stops saying [`Poll::Pending`]. A real executor would park the thread
/// between polls and wake on a `Waker`; here we just spin, which is fine because
/// our futures make progress on every poll.
///
/// # Examples
///
/// ```
/// use ch17_async::{block_on, Countdown};
///
/// let value = block_on(Countdown::new("hi", 3));
/// assert_eq!(value, "hi");
/// ```
pub fn block_on<F: SimpleFuture>(mut future: F) -> F::Output {
    // TODO: loop, calling `future.poll()`. On `Poll::Ready(value)` return the
    // value; on `Poll::Pending` keep looping.
    todo!("poll the future in a loop until it is Ready, then return its value")
}

// ---------------------------------------------------------------------------
// Exercise 3 — Concurrency: join two futures
// ---------------------------------------------------------------------------

/// Runs two futures concurrently, completing only when *both* have finished.
///
/// On every [`poll`](SimpleFuture::poll), `Join` polls whichever sub-future is
/// not yet done, caching each result as it arrives. This is cooperative
/// concurrency on a single thread: progress on either future is interleaved,
/// and the combined future is `Ready` only once both outputs are in hand. It is
/// the building block behind the book's `trpl::join`.
///
/// # Examples
///
/// ```
/// use ch17_async::{block_on, Countdown, Join};
///
/// let a = Countdown::new("a", 1);
/// let b = Countdown::new("b", 3);
/// assert_eq!(block_on(Join::new(a, b)), ("a", "b"));
/// ```
pub struct Join<A: SimpleFuture, B: SimpleFuture> {
    a: A,
    b: B,
    a_done: Option<A::Output>,
    b_done: Option<B::Output>,
}

impl<A: SimpleFuture, B: SimpleFuture> Join<A, B> {
    /// Combines two futures into one that resolves to both of their outputs.
    pub fn new(a: A, b: B) -> Join<A, B> {
        // TODO: store both futures and initialise `a_done`/`b_done` to `None`.
        todo!("construct a Join holding both futures with no results yet")
    }
}

impl<A: SimpleFuture, B: SimpleFuture> SimpleFuture for Join<A, B> {
    type Output = (A::Output, B::Output);

    /// Polls each sub-future that has not completed, stashing its result. Once
    /// both results are stored, returns them as a pair; otherwise `Pending`.
    fn poll(&mut self) -> Poll<(A::Output, B::Output)> {
        // TODO:
        // 1. If `a_done` is still `None`, poll `self.a`; on `Ready(value)`
        //    store `Some(value)` into `a_done`.
        // 2. Do the same for `b`.
        // 3. If both are now `Some`, return `Poll::Ready` of the pair (use
        //    `.take().unwrap()` on each). Otherwise return `Poll::Pending`.
        todo!("poll both futures, caching results, until both are Ready")
    }
}

// ---------------------------------------------------------------------------
// Exercise 4 — Racing: the first future to finish wins
// ---------------------------------------------------------------------------

/// Which side of a [`Race`] finished first.
#[derive(Debug, PartialEq, Eq)]
pub enum Winner<T> {
    /// The left (`a`) future completed first, carrying its output.
    Left(T),
    /// The right (`b`) future completed first, carrying its output.
    Right(T),
}

/// Runs two futures that share an output type and resolves as soon as *either*
/// one finishes, reporting which side won.
///
/// On each poll, `a` is polled before `b`. If both would be ready on the same
/// poll, `a` wins because it is checked first — a deliberate, deterministic
/// tie-break (no randomness). This mirrors the book's `trpl::race`, which
/// returns the first result and drops the loser.
///
/// # Examples
///
/// ```
/// use ch17_async::{block_on, Countdown, Race, Winner};
///
/// let slow = Countdown::new("slow", 5);
/// let fast = Countdown::new("fast", 1);
/// assert_eq!(block_on(Race::new(slow, fast)), Winner::Right("fast"));
/// ```
pub struct Race<A, B> {
    a: A,
    b: B,
}

impl<T, A, B> Race<A, B>
where
    A: SimpleFuture<Output = T>,
    B: SimpleFuture<Output = T>,
{
    /// Combines two futures into one that resolves with whichever finishes first.
    pub fn new(a: A, b: B) -> Race<A, B> {
        // TODO: store both futures in a `Race`.
        todo!("construct a Race holding both futures")
    }
}

impl<T, A, B> SimpleFuture for Race<A, B>
where
    A: SimpleFuture<Output = T>,
    B: SimpleFuture<Output = T>,
{
    type Output = Winner<T>;

    /// Polls `a` first, then `b`; returns the first one that is `Ready`,
    /// otherwise `Pending`.
    fn poll(&mut self) -> Poll<Winner<T>> {
        // TODO: poll `self.a` first; if `Ready(value)`, return
        // `Poll::Ready(Winner::Left(value))`. Then poll `self.b`; if `Ready`,
        // return `Winner::Right`. Otherwise return `Poll::Pending`.
        todo!("return whichever future is Ready first, tagged Left or Right")
    }
}

// ---------------------------------------------------------------------------
// Exercise 5 — Streams: futures in sequence
// ---------------------------------------------------------------------------

/// A stream is to an iterator what a future is to a value: it produces *many*
/// items over time, yielding [`Poll::Pending`] between them.
///
/// [`poll_next`](SimpleStream::poll_next) returns:
/// * `Poll::Pending` — no item ready yet, poll again;
/// * `Poll::Ready(Some(item))` — here is the next item;
/// * `Poll::Ready(None)` — the stream is finished.
pub trait SimpleStream {
    /// The type of each item the stream yields.
    type Item;

    /// Advances the stream by one step.
    fn poll_next(&mut self) -> Poll<Option<Self::Item>>;
}

/// A stream that yields the integers `start, start+1, …` up to but not
/// including `end`, then finishes.
///
/// Every other `poll_next` returns [`Poll::Pending`] to simulate a value that
/// is not ready instantly — proving the consumer must tolerate gaps in the
/// sequence, just like a real network stream.
///
/// # Examples
///
/// ```
/// use ch17_async::{collect_stream, CountStream};
///
/// // Yields 0, 1, 2 (with Pending stalls in between, handled by the collector).
/// assert_eq!(collect_stream(CountStream::new(0, 3)), vec![0, 1, 2]);
/// ```
pub struct CountStream {
    next: u32,
    end: u32,
    stall: bool,
}

impl CountStream {
    /// Creates a stream over the half-open range `start..end`.
    pub fn new(start: u32, end: u32) -> CountStream {
        // TODO: build a `CountStream` with `next = start`, the given `end`, and
        // `stall = true` so the first poll returns `Pending`.
        todo!("construct a CountStream over start..end that starts stalled")
    }
}

impl SimpleStream for CountStream {
    type Item = u32;

    /// Stalls (returns `Pending`) on alternate calls; otherwise yields the next
    /// integer, or `Ready(None)` once `end` is reached.
    fn poll_next(&mut self) -> Poll<Option<u32>> {
        // TODO:
        // 1. If `self.stall` is true, set it to false and return `Poll::Pending`.
        // 2. Otherwise set `self.stall = true`, then:
        //    - if `self.next < self.end`, yield `Poll::Ready(Some(self.next))`
        //      and advance `self.next` by 1;
        //    - else return `Poll::Ready(None)`.
        todo!("alternate Pending and the next integer, ending with Ready(None)")
    }
}

/// Drains a [`SimpleStream`] into a `Vec`, ignoring `Pending` stalls.
///
/// This is the streaming analogue of [`block_on`]: keep polling, skip over
/// `Pending`, push each `Some(item)`, and stop at the terminating `None`. It is
/// what a `while let Some(item) = stream.next().await` loop compiles down to in
/// spirit.
///
/// # Examples
///
/// ```
/// use ch17_async::{collect_stream, CountStream};
///
/// assert_eq!(collect_stream(CountStream::new(5, 8)), vec![5, 6, 7]);
/// ```
pub fn collect_stream<S: SimpleStream>(mut stream: S) -> Vec<S::Item> {
    // TODO: loop, calling `stream.poll_next()`. Skip `Poll::Pending`, push each
    // `Poll::Ready(Some(item))`, and return the collected `Vec` on
    // `Poll::Ready(None)`.
    todo!("drain the stream into a Vec, ignoring Pending stalls")
}
