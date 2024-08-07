extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{ parse_macro_input, DeriveInput };
use quote::quote;

/// This macros provides the implementation of the trait [FromStr](std::str::FromStr) (writed for crate [add_macro](https://docs.rs/add_macro))
/// 
/// # Examples:
/// ```
/// use add_macro_impl_fromstr::FromStr;
/// 
/// #[derive(Debug)]
/// enum Error {
///     ParseError,
/// }
/// 
/// #[derive(FromStr)]
/// struct Email {
///     name: String,
///     host: String
/// }
/// 
/// impl Email {
///     // WARNING: this method needs for working the implementation trait FromStr
///     pub fn parse(s: &str) -> Result<Self, Error> {
///         let spl = s.trim().splitn(2, "@").collect::<Vec<_>>();
///         
///         if spl.len() == 2 {
///             Ok(Self {
///                 name: spl[0].to_owned(),
///                 host: spl[1].to_owned(),
///             })
///         } else {
///             Err(Error::ParseError)
///         }
///     }
/// }
/// 
/// fn main() -> Result<(), Error> {
///     let _bob_email: Email = "bob@example.loc".parse()?;
/// 
///     Ok(())
/// }
/// ```
#[proc_macro_derive(FromStr)]
pub fn derive_fromstr(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    let name = &parsed.ident;
    
    quote!{
        impl ::core::str::FromStr for #name {
            type Err = Error;
            
            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                Self::parse(s)
            }
        }
    }
    .into()
}
