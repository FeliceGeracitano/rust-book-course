# Chapter 19 — Solutions

```rust
pub fn classify_char(c: char) -> &'static str {
    match c {
        '0'..='9' => "digit",
        'a'..='z' | 'A'..='Z' => "letter",
        ' ' | '\t' | '\n' => "space",
        _ => "other",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn describe_point(p: Point) -> &'static str {
    match p {
        Point { x: 0, y: 0 } => "origin",
        Point { x: _, y: 0 } => "on the x axis",
        Point { x: 0, y: _ } => "on the y axis",
        Point { .. } => "off both axes",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn summarize(msg: Message) -> String {
    match msg {
        Message::Quit => String::from("quit"),
        Message::Move { x, y } if x == y => format!("move diagonally to {x}"),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write: {text}"),
        Message::ChangeColor(r, g, b)
            if (0..=255).contains(&r) && (0..=255).contains(&g) && (0..=255).contains(&b) =>
        {
            format!("color #{r:02x}{g:02x}{b:02x}")
        }
        Message::ChangeColor(..) => String::from("invalid color"),
    }
}

pub fn grade_score(score: i32) -> Option<String> {
    match score {
        n @ 90..=100 => Some(format!("grade A ({n})")),
        n @ 1..=89 => Some(format!("grade B ({n})")),
        _ => None,
    }
}
```
