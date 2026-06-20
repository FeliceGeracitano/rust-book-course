use ch01_getting_started::{build_tool, greeting};

#[test]
fn greeting_is_hello_world() {
    assert_eq!(greeting(), "Hello, world!");
}

#[test]
fn build_tool_is_cargo() {
    assert_eq!(build_tool(), "cargo");
}
