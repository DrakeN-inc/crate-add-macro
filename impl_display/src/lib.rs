extern crate proc_macro;
use proc_macro::TokenStream;
use venial::{ parse_item, Item };

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
/// 
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
