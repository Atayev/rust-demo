use demo::demo;

mod common;

#[test]

fn test_demo() {
    assert_eq!(demo(), 42, "Invalid value");
}
