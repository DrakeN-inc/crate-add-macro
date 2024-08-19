use crate::prelude::*;
use proc_macro2::{ TokenStream, TokenTree, Ident, Literal, Span };
use venial::{ Enum, EnumVariant, Fields, Attribute, AttributeValue };
use quote::quote;

// Implementation of trait 'Display' for Enum
pub(crate) fn impl_display_enum(data: Enum) -> Result<TokenStream> {
    // generate values for block 'match &self { ... }':
    let mut values = quote!{};
    for (variant, _) in data.variants.into_iter() {
        let value = handle_enum_variant(&variant)?;
        values.extend(quote!{ #value, });
    }
    
    // generate tokens:
    let name = data.name;
    Ok(quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self {
                    #values
                }
            }
        }
    })
}

// Handle the enum variant
fn handle_enum_variant(variant: &EnumVariant) -> Result<TokenStream> {
    let name = &variant.name;
    
    // for each attributes & get format string:
    let mut format = None;
    for attr in &variant.attributes {
        if check_attr_name(&attr, "display") {
            format = handle_enum_variant_attribute(&attr)?;
            break;
        }
    };

    // generate tokens:
    match &variant.fields {
        Fields::Unit => {
            let format = format.unwrap_or( Literal::string(&name.to_string()) );
            Ok(quote!{ Self::#name => write!(f, #format) })
        },

        Fields::Tuple(tuple_fields) => {
            let mut is_none = false;
            let format = format.unwrap_or_else(|| {is_none = true; Literal::string(&name.to_string())});

            if is_none && tuple_fields.fields.len() == 1 {
                Ok(quote!{ Self::#name(v) => write!(f, "{v}") })
            } else {
                let args = tuple_fields.fields
                    .into_iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("v{i}"), Span::call_site()))
                    .collect::<Vec<_>>();
    
                Ok(quote!{ Self::#name(#(#args),*) => write!(f, #format, #(#args),*) })
            }
        },

        Fields::Named(named_fields) => {
            let format = format.unwrap_or( Literal::string(&name.to_string()) );
            let args = named_fields.fields
                .into_iter()
                .map(|(field, _)| field.name.clone())
                .collect::<Vec<_>>();

            Ok(quote!{ Self::#name{#(#args),*} => write!(f, #format) })
        },
    }
}

// Handle the macros attribute
fn handle_enum_variant_attribute(attr: &Attribute) -> Result<Option<Literal>> {   
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
