use crate::prelude::*;
use proc_macro2::TokenStream;
use venial::{ Struct, AttributeValue };
use quote::quote;

// Implementation of trait 'Into<T>' for structures
pub(crate) fn impl_into_struct(data: Struct) -> Result<TokenStream> {
    let name = &data.name;
    let mut output = quote!{};

    // reading attributes:
    let attrs = data.attributes;
    for attr in attrs.into_iter() {
        if !check_attr_name(&attr, "into") { continue }

        match attr.value {
            AttributeValue::Group(_, tokens) => {
                // parse attribute arguments:
                let (ty, expr) = parse_attr_arguments(&tokens)?;

                // generate tokens:
                output.extend(quote! {
                    impl ::core::convert::Into<#ty> for #name {
                        fn into(self) -> #ty {
                            #expr
                        }
                    }
                });
            },

            _ => return Err(Error::IncorrectAttribute)
        }
    }

    Ok(output)
}
