# Chapter 10 — Solutions

```rust
// ---------------------------------------------------------------------------
// 10.1 Generic Data Types
// ---------------------------------------------------------------------------

pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut iter = items.iter();
    let mut best = *iter.next()?;
    for &item in iter {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T: PartialOrd + Copy> Pair<T> {
    pub fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    pub fn larger(&self) -> T {
        if self.first >= self.second {
            self.first
        } else {
            self.second
        }
    }
}

// ---------------------------------------------------------------------------
// 10.2 Defining Shared Behavior with Traits
// ---------------------------------------------------------------------------

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

pub struct Article {
    pub headline: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // No `summarize` here on purpose — it uses the trait's default.
}

pub fn notify(item: &impl Summary) -> String {
    format!("Breaking! {}", item.summarize())
}

// ---------------------------------------------------------------------------
// 10.3 Validating References with Lifetimes
// ---------------------------------------------------------------------------

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        let part = match text.find('.') {
            Some(idx) => &text[..idx],
            None => text,
        };
        Excerpt { part }
    }

    pub fn len(&self) -> usize {
        self.part.len()
    }
}
```
