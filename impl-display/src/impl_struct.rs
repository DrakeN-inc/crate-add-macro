use crate::prelude::*;
use proc_macro2::{ TokenStream, TokenTree, Literal };
use venial::{ Struct, Attribute, AttributeValue, Fields };
use quote::quote;

// Implementation of trait 'Display' for Struct
pub(crate) fn impl_display_struct(data: Struct) -> Result<TokenStream> {
    // handle struct attribute & get format string:
    let format = if !data.attributes.is_empty() {
        handle_struct_attribute(&data.attributes[0])?
    } else {
        None
    };

    // handle struct fields:
    let value = handle_struct_fields(&data.fields, format)?;
    
    // generate tokens:
    let name = data.name;
    Ok(quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #value
            }
        }
    })
}

// Handle the structure fields
fn handle_struct_fields(fields: &Fields, format: Option<Literal>) -> Result<TokenStream> {
    Ok(match format {
        Some(format) => {
            // prepare fields arguments list:
            let mut args: Vec<TokenStream> = vec![];
            match fields {
                Fields::Named(named_fields) => {
                    // for each fields:
                    for field in named_fields.fields.iter() {
                        let name = &field.0.name;
                        args.push(quote!{ #name = &self.#name })
                    }
                },
                _ => return Err(Error::ExpectedNamedFields),
            }

            // generate tokens:
            if !args.is_empty() {
                quote!{ write!(f, #format, #(#args),*) }
            } else {
                quote!{ write!(f, #format) }
            }
        },
        None => quote! {
            write!(f, "{self:?}")
        }
    })
}

// Handle the structure attribute value
fn handle_struct_attribute(attr: &Attribute) -> Result<Option<Literal>> {
    // check the attribute path for correctly format:
    if attr.get_single_path_segment().is_none() { return Err(Error::IncorrectAttribute) };
    
    match &attr.value {
        AttributeValue::Empty => {
            Ok(None)
        },

        AttributeValue::Group(_, tokens) => {
            let token = if let Some(token) = tokens.get(0) { token }else{ return Err(Error::IncorrectAttribute) };

            if let TokenTree::Literal(value) = token {
                Ok(Some(value.clone()))
            }else{ 
                Err(Error::IncorrectAttribute)
            }
        },

        AttributeValue::Equals(_, tokens) => {
            let token = if let Some(token) = tokens.get(0) { token }else{ return Err(Error::IncorrectAttribute) };

            if let TokenTree::Literal(value) = token {
                Ok(Some(value.clone()))
            }else{ 
                Err(Error::IncorrectAttribute)
            }
        }
    }
}
