//! Chapter 18 — Object Oriented Programming Features
//!
//! These exercises drill the chapter's three big ideas: defining a trait
//! object (`Box<dyn Trait>`) so a collection can hold many different concrete
//! types, relying on *dynamic dispatch* to call the right method at run time,
//! and encoding a workflow with the *state pattern* so each state owns its own
//! behavior.
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch18_oop
//! ```

// ---------------------------------------------------------------------------
// Exercise 1 — Trait objects: a screen of drawable components
// ---------------------------------------------------------------------------

/// Shared behavior for anything that can be rendered to the screen.
///
/// To keep the test deterministic, "drawing" just returns a `String`
/// description instead of touching a real display. Because [`Screen`] stores
/// `Box<dyn Draw>` values, any type implementing this trait can be mixed into
/// the same list and rendered through *dynamic dispatch*.
pub trait Draw {
    /// Renders this component to a short textual description.
    fn draw(&self) -> String;
}

/// A clickable button with a label.
pub struct Button {
    /// The text shown on the button.
    pub label: String,
}

impl Draw for Button {
    /// Renders the button as `"[Button: <label>]"`.
    fn draw(&self) -> String {
        // TODO: use `format!` to produce `"[Button: {}]"` with `self.label`.
        todo!("render the button's label")
    }
}

/// A single-select list of options.
pub struct SelectBox {
    /// The choices the box presents.
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    /// Renders the box as `"[SelectBox: a, b, c]"` (options joined by ", ").
    fn draw(&self) -> String {
        // TODO: join `self.options` with ", " and wrap it in `"[SelectBox: ...]"`.
        todo!("render the select box's options")
    }
}

/// A screen owning a heterogeneous list of drawable components.
///
/// The components are stored as `Box<dyn Draw>` — *trait objects* — so the
/// screen can hold buttons, select boxes, and any future `Draw` type without
/// knowing their concrete types at compile time.
///
/// # Examples
///
/// ```
/// use ch18_oop::{Button, Draw, Screen};
///
/// let screen = Screen {
///     components: vec![Box::new(Button { label: String::from("OK") })],
/// };
/// assert_eq!(screen.render(), "[Button: OK]");
/// ```
pub struct Screen {
    /// The drawable components, each behind a trait object.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// Draws every component and joins the pieces with a newline.
    ///
    /// Each `component.draw()` call is dispatched dynamically: the runtime
    /// follows the trait object's vtable to the right concrete `draw`.
    pub fn render(&self) -> String {
        // TODO: iterate `self.components`, call `draw()` on each, and join the
        // resulting strings with "\n".
        todo!("draw and join every component")
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 — Dynamic dispatch over a function argument
// ---------------------------------------------------------------------------

/// A shape that knows its own area.
pub trait Shape {
    /// Returns the area of this shape.
    fn area(&self) -> f64;
}

/// A circle defined by its radius.
pub struct Circle {
    /// The circle's radius.
    pub radius: f64,
}

impl Shape for Circle {
    /// Area is `π · r²` (uses [`std::f64::consts::PI`]).
    fn area(&self) -> f64 {
        // TODO: return `std::f64::consts::PI * self.radius * self.radius`.
        todo!("compute the circle's area")
    }
}

/// An axis-aligned rectangle.
pub struct Rectangle {
    /// The rectangle's width.
    pub width: f64,
    /// The rectangle's height.
    pub height: f64,
}

impl Shape for Rectangle {
    /// Area is `width · height`.
    fn area(&self) -> f64 {
        // TODO: return `self.width * self.height`.
        todo!("compute the rectangle's area")
    }
}

/// Sums the area of every shape in a slice of trait objects.
///
/// Taking `&[Box<dyn Shape>]` means the caller can pass a mix of concrete
/// shapes; each `area()` call is resolved through dynamic dispatch.
///
/// # Examples
///
/// ```
/// use ch18_oop::{Rectangle, Shape, total_area};
///
/// let shapes: Vec<Box<dyn Shape>> =
///     vec![Box::new(Rectangle { width: 2.0, height: 3.0 })];
/// assert_eq!(total_area(&shapes), 6.0);
/// ```
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // TODO: iterate `shapes`, call `area()` on each, and sum the results.
    todo!("sum the areas of all shapes")
}

// ---------------------------------------------------------------------------
// Exercise 3 — The state pattern: a blog post workflow
// ---------------------------------------------------------------------------

/// A blog post that moves through a draft → pending-review → published
/// workflow.
///
/// The current [`State`] lives behind a trait object, so [`Post`] delegates
/// every transition to the state instead of branching on an enum. The public
/// content is only visible once the post reaches the published state.
///
/// # Examples
///
/// ```
/// use ch18_oop::Post;
///
/// let mut post = Post::new();
/// post.add_text("hello");
/// assert_eq!(post.content(), ""); // still a draft
/// post.request_review();
/// post.approve();
/// assert_eq!(post.content(), "hello"); // now published
/// ```
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    /// Creates a brand-new post in the draft state with empty content.
    pub fn new() -> Post {
        // TODO: build a `Post` whose `state` is `Some(Box::new(Draft {}))` and
        // whose `content` is an empty `String`.
        todo!("create a draft post")
    }

