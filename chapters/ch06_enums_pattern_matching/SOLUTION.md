# Chapter 6 — Solutions

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsState {
    Alabama,
    Alaska,
    California,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}

pub fn describe_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        Some(format!("Quarter from {state:?}"))
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Move { dx: i32, dy: i32 },
    Home,
}

pub fn apply_command(position: (i32, i32), command: Command) -> (i32, i32) {
    match command {
        Command::Move { dx, dy } => (position.0 + dx, position.1 + dy),
        Command::Home => (0, 0),
    }
}

pub fn username_or_guest<'a>(users: &[(u32, &'a str)], id: u32) -> &'a str {
    let Some(&(_, name)) = users.iter().find(|&&(uid, _)| uid == id) else {
        return "guest";
    };
    name
}
```
