use ch08_collections::{capitalize_words, running_totals, word_count};

// ---- 8.1 Vectors: running_totals ----

#[test]
fn running_totals_basic() {
    assert_eq!(running_totals(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn running_totals_empty() {
    assert_eq!(running_totals(&[]), Vec::<i64>::new());
}

#[test]
fn running_totals_handles_negatives() {
    assert_eq!(running_totals(&[5, -2, -1, 10]), vec![5, 3, 2, 12]);
}

#[test]
fn running_totals_single_element() {
    assert_eq!(running_totals(&[42]), vec![42]);
}

// ---- 8.2 Strings: capitalize_words ----

#[test]
fn capitalize_words_basic() {
    assert_eq!(capitalize_words("hello world"), "Hello World");
}

#[test]
fn capitalize_words_collapses_whitespace() {
    assert_eq!(capitalize_words("  rust   is fun "), "Rust Is Fun");
}

#[test]
fn capitalize_words_empty() {
    assert_eq!(capitalize_words(""), "");
}

#[test]
fn capitalize_words_keeps_rest_of_word_untouched() {
    assert_eq!(capitalize_words("rustACEAN iCED"), "RustACEAN ICED");
}

#[test]
fn capitalize_words_is_utf8_aware() {
    // 'é' upper-cases to 'É'; the multi-byte tail must survive intact.
    assert_eq!(capitalize_words("éclair café"), "Éclair Café");
}

// ---- 8.3 Hash Maps: word_count ----

#[test]
fn word_count_counts_repeats() {
    let counts = word_count("the cat the dog the");
    assert_eq!(counts.get("the"), Some(&3));
    assert_eq!(counts.get("cat"), Some(&1));
    assert_eq!(counts.get("dog"), Some(&1));
}

#[test]
fn word_count_missing_key_is_none() {
    let counts = word_count("the cat the dog the");
    assert_eq!(counts.get("fish"), None);
}

#[test]
fn word_count_empty_input() {
    let counts = word_count("");
    assert!(counts.is_empty());
}

#[test]
fn word_count_total_distinct_words() {
    let counts = word_count("a b c a b a");
    assert_eq!(counts.len(), 3);
    assert_eq!(counts.get("a"), Some(&3));
    assert_eq!(counts.get("b"), Some(&2));
    assert_eq!(counts.get("c"), Some(&1));
}
