use ch19_patterns_matching::{
    classify_char, describe_point, grade_score, summarize, Message, Point,
};

#[test]
fn classify_char_uses_ranges_and_catch_all() {
    // digits
    assert_eq!(classify_char('0'), "digit");
    assert_eq!(classify_char('9'), "digit");
    // letters, both cases
    assert_eq!(classify_char('a'), "letter");
    assert_eq!(classify_char('z'), "letter");
    assert_eq!(classify_char('A'), "letter");
    assert_eq!(classify_char('Z'), "letter");
    // whitespace
    assert_eq!(classify_char(' '), "space");
    assert_eq!(classify_char('\t'), "space");
    assert_eq!(classify_char('\n'), "space");
    // everything else
    assert_eq!(classify_char('%'), "other");
    assert_eq!(classify_char('-'), "other");
    assert_eq!(classify_char('é'), "other");
}

#[test]
fn describe_point_destructures_struct() {
    assert_eq!(describe_point(Point { x: 0, y: 0 }), "origin");
    assert_eq!(describe_point(Point { x: 4, y: 0 }), "on the x axis");
    assert_eq!(describe_point(Point { x: -7, y: 0 }), "on the x axis");
    assert_eq!(describe_point(Point { x: 0, y: 2 }), "on the y axis");
    assert_eq!(describe_point(Point { x: 0, y: -2 }), "on the y axis");
    assert_eq!(describe_point(Point { x: 3, y: 5 }), "off both axes");
    assert_eq!(describe_point(Point { x: -1, y: -1 }), "off both axes");
}

#[test]
fn summarize_quit_and_write() {
    assert_eq!(summarize(Message::Quit), "quit");
    assert_eq!(summarize(Message::Write(String::from("hello"))), "write: hello");
    assert_eq!(summarize(Message::Write(String::new())), "write: ");
}

#[test]
fn summarize_move_uses_guard_for_diagonal() {
    assert_eq!(summarize(Message::Move { x: 3, y: 3 }), "move diagonally to 3");
    assert_eq!(summarize(Message::Move { x: 0, y: 0 }), "move diagonally to 0");
    assert_eq!(summarize(Message::Move { x: 1, y: 4 }), "move to (1, 4)");
    assert_eq!(summarize(Message::Move { x: -2, y: 5 }), "move to (-2, 5)");
}

#[test]
fn summarize_change_color_guards_on_range() {
    assert_eq!(summarize(Message::ChangeColor(255, 0, 128)), "color #ff0080");
    assert_eq!(summarize(Message::ChangeColor(0, 0, 0)), "color #000000");
    assert_eq!(summarize(Message::ChangeColor(16, 32, 48)), "color #102030");
    // out of range on any channel -> invalid
    assert_eq!(summarize(Message::ChangeColor(300, 0, 0)), "invalid color");
    assert_eq!(summarize(Message::ChangeColor(0, -1, 0)), "invalid color");
    assert_eq!(summarize(Message::ChangeColor(0, 0, 256)), "invalid color");
}

#[test]
fn grade_score_binds_with_at_operator() {
    assert_eq!(grade_score(100), Some(String::from("grade A (100)")));
    assert_eq!(grade_score(95), Some(String::from("grade A (95)")));
    assert_eq!(grade_score(90), Some(String::from("grade A (90)")));
    assert_eq!(grade_score(89), Some(String::from("grade B (89)")));
    assert_eq!(grade_score(42), Some(String::from("grade B (42)")));
    assert_eq!(grade_score(1), Some(String::from("grade B (1)")));
    assert_eq!(grade_score(0), None);
    assert_eq!(grade_score(-5), None);
    assert_eq!(grade_score(101), None);
    assert_eq!(grade_score(200), None);
}
