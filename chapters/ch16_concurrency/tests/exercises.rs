use std::sync::{Arc, Mutex};

use ch16_concurrency::{
    SharedCounter, assert_send, assert_sync, channel_sum, concurrent_increments, parallel_squares,
};

// --- Exercise 1: threads + join ---------------------------------------------

#[test]
fn squares_each_input_in_its_own_thread() {
    assert_eq!(parallel_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

#[test]
fn parallel_squares_preserves_input_order() {
    // Joining handles in order means output index lines up with input index.
    assert_eq!(parallel_squares(vec![5, 0, -3]), vec![25, 0, 9]);
}

#[test]
fn parallel_squares_handles_empty_input() {
    assert_eq!(parallel_squares(vec![]), Vec::<i64>::new());
}

// --- Exercise 2: mpsc message passing ---------------------------------------

#[test]
fn channel_sum_totals_all_values() {
    assert_eq!(channel_sum(vec![1, 2, 3, 4, 5, 6], 3), 21);
}

#[test]
fn channel_sum_is_order_independent() {
    // Regardless of how many producers split the work, the total is the same.
    let values: Vec<i64> = (1..=100).collect();
    let expected = 5050;
    for workers in 1..=7 {
        assert_eq!(channel_sum(values.clone(), workers), expected);
    }
}

#[test]
fn channel_sum_handles_more_workers_than_values() {
    assert_eq!(channel_sum(vec![10, 20], 8), 30);
}

// --- Exercise 3: Arc<Mutex<T>> shared counter -------------------------------

#[test]
fn shared_counter_clones_share_one_count() {
    let counter = SharedCounter::new();
    let clone = counter.clone();
    counter.increment();
    clone.increment();
    counter.increment();
    assert_eq!(counter.value(), 3);
    assert_eq!(clone.value(), 3);
}

#[test]
fn concurrent_increments_has_no_lost_updates() {
    // 8 threads * 1000 increments = exactly 8000 once all threads are joined.
    assert_eq!(concurrent_increments(8, 1000), 8000);
}

#[test]
fn concurrent_increments_with_zero_work_stays_zero() {
    assert_eq!(concurrent_increments(4, 0), 0);
    assert_eq!(concurrent_increments(0, 100), 0);
}

// --- Exercise 4: Send + Sync marker traits ----------------------------------

#[test]
fn arc_mutex_is_send_and_sync() {
    // Arc<Mutex<T>> is the canonical "share across threads" type. If these
    // calls compile, the bounds `T: Send` / `T: Sync` are satisfied.
    let shared = Arc::new(Mutex::new(0u64));
    assert_send(shared.clone());
    assert_sync(shared);
}

#[test]
fn plain_data_is_send_and_sync() {
    assert_send(42u64);
    assert_sync(String::from("ferris"));
    assert_sync(vec![1, 2, 3]);
}
