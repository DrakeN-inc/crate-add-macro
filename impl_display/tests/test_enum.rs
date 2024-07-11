extern crate add_macro_impl_display;
use add_macro_impl_display::Display;


#[derive(Display)]
enum Pets {
    #[display = "The cat"]
    Cat,
    #[display = "The dog"]
    Dog,
    #[display = "The bird"]
    Bird,
    #[display = "Name: {0}, Age: {1}"]
    Person(String, u8),
}

#[test]
fn impl_enum() {
    debug_assert_eq!(format!("{}", Pets::Cat), "The cat");
    debug_assert_eq!(format!("{}", Pets::Dog), "The dog");
    debug_assert_eq!(format!("{}", Pets::Bird), "The bird");
}
