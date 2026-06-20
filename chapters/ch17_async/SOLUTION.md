# Chapter 17 — Solutions

These are the worked implementations for `src/lib.rs`. We have no async runtime
available, so we model futures by hand: a future is a value with a `poll`
method returning `Pending` or `Ready`.

```rust
/// The result of polling a future: not done yet, or finished with a value.
#[derive(Debug, PartialEq, Eq)]
pub enum Poll<T> {
    Pending,
    Ready(T),
}

/// A hand-rolled stand-in for `std::future::Future`.
pub trait SimpleFuture {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

// --- Exercise 1: Futures and polling ---------------------------------------

/// A future that becomes ready after a fixed number of polls.
pub struct Countdown {
    label: &'static str,
    remaining: u32,
}

impl Countdown {
    pub fn new(label: &'static str, polls_needed: u32) -> Countdown {
        Countdown { label, remaining: polls_needed }
    }
}

impl SimpleFuture for Countdown {
    type Output = &'static str;

    fn poll(&mut self) -> Poll<&'static str> {
        if self.remaining == 0 {
            Poll::Ready(self.label)
        } else {
            self.remaining -= 1;
            Poll::Pending
        }
    }
}

// --- Exercise 2: An executor (block_on) ------------------------------------

/// Drives a future to completion by polling in a loop.
pub fn block_on<F: SimpleFuture>(mut future: F) -> F::Output {
    loop {
        match future.poll() {
            Poll::Ready(value) => return value,
            Poll::Pending => continue,
        }
    }
}

// --- Exercise 3: Join two futures ------------------------------------------

/// Runs two futures concurrently, completing when both are done.
pub struct Join<A: SimpleFuture, B: SimpleFuture> {
    a: A,
    b: B,
    a_done: Option<A::Output>,
    b_done: Option<B::Output>,
}

impl<A: SimpleFuture, B: SimpleFuture> Join<A, B> {
    pub fn new(a: A, b: B) -> Join<A, B> {
        Join { a, b, a_done: None, b_done: None }
    }
}

impl<A: SimpleFuture, B: SimpleFuture> SimpleFuture for Join<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(&mut self) -> Poll<(A::Output, B::Output)> {
        if self.a_done.is_none() {
            if let Poll::Ready(value) = self.a.poll() {
                self.a_done = Some(value);
            }
        }
        if self.b_done.is_none() {
            if let Poll::Ready(value) = self.b.poll() {
                self.b_done = Some(value);
            }
        }

        if self.a_done.is_some() && self.b_done.is_some() {
            Poll::Ready((self.a_done.take().unwrap(), self.b_done.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

// --- Exercise 4: Race ------------------------------------------------------

/// Which side of a race finished first.
#[derive(Debug, PartialEq, Eq)]
pub enum Winner<T> {
    Left(T),
    Right(T),
}

/// Resolves as soon as either future finishes; `a` wins ties (polled first).
pub struct Race<A, B> {
    a: A,
    b: B,
}

impl<T, A, B> Race<A, B>
where
    A: SimpleFuture<Output = T>,
    B: SimpleFuture<Output = T>,
{
    pub fn new(a: A, b: B) -> Race<A, B> {
        Race { a, b }
    }
}

impl<T, A, B> SimpleFuture for Race<A, B>
where
    A: SimpleFuture<Output = T>,
    B: SimpleFuture<Output = T>,
{
    type Output = Winner<T>;

    fn poll(&mut self) -> Poll<Winner<T>> {
        if let Poll::Ready(value) = self.a.poll() {
            return Poll::Ready(Winner::Left(value));
        }
        if let Poll::Ready(value) = self.b.poll() {
            return Poll::Ready(Winner::Right(value));
        }
        Poll::Pending
    }
}

// --- Exercise 5: Streams ---------------------------------------------------

/// Like a future, but yields many items over time.
pub trait SimpleStream {
    type Item;
    fn poll_next(&mut self) -> Poll<Option<Self::Item>>;
}

/// Yields `start..end`, stalling with `Pending` between items.
pub struct CountStream {
    next: u32,
    end: u32,
    stall: bool,
}

impl CountStream {
    pub fn new(start: u32, end: u32) -> CountStream {
        CountStream { next: start, end, stall: true }
    }
}

impl SimpleStream for CountStream {
    type Item = u32;

    fn poll_next(&mut self) -> Poll<Option<u32>> {
        if self.stall {
            self.stall = false;
            return Poll::Pending;
        }
        self.stall = true;

        if self.next < self.end {
            let item = self.next;
            self.next += 1;
            Poll::Ready(Some(item))
        } else {
            Poll::Ready(None)
        }
    }
}

/// Drains a stream into a `Vec`, ignoring `Pending` stalls.
pub fn collect_stream<S: SimpleStream>(mut stream: S) -> Vec<S::Item> {
    let mut items = Vec::new();
    loop {
        match stream.poll_next() {
            Poll::Pending => continue,
            Poll::Ready(Some(item)) => items.push(item),
            Poll::Ready(None) => return items,
        }
    }
}
```
