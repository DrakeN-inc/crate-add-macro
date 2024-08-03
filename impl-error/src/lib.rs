extern crate proc_macro;
use proc_macro::TokenStream;
use venial::{ parse_item, Item };

pub(crate) mod error;
pub(crate) mod prelude;     use prelude::*;
mod impl_enum;              use impl_enum::impl_error_enum;
mod impl_struct;            use impl_struct::impl_error_struct;

/// This macros provides the implementation of trait [Error](std::error::Error) (writed for crate [add_macro](https://docs.rs/add_macro))
/// 
/// # Examples:
/// ```
/// use add_macro_impl_error::Error;
/// 
/// #[derive(Debug, Error)]
/// enum CustomError {
///     Io(std::io::Error),
///     Wrong,
/// }
///
/// impl std::fmt::Display for CustomError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match &self {
///             Self::Io(e) => write!(f, "{e}"),
///             Self::Wrong => write!(f, "Something went wrong.. =/"),
///         }
///     }
/// }
///
/// #[derive(Debug, Error)]
/// pub struct Error {
///     source: CustomError,
/// }
///
/// impl std::fmt::Display for Error {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "{}", &self.source)
///     }
/// }
///
/// fn main() {
///     let err = CustomError::Wrong;
///     assert_eq!(format!("{err}"), "Something went wrong.. =/");
/// 
///     let err = Error { source: CustomError::Wrong };
///     assert_eq!(format!("{err}"), "Something went wrong.. =/");
/// }
/// ```
#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let item = parse_item(input.into()).unwrap();
    
    match item {
        Item::Enum(data) => impl_error_enum(data).unwrap().into(),
        Item::Struct(data) => impl_error_struct(data).unwrap().into(),
        _ => panic!("{}", Error::ImplementationError)
    }
}
