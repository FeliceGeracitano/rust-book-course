use ch10_generics_traits_lifetimes::{
    Article, Excerpt, Pair, Summary, Tweet, largest, longest, notify,
};

// --- 10.1 Generic Data Types ----------------------------------------------

#[test]
fn largest_finds_max_in_ints() {
    assert_eq!(largest(&[3, 7, 2, 9, 4]), Some(9));
    assert_eq!(largest(&[10]), Some(10));
    assert_eq!(largest(&[-5, -1, -9]), Some(-1));
}

#[test]
fn largest_works_for_chars_and_floats() {
    assert_eq!(largest(&['a', 'z', 'm']), Some('z'));
    assert_eq!(largest(&[1.5_f64, 0.25, 3.75]), Some(3.75));
}

#[test]
fn largest_of_empty_is_none() {
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn pair_reports_larger_element() {
    assert_eq!(Pair::new(3, 8).larger(), 8);
    assert_eq!(Pair::new(8, 3).larger(), 8);
    assert_eq!(Pair::new(5, 5).larger(), 5);
    assert_eq!(Pair::new('x', 'a').larger(), 'x');
}

// --- 10.2 Defining Shared Behavior with Traits ----------------------------

#[test]
fn tweet_overrides_summarize() {
    let tweet = Tweet {
        username: String::from("alice"),
        content: String::from("learning rust"),
    };
    assert_eq!(tweet.summarize_author(), "@alice");
    assert_eq!(tweet.summarize(), "@alice: learning rust");
}

#[test]
fn article_uses_default_summarize() {
    let article = Article {
        headline: String::from("Rust 2024 ships"),
        author: String::from("Jo"),
    };
    assert_eq!(article.summarize_author(), "Jo");
    assert_eq!(article.summarize(), "(Read more from Jo...)");
}

#[test]
fn notify_accepts_any_summary() {
    let tweet = Tweet {
        username: String::from("bob"),
        content: String::from("hi"),
    };
    let article = Article {
        headline: String::from("News"),
        author: String::from("Kim"),
    };
    assert_eq!(notify(&tweet), "Breaking! @bob: hi");
    assert_eq!(notify(&article), "Breaking! (Read more from Kim...)");
}

// --- 10.3 Validating References with Lifetimes ----------------------------

#[test]
fn longest_returns_longer_slice() {
    assert_eq!(longest("apple", "fig"), "apple");
    assert_eq!(longest("hi", "world"), "world");
}

#[test]
fn longest_prefers_first_on_tie() {
    assert_eq!(longest("cat", "dog"), "cat");
}

#[test]
fn excerpt_takes_first_sentence() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::first_sentence(&novel);
    assert_eq!(excerpt.part, "Call me Ishmael");
    assert_eq!(excerpt.len(), "Call me Ishmael".len());
}

#[test]
fn excerpt_handles_no_period() {
    let line = String::from("no period here");
    let excerpt = Excerpt::first_sentence(&line);
    assert_eq!(excerpt.part, "no period here");
    assert_eq!(excerpt.len(), 14);
}
