extern crate add_macro_impl_display;
use add_macro_impl_display::Display;

// Test structure without attribute:
#[derive(Debug, Display)]
struct SimplePerson {
    pub name: String,
    pub age: u8,
}

#[test]
fn test_simple_struct() -> std::io::Result<()> {
    assert_eq!(
        format!("{}", SimplePerson {
            name: "Bob".to_owned(),
            age: 22
        }),
        "SimplePerson { name: \"Bob\", age: 22 }"
    );

    Ok(())
}


// Test structure with attribute:
#[derive(Debug, Display)]
#[display("Hello, {name}! Your age is {age} years old.")]
struct Person {
    pub name: String,
    pub age: u8,
}

#[test]
fn test_struct() -> std::io::Result<()> {
    assert_eq!(
        format!("{}", Person {
            name: "Bob".to_owned(),
            age: 22
        }),
        "Hello, Bob! Your age is 22 years old."
    );

    Ok(())
}
