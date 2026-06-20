# Chapter 13 — Solutions

```rust
#[derive(Debug, Clone)]
pub struct Inventory {
    pub red: u32,
    pub blue: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShirtColor {
    Red,
    Blue,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        if self.red >= self.blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn map_collect<T, U, F>(items: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    items.iter().map(|item| f(item)).collect()
}

pub fn filter_collect<T, F>(items: &[T], mut predicate: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    items
        .iter()
        .filter(|item| predicate(item))
        .cloned()
        .collect()
}

pub fn total_length(words: &[&str]) -> usize {
    words.iter().fold(0, |acc, word| acc + word.len())
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```
