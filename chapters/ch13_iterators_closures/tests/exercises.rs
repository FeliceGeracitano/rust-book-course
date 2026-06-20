use ch13_iterators_closures::{
    Inventory, ShirtColor, filter_collect, map_collect, search_insensitive, total_length,
};

// --- Exercise 1: closures (13.1) ---

#[test]
fn giveaway_respects_user_preference() {
    let store = Inventory { red: 3, blue: 5 };
    assert_eq!(store.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);
    assert_eq!(store.giveaway(Some(ShirtColor::Blue)), ShirtColor::Blue);
}

#[test]
fn giveaway_falls_back_to_most_stocked() {
    let mostly_blue = Inventory { red: 1, blue: 9 };
    assert_eq!(mostly_blue.giveaway(None), ShirtColor::Blue);

    let mostly_red = Inventory { red: 7, blue: 2 };
    assert_eq!(mostly_red.giveaway(None), ShirtColor::Red);
}

#[test]
fn most_stocked_breaks_ties_with_red() {
    let tied = Inventory { red: 4, blue: 4 };
    assert_eq!(tied.most_stocked(), ShirtColor::Red);
}

// --- Exercise 2: map + collect (13.2) ---

#[test]
fn map_collect_doubles_numbers() {
    let doubled = map_collect(&[1, 2, 3], |n| n * 2);
    assert_eq!(doubled, vec![2, 4, 6]);
}

#[test]
fn map_collect_changes_type() {
    let lengths = map_collect(&["a", "bb", "ccc"], |s| s.len());
    assert_eq!(lengths, vec![1, 2, 3]);
}

#[test]
fn map_collect_on_empty_slice() {
    let out: Vec<i32> = map_collect(&[], |n: &i32| n + 1);
    assert_eq!(out, Vec::<i32>::new());
}

// --- Exercise 3: filter + collect, fold (13.2) ---

#[test]
fn filter_collect_keeps_evens() {
    let evens = filter_collect(&[1, 2, 3, 4, 5, 6], |n| n % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6]);
}

#[test]
fn filter_collect_can_match_nothing() {
    let out = filter_collect(&[1, 3, 5], |n| n % 2 == 0);
    assert_eq!(out, Vec::<i32>::new());
}

#[test]
fn total_length_sums_word_lengths() {
    assert_eq!(total_length(&["ab", "cde"]), 5);
    assert_eq!(total_length(&["hello", "world"]), 10);
}

#[test]
fn total_length_of_empty_is_zero() {
    assert_eq!(total_length(&[]), 0);
}

// --- Exercise 4: iterator-style search (13.3) ---

#[test]
fn search_insensitive_finds_matching_lines() {
    let text = "Rust\nsafe, fast, productive.\nPick three.\nTrust me.";
    assert_eq!(
        search_insensitive("rUsT", text),
        vec!["Rust", "Trust me."]
    );
}

#[test]
fn search_insensitive_returns_empty_when_no_match() {
    let text = "alpha\nbeta\ngamma";
    assert_eq!(search_insensitive("zzz", text), Vec::<&str>::new());
}
