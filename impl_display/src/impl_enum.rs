use crate::prelude::*;
use proc_macro2::{ TokenStream, TokenTree, Ident, Literal, Span };
use venial::{ Enum, EnumVariant, Fields, Attribute, AttributeValue };
use quote::quote;

// Implementation of trait 'Display' for Enum
pub(crate) fn impl_display_enum(data: Enum) -> Result<TokenStream> {
    // generate values for block 'match &self { ... }':
    let mut values = quote!{};
    for (variant, _) in data.variants.into_iter() {
        values.extend( handle_enum_variant(variant.clone()) );
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
fn handle_enum_variant(variant: EnumVariant) -> Result<TokenStream> {
    let name = &variant.name;
    
    // handle macro attribute & get format string:
    let mut attrs = variant.attributes.into_iter();
    let format = match attrs.next() {
        Some(attr) => handle_enum_variant_attribute(name.clone(), attr)?,
        None    => Literal::string(&name.to_string())
    };

    // generate tokens:
    match &variant.fields {
        Fields::Unit => {
            Ok(quote! {
                Self::#name => write!(f, #format),
            })
        },

        Fields::Tuple(tuple_fields) => {
            let mut args = vec![];
            for i in 0..tuple_fields.fields.len() {
                args.push( Ident::new(&format!("v{i}"), Span::call_site()) );
            }

            Ok(quote! {
                Self::#name(#(#args),*) => write!(f, #format, #(#args),*),
            })
        },

        Fields::Named(named_fields) => {
            let args = named_fields.fields
                .into_iter()
                .map(|(field, _)| field.name.clone())
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#name{#(#args),*} => write!(f, #format),
            })
        },
    }
}

// Handle the macro attribute
fn handle_enum_variant_attribute(name: Ident, attr: Attribute) -> Result<Literal> {
    // check the attribute path for correctly format:
    if attr.get_single_path_segment().is_none() { return Err(Error::IncorrectAttribute) };
    
    match &attr.value {
        AttributeValue::Empty => {
            Ok(Literal::string(&name.to_string()))
        },

        AttributeValue::Group(_, tokens) => {
            let token = if let Some(token) = tokens.get(0) { token }else{ return Err(Error::IncorrectAttribute) };

            if let TokenTree::Literal(value) = token {
                Ok(value.clone())
            }else{ 
                Err(Error::IncorrectAttribute)
            }
        },

        AttributeValue::Equals(_, tokens) => {
            let token = if let Some(token) = tokens.get(0) { token }else{ return Err(Error::IncorrectAttribute) };

            if let TokenTree::Literal(value) = token {
                Ok(value.clone())
            }else{ 
                Err(Error::IncorrectAttribute)
            }
        }
    }
}
