use crate::prelude::*;
use proc_macro2::TokenStream;
use venial::Enum;
use quote::quote;

// Implementation of trait 'Error' for enumerations
pub(crate) fn impl_error_enum(data: Enum) -> Result<TokenStream> {
    let name = &data.name;

    Ok(quote! {
        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }
    })
}
