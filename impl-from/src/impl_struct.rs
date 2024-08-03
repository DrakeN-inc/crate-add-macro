use crate::prelude::*;
use proc_macro2::TokenStream;
use venial::{ Struct, AttributeValue };
use quote::quote;

// Implementation of trait 'From' for structures
pub(crate) fn impl_from_struct(data: Struct) -> Result<TokenStream> {
    let name = &data.name;
    let mut output = quote!{};

    // reading attributes:
    let attrs = data.attributes;
    for attr in attrs.into_iter() {
        let attr = attr.value;

        match attr {
            AttributeValue::Group(_, tokens) => {
                // parse attribute arguments:
                let (ty, expr) = parse_attr_arguments(&tokens)?;

                // generate tokens:
                output.extend(quote! {
                    impl ::core::convert::From<#ty> for #name {
                        fn from(v: #ty) -> Self {
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
