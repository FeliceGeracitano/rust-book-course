use ch18_oop::{Button, Circle, Post, Rectangle, Screen, SelectBox, Shape, total_area};

// --- Exercise 1: trait objects + a heterogeneous Screen ---------------------

#[test]
fn screen_renders_a_single_button() {
    let screen = Screen {
        components: vec![Box::new(Button {
            label: String::from("OK"),
        })],
    };
    assert_eq!(screen.render(), "[Button: OK]");
}

#[test]
fn screen_mixes_concrete_types_behind_trait_objects() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                options: vec![String::from("yes"), String::from("no")],
            }),
            Box::new(Button {
                label: String::from("Submit"),
            }),
        ],
    };
    assert_eq!(screen.render(), "[SelectBox: yes, no]\n[Button: Submit]");
}

#[test]
fn empty_screen_renders_nothing() {
    let screen = Screen { components: vec![] };
    assert_eq!(screen.render(), "");
}

// --- Exercise 2: dynamic dispatch over a slice of shapes --------------------

#[test]
fn rectangle_area_is_width_times_height() {
    let rect = Rectangle {
        width: 4.0,
        height: 2.5,
    };
    assert_eq!(rect.area(), 10.0);
}

#[test]
fn total_area_sums_mixed_shapes() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle {
            width: 2.0,
            height: 3.0,
        }),
        Box::new(Circle { radius: 1.0 }),
    ];
    // 6.0 from the rectangle + π·1² from the circle.
    let expected = 6.0 + std::f64::consts::PI;
    assert!((total_area(&shapes) - expected).abs() < 1e-9);
}

#[test]
fn total_area_of_no_shapes_is_zero() {
    let shapes: Vec<Box<dyn Shape>> = vec![];
    assert_eq!(total_area(&shapes), 0.0);
}

// --- Exercise 3: the state pattern ------------------------------------------

#[test]
fn draft_hides_content_until_published() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!(post.content(), "");

    post.request_review();
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "I ate a salad for lunch today");
}

#[test]
fn text_added_after_review_request_is_ignored() {
    let mut post = Post::new();
    post.add_text("first");
    post.request_review();
    post.add_text(" second"); // ignored: no longer a draft
    post.approve();
    assert_eq!(post.content(), "first");
}

#[test]
fn approving_a_draft_does_not_publish() {
    let mut post = Post::new();
    post.add_text("content");
    post.approve(); // a draft ignores approval
    assert_eq!(post.content(), "");

    post.request_review();
    post.approve();
    assert_eq!(post.content(), "content");
}
