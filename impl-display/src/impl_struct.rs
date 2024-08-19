use crate::prelude::*;
use proc_macro2::{ TokenStream, TokenTree, Literal };
use venial::{ Struct, Attribute, AttributeValue, Fields };
use quote::quote;

// Implementation of trait 'Display' for Struct
pub(crate) fn impl_display_struct(data: Struct) -> Result<TokenStream> {
    // for each attributes & get format string:
    let mut format = None;
    for attr in &data.attributes {
        if check_attr_name(&attr, "display") {
            format = handle_struct_attribute(&attr)?;
            break;
        }
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
    if let Some(format) = format {
        // prepare fields list:
        let args = match fields {
            Fields::Named(named_fields) => {
                named_fields.fields
                    .into_iter()
                    .map(|(field, _)| {
                        let name = &field.name;
                        quote!{ #name = &self.#name }
                    })
                    .collect::<Vec<_>>()
            },
            _ => return Err(Error::ExpectedNamedFields),
        };

        // generate tokens:
        let tokens = if !args.is_empty() {
            quote!{ write!(f, #format, #(#args),*) }
        } else {
            quote!{ write!(f, #format) }
        };

        Ok(tokens)
    } else {
        Ok(quote!{ write!(f, "{self:?}") })
    }
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
