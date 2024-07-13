extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{ TokenTree, Literal, Ident, Span };
use venial::{ Item, parse_item };
use quote::quote;

pub(crate) mod error;
pub(crate) mod prelude;     use prelude::*;
mod impl_enum;              use impl_enum::impl_display_enum;
mod impl_struct;            use impl_struct::impl_display_struct;

/// The implementation of trait 'Display'
/// 
/// # Examples:
/// ```
/// use add_macro_impl_display::Display;
///
/// #[derive(Display)]
/// enum Animals {
///     Turtle,                                       /// style 1: "Turtle"
///
///     #[display]                                    /// style 2: "Bird"
///     Bird,
///
///     #[display = "The Cat"]                        /// style 3: "The Cat"
///     Cat,
///
///     #[display("The Dog")]                         /// style 4: "The Dog"
///     Dog,
///
///     #[display = "{0}"]                            /// style 3 with arguments: "..."
///     Other(&'static str),
///
///     #[display("{0} {1}, {2} years old")]          /// style 4 with arguments: "... ..., ... years old"
///     Info(Box<Self>, &'static str, i32),
///
///     #[display("{kind} {name}, {age} years old")]  /// style 4 with arguments: "... ..., ... years old" 
///     Info2 {
///         kind: Box<Self>,
///         name: &'static str,
///         age: i32,
///     },
/// }
///
/// #[test]
/// fn test_enum() -> std::io::Result<()> {
///     assert_eq!(format!("{}", Animals::Turtle), "Turtle");
///     assert_eq!(format!("{}", Animals::Bird), "Bird");
///     assert_eq!(format!("{}", Animals::Cat), "The Cat");
///     assert_eq!(format!("{}", Animals::Dog), "The Dog");
///     assert_eq!(format!("{}", Animals::Other("Tiger")), "Tiger");
///     assert_eq!(format!("{}", Animals::Info(Box::new(Animals::Cat), "Tomas", 2)), "The Cat Tomas, 2 years old");
///     assert_eq!(format!("{}", 
///         Animals::Info2 {
///             kind: Box::new(Animals::Cat),
///             name: "Tomas",
///             age: 2,
///         }),
///         "The Cat Tomas, 2 years old"
///     );
///
///     Ok(())
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
