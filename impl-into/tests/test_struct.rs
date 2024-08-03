extern crate add_macro_impl_into;
use add_macro_impl_into::Into;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    subname: Option<String>,
    age: u8
}

#[derive(Debug, Clone, Into)]
#[into("User" = "User { name: self.name, subname: None, age: self.age,  }")]
#[into("String" = "format!(\"name: {}, age: {}\", self.name, self.age)")]
struct Person {
    name: String,
    age: u8
}

#[test]
fn test_struct() {
    let bob = Person {
        name: "Bob".to_owned(),
        age: 22
    };

    let bob_user: User = bob.clone().into();
    assert_eq!(
        bob_user,
        User {
            name: "Bob".to_owned(),
            subname: None,
            age: 22
        }
    );

    let bob_str: String = bob.into();
    assert_eq!(bob_str, "name: Bob, age: 22");
}
