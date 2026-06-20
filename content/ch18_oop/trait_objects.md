# 18.2 Using Trait Objects to Abstract over Shared Behavior

Sometimes you want a collection of values that share *behavior* but not a
single concrete type — a list of GUI widgets, say, that all know how to draw
themselves. A generic `Vec<T>` won't do: it holds exactly one type. The answer
is a *trait object*, written `Box<dyn Trait>`. It pairs a pointer to some value
with a pointer to that type's method table (its vtable).

```rust
pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn render(&self) -> String {
        self.components.iter().map(|c| c.draw()).collect::<Vec<_>>().join("\n")
    }
}
```

Because the concrete type is erased, the call `c.draw()` is resolved through
*dynamic dispatch*: at run time Rust follows the vtable to the right method.
That costs a pointer hop generics avoid, but buys you a single list holding
buttons, text boxes, or any future type that implements `Draw` — without the
`Screen` knowing those types exist. Trait objects must be *dyn compatible*
(roughly: no generic methods, no `Self` return), which `Draw` satisfies.

### Exercise
In `chapters/ch18_oop/src/lib.rs`, finish `Screen::render`, the `Shape`
implementations, and `total_area` (Exercises 1–2). Then run:

```bash
cargo test -p ch18_oop
```
