extern crate add_macro_impl_display;
use add_macro_impl_display::Display;

#[derive(Display)]
struct Person {
    pub name: String,
    pub age: u8,
}

#[test]
fn impl_struct() {
    let bob = Person {
        name: "Bob".to_owned(),
        age: 22,
    };

    debug_assert_eq!(format!("{bob}"), "123");
}
