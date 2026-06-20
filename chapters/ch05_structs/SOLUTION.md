# Chapter 5 — Solutions

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn build_rectangle(width: u32, height: u32) -> Rectangle {
    // Field init shorthand: `width` and `height` match the field names.
    Rectangle { width, height }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u64,
}

pub fn with_email(base: User, email: String) -> User {
    // Struct update syntax: take `email` explicitly, copy the rest from `base`.
    User {
        email,
        ..base
    }
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```
