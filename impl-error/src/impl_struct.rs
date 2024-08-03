use crate::prelude::*;
use proc_macro2::TokenStream;
use venial::Struct;
use quote::quote;

// Implementation of trait 'Error' for structures
pub(crate) fn impl_error_struct(data: Struct) -> Result<TokenStream> {
    let name = &data.name;

    Ok(quote! {
        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(&self.source)
            }
        }
    })
}
