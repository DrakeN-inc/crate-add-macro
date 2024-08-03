extern crate proc_macro;
use proc_macro::TokenStream;
use venial::{ parse_item, Item };

pub(crate) mod error;
pub(crate) mod tools;
pub(crate) mod prelude;     use prelude::*;
mod impl_enum;              use impl_enum::impl_from_enum;
mod impl_struct;            use impl_struct::impl_from_struct;

/// This macros provides the implementation of trait [From<T>](std::convert::From) (writed for crate [add_macro](https://docs.rs/add_macro))
/// 
/// # Examples:
/// ```
/// use add_macro_impl_from::From;
/// 
/// #[derive(Debug)]
/// enum SimpleError {
///     Wrong,
/// }
///
/// impl std::fmt::Display for SimpleError {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         match &self {
///             Self::Wrong => write!(f, "Something went wrong.. =/"),
///         }
///     }
/// }
///
/// #[derive(Debug)]
/// struct SuperError {
///     source: String,
/// }
///
/// impl std::fmt::Display for SuperError {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         write!(f, "{}", &self.source)
///     }
/// }
///
/// #[derive(Debug, From)]
/// #[from("std::io::Error" = "Self::Io(v)")]       // result: impl From<std::io::Error> for Error { fn from(v: std::io::Error) -> Self { Self::Io(v) } }
/// enum Error {
///     Io(std::io::Error),
///
///     #[from]
///     Simple(SimpleError),
///
///     #[from = "SuperError { source: format!(\"Super error: {}\", v.source) }"]
///     Super(SuperError),
///
///     #[from("String")]
///     #[from("&str" = "v.to_owned()")]
///     #[from("i32" = "format!(\"Error code: {v}\")")]
///     Stringify(String),
/// }
///
/// impl std::fmt::Display for Error {
///     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///         match &self {
///             Self::Io(e) => write!(f, "{e}"),
///             Self::Simple(e) => write!(f, "{e}"),
///             Self::Super(e) => write!(f, "{e}"),
///             Self::Stringify(e) => write!(f, "{e}"),
///         }
///     }
/// }
///
/// fn main() {
///     let io_err = Error::from( std::fs::read("fake/path/to/file").unwrap_err() );
///     assert_eq!(format!("{io_err}"), "No such file or directory (os error 2)");
///
///     let simple_err = Error::from( SimpleError::Wrong );
///     assert_eq!(format!("{simple_err}"), "Something went wrong.. =/");
///
///     let super_err = Error::from( SuperError { source: "Bad request".to_owned() } );
///     assert_eq!(format!("{super_err}"), "Super error: Bad request");
///
///     let str_err = Error::from( String::from("Something went wrong.. =/") );
///     assert_eq!(format!("{str_err}"), "Something went wrong.. =/");
///
///     let str_err2 = Error::from("Something went wrong.. =/");
///     assert_eq!(format!("{str_err2}"), "Something went wrong.. =/");
///
///     let str_err3 = Error::from(404);
///     assert_eq!(format!("{str_err3}"), "Error code: 404");
/// }
/// ```
#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let item = parse_item(input.into()).unwrap();
    
    match item {
        Item::Enum(data) => impl_from_enum(data).unwrap().into(),
        Item::Struct(data) => impl_from_struct(data).unwrap().into(),
        _ => panic!("{}", Error::ImplementationError)
    }
}
