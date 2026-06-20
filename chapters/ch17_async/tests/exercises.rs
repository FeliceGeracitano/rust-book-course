use ch17_async::{
    block_on, collect_stream, Countdown, CountStream, Join, Poll, Race, SimpleFuture, SimpleStream,
    Winner,
};

// --- Exercise 1: Futures and polling ----------------------------------------

#[test]
fn countdown_pends_then_becomes_ready() {
    let mut fut = Countdown::new("done", 2);
    assert_eq!(fut.poll(), Poll::Pending);
    assert_eq!(fut.poll(), Poll::Pending);
    assert_eq!(fut.poll(), Poll::Ready("done"));
}

#[test]
fn countdown_zero_is_ready_immediately() {
    let mut fut = Countdown::new("now", 0);
    assert_eq!(fut.poll(), Poll::Ready("now"));
}

#[test]
fn countdown_stays_ready_after_completing() {
    let mut fut = Countdown::new("x", 1);
    assert_eq!(fut.poll(), Poll::Pending);
    assert_eq!(fut.poll(), Poll::Ready("x"));
    // Polling again still reports Ready (remaining stays at 0).
    assert_eq!(fut.poll(), Poll::Ready("x"));
}

// --- Exercise 2: block_on executor ------------------------------------------

#[test]
fn block_on_runs_a_future_to_completion() {
    assert_eq!(block_on(Countdown::new("hello", 5)), "hello");
}

#[test]
fn block_on_handles_an_already_ready_future() {
    assert_eq!(block_on(Countdown::new("instant", 0)), "instant");
}

// --- Exercise 3: Join two futures -------------------------------------------

#[test]
fn join_waits_for_both_futures() {
    let a = Countdown::new("a", 1);
    let b = Countdown::new("b", 4);
    assert_eq!(block_on(Join::new(a, b)), ("a", "b"));
}

#[test]
fn join_is_pending_until_both_done() {
    // a finishes on the first poll, b needs two polls.
    let a = Countdown::new("a", 0);
    let b = Countdown::new("b", 2);
    let mut joined = Join::new(a, b);
    assert_eq!(joined.poll(), Poll::Pending); // a done, b not
    assert_eq!(joined.poll(), Poll::Pending); // b still pending
    assert_eq!(joined.poll(), Poll::Ready(("a", "b")));
}

#[test]
fn join_supports_different_output_types() {
    // Outputs need not match: both are &'static str here, but Join is generic.
    let a = Countdown::new("left", 3);
    let b = Countdown::new("right", 1);
    assert_eq!(block_on(Join::new(a, b)), ("left", "right"));
}

// --- Exercise 4: Race -------------------------------------------------------

#[test]
fn race_returns_the_faster_future() {
    let slow = Countdown::new("slow", 5);
    let fast = Countdown::new("fast", 1);
    assert_eq!(block_on(Race::new(slow, fast)), Winner::Right("fast"));
}

#[test]
fn race_left_can_win() {
    let fast = Countdown::new("fast", 1);
    let slow = Countdown::new("slow", 5);
    assert_eq!(block_on(Race::new(fast, slow)), Winner::Left("fast"));
}

#[test]
fn race_breaks_ties_in_favor_of_the_left_future() {
    // Both ready on the same poll; `a` is polled first, so it wins.
    let a = Countdown::new("a", 0);
    let b = Countdown::new("b", 0);
    assert_eq!(block_on(Race::new(a, b)), Winner::Left("a"));
}

// --- Exercise 5: Streams ----------------------------------------------------

#[test]
fn count_stream_yields_the_range() {
    assert_eq!(collect_stream(CountStream::new(0, 3)), vec![0, 1, 2]);
}

#[test]
fn count_stream_can_be_empty() {
    assert_eq!(collect_stream(CountStream::new(5, 5)), Vec::<u32>::new());
}

#[test]
fn count_stream_stalls_with_pending_between_items() {
    let mut stream = CountStream::new(10, 12);
    assert_eq!(stream.poll_next(), Poll::Pending);
    assert_eq!(stream.poll_next(), Poll::Ready(Some(10)));
    assert_eq!(stream.poll_next(), Poll::Pending);
    assert_eq!(stream.poll_next(), Poll::Ready(Some(11)));
    assert_eq!(stream.poll_next(), Poll::Pending);
    assert_eq!(stream.poll_next(), Poll::Ready(None));
}
