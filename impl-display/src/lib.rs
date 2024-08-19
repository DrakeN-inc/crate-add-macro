extern crate proc_macro;
use proc_macro::TokenStream;
use venial::{ parse_item, Item };

pub(crate) mod error;
pub(crate) mod tools;
pub(crate) mod prelude;     use prelude::*;
mod impl_enum;              use impl_enum::impl_display_enum;
mod impl_struct;            use impl_struct::impl_display_struct;

/// This macros provides the implementation of trait [Display](std::fmt::Display) (writed for crate [add_macro](https://docs.rs/add_macro))
/// 
/// # Examples:
/// ```
/// use add_macro_impl_display::Display;
/// 
/// #[derive(Display)]
/// enum Animals {
///     Turtle,
///   
///     #[display]
///     Bird,
///    
///     #[display = "The Cat"]
///     Cat,
/// 
///     #[display("The Dog")]
///     Dog,
///
///     Other(&'static str),
///
///     #[display]
///     Other2(&'static str),
/// 
///     #[display("{0} {1}, {2} years old")]
///     Info(Box<Self>, &'static str, i32),
///
///     #[display("{kind} {name}, {age} years old")] 
///     Info2 {
///         kind: Box<Self>,
///         name: &'static str,
///         age: i32,
///     },
/// }
///
/// fn main() {
///     assert_eq!(format!("{}", Animals::Turtle), "Turtle");
///     assert_eq!(format!("{}", Animals::Bird), "Bird");
///     assert_eq!(format!("{}", Animals::Cat), "The Cat");
///     assert_eq!(format!("{}", Animals::Dog), "The Dog");
///     assert_eq!(format!("{}", Animals::Other("Tiger")), "Tiger");
///     assert_eq!(format!("{}", Animals::Other2("Tiger")), "Tiger");
///     assert_eq!(format!("{}", Animals::Info(Box::new(Animals::Cat), "Tomas", 2)), "The Cat Tomas, 2 years old");
///     assert_eq!(format!("{}", 
///         Animals::Info2 {
///             kind: Box::new(Animals::Cat),
///             name: "Tomas",
///             age: 2,
///         }),
///         "The Cat Tomas, 2 years old"
///     );
/// }
/// ```
/// ```
/// use add_macro_impl_display::Display;
/// 
/// #[derive(Debug, Display)]
/// struct Person {
///     pub name: String,
///     pub age: u8,
///     pub email: String,
/// }
/// 
/// #[derive(Debug, Display)]
/// #[display("Hello, {name}! Your age is {age} years old.{email:.0}")]  // NOTE: if you don't need to use one of field, that use {field_name:.0} syntax
/// struct PersonInfo {
///     pub name: String,
///     pub age: u8,
///     pub email: String,
/// }
///
/// fn main() {
///     assert_eq!(
///         format!("{}", Person {
///             name: "Bob".to_owned(),
///             age: 22,
///             email: "bob@example.loc".to_owned()
///         }),
///         r#"Person { name: "Bob", age: 22, email: "bob@example.loc" }"#
///     );
/// 
///     assert_eq!(
///         format!("{}", PersonInfo {
///             name: "Bob".to_owned(),
///             age: 22,
///             email: "bob@example.loc".to_owned()
///         }),
///         "Hello, Bob! Your age is 22 years old."
///     );
/// }
/// ```
#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let item = parse_item(input.into()).unwrap();
    
    match item {
        Item::Enum(data) => impl_display_enum(data).unwrap().into(),
        Item::Struct(data) => impl_display_struct(data).unwrap().into(),
        _ => panic!("{}", Error::ImplementationError)
    }
}