    /// Appends `text` to the post's draft content.
    ///
    /// Text can only be added while drafting; once review is requested this is
    /// a no-op (the current state decides via [`State::can_edit`]).
    pub fn add_text(&mut self, text: &str) {
        // TODO: if the current state's `can_edit()` is true, push `text` onto
        // `self.content`.
        todo!("append text only while editing is allowed")
    }

    /// Returns the content that should be shown publicly.
    ///
    /// Only the published state reveals the real content; every other state
    /// returns an empty string.
    pub fn content(&self) -> &str {
        // TODO: ask the current state for the content (`state.content(self)`).
        todo!("delegate content visibility to the current state")
    }

    /// Requests review, moving a draft into the pending-review state.
    pub fn request_review(&mut self) {
        // TODO: take `self.state` out, call `request_review()` on it, and store
        // the returned state back into `self.state`.
        todo!("transition the post toward review")
    }

    /// Approves the post; from pending-review this publishes it.
    pub fn approve(&mut self) {
        // TODO: take `self.state` out, call `approve()` on it, and store the
        // returned state back into `self.state`.
        todo!("transition the post toward publication")
    }
}

impl Default for Post {
    fn default() -> Self {
        Post::new()
    }
}

/// The behavior shared by every post state.
///
/// Transition methods consume `self: Box<Self>` and return the next state, so
/// the old state value is moved out and cannot be used again.
trait State {
    /// Returns the state to use after review is requested.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    /// Returns the state to use after an approval.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// Whether new text may be added in this state. Defaults to `false`.
    fn can_edit(&self) -> bool {
        false
    }

    /// The publicly visible content for this state. Defaults to empty.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/// The initial editable state.
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // TODO: a draft under review becomes `PendingReview`.
        todo!("move from draft to pending review")
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // TODO: approving a draft does nothing — return `self`.
        todo!("a draft ignores approval")
    }

    fn can_edit(&self) -> bool {
        // TODO: drafts are editable — return `true`.
        todo!("allow editing while drafting")
    }
}

/// Waiting for one approval before publishing.
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // TODO: requesting review again does nothing — return `self`.
        todo!("pending review ignores another review request")
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // TODO: approval here publishes the post — return `Published`.
        todo!("approve a pending post into publication")
    }
}

/// The published state, where content becomes visible.
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // TODO: a published post stays published — return `self`.
        todo!("published posts ignore review requests")
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // TODO: a published post stays published — return `self`.
        todo!("published posts ignore further approvals")
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // TODO: return the post's real content (`&post.content`).
        todo!("reveal the published content")
    }
}
