extern crate add_macro_impl_display;
use add_macro_impl_display::Display;

#[allow(dead_code)]
#[derive(Debug, Display)]
struct Person {
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[allow(dead_code)]
#[derive(Debug, Display)]
#[display("Hello, {name}! Your age is {age} years old.{email:.0}")]  // NOTE: if you don't need to use one of field, that use {field_name:.0} syntax
struct PersonInfo {
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[test]
fn test_struct() {
    assert_eq!(
        format!("{}", Person {
            name: "Bob".to_owned(),
            age: 22,
            email: "bob@example.loc".to_owned()
        }),
        r#"Person { name: "Bob", age: 22, email: "bob@example.loc" }"#
    );

    assert_eq!(
        format!("{}", PersonInfo {
            name: "Bob".to_owned(),
            age: 22,
            email: "bob@example.loc".to_owned()
        }),
        "Hello, Bob! Your age is 22 years old."
    );
}
