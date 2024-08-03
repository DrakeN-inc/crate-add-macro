extern crate proc_macro;
use proc_macro::TokenStream;
use venial::{ parse_item, Item };

pub(crate) mod error;
pub(crate) mod tools;
pub(crate) mod prelude;     use prelude::*;
mod impl_enum;              use impl_enum::impl_into_enum;
mod impl_struct;            use impl_struct::impl_into_struct;

/// This macros provides the implementation of trait [Into<T>](std::convert::Into) (writed for crate [add_macro](https://docs.rs/add_macro))
/// 
/// # Examples:
/// ```
/// use add_macro_impl_into::Into;
///
/// #[derive(Debug, PartialEq)]
/// struct User {
///     name: String,
///     subname: Option<String>,
///     age: u8
/// }
///
/// #[derive(Debug, Clone, Into)]
/// #[into("User" = "User { name: self.name, subname: None, age: self.age }")]
/// #[into("String" = "format!(\"name: {}, age: {}\", self.name, self.age)")]
/// struct Person {
///     name: String,
///     age: u8
/// }
///
/// fn main() {
///     let bob = Person {
///         name: "Bob".to_owned(),
///         age: 22
///     };
///
///     let bob_user: User = bob.clone().into();
///     assert_eq!(
///         bob_user,
///         User {
///             name: "Bob".to_owned(),
///             subname: None,
///             age: 22
///         }
///     );
///
///     let bob_str: String = bob.into();
///     assert_eq!(bob_str, "name: Bob, age: 22");
/// }
/// ```
/// ```
/// use add_macro_impl_into::Into;
///
/// #[derive(Debug, PartialEq)]
/// struct Cat;
///
/// #[derive(Debug, PartialEq)]
/// struct Dog;
///
/// #[derive(Debug, PartialEq)]
/// struct Bird;
///
/// #[derive(Debug, PartialEq)]
/// struct Python;
///
/// #[derive(Debug, PartialEq, Into)]
/// #[into("String" = "format!(\"Animal::{self:?}\")")]
/// #[into("Option<Cat>" = "if let Self::Cat(v) = self { Some(v) }else{ None }")]
/// enum Animal {
///     Cat(Cat),
///
///     #[into]
///     Dog(Dog),   // Option<Dog>
/// 
///     #[into = "if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }"]
///     // #[into("if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }")]
///     Bird(Bird),
/// 
///     #[into("Option<Python>" = "if let Self::Python(v) = self { Some(v) }else{ None }")]
///     #[into("Python" = "Into::<Option<Python>>::into(self).expect(\"It's not a Python\")")]
///     Python(Python),
/// }
///
/// fn main() {
///     let cat_str: String = Animal::Cat(Cat {}).into();
///     assert_eq!(cat_str, "Animal::Cat(Cat)");
/// 
///     let cat: Option<Cat> = Animal::Cat(Cat {}).into();
///     assert_eq!(cat, Some(Cat {}));
/// 
///     let dog: Option<Dog> = Animal::Dog(Dog {}).into();
///     assert_eq!(dog, Some(Dog {}));
///
///     let bird: Bird = Animal::Bird(Bird {}).into();
///     assert_eq!(bird, Bird {});
///
///     let python: Option<Python> = Animal::Python(Python {}).into();
///     assert_eq!(python, Some(Python {}));
///
///     let python: Python = Animal::Python(Python {}).into();
///     assert_eq!(python, Python {});
/// }
/// ```
#[proc_macro_derive(Into, attributes(into))]
pub fn derive_into(input: TokenStream) -> TokenStream {
    let item = parse_item(input.into()).unwrap();
    
    match item {
        Item::Enum(data) => match impl_into_enum(data) {
            Ok(output) => output.into(),
            Err(e) => panic!("{e}")
        },

        Item::Struct(data) => match impl_into_struct(data) {
            Ok(output) => output.into(),
            Err(e) => panic!("{e}")
        },

        _ => panic!("{}", Error::ImplementationError)
    }
}
