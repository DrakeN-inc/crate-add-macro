extern crate proc_macro2;
pub(crate) mod prelude;     use prelude::*;
mod impl_struct;            use impl_struct::impl_display_struct;
mod impl_enum;              use impl_enum::impl_display_enum;


#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let DeriveInput {ident, data, ..} = syn::parse_macro_input!(input as DeriveInput);
    
    match data {
        Data::Struct(data) => impl_display_struct(ident, data),
        Data::Enum(data) => impl_display_enum(ident, data),
        _ => panic!("The trait 'Display' must be implemented on a structs and enums only!")
    }
}
