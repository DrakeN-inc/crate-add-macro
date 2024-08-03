extern crate add_macro_impl_display;
use add_macro_impl_display::Display;

#[derive(Debug, Display)]
struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Display)]
#[display("Hello, {name}! Your age is {age} years old.")]
struct Person2 {
    pub name: String,
    pub age: u8,
}

#[test]
fn test_struct() {
    assert_eq!(
        format!("{}", Person {
            name: "Bob".to_owned(),
            age: 22
        }),
        "Person { name: \"Bob\", age: 22 }"
    );

    assert_eq!(
        format!("{}", Person2 {
            name: "Bob".to_owned(),
            age: 22
        }),
        "Hello, Bob! Your age is 22 years old."
    );
}
