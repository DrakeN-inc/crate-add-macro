extern crate add_macro;
use add_macro::input;

#[test]
fn test_input() {
    let buf: String = input!("Type something: ");

    assert_eq!(buf, "123");
}
