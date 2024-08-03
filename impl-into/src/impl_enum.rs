use crate::prelude::*;
use proc_macro2::{ TokenStream, Ident };
use venial::{ Enum, AttributeValue, EnumVariant, Fields };
use quote::quote;

// Implementation of trait 'Into<T>' for enumerations
pub(crate) fn impl_into_enum(data: Enum) -> Result<TokenStream> {
    let name = &data.name;
    let mut output = quote!{};

    // reading enum attributes:
    for attr in data.attributes.into_iter() {
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

    // reading enum variants attributes:
    for (variant, _) in data.variants.into_iter() {
        let value = handle_enum_variant(&name, variant)?;
        output.extend(value);
    }

    Ok(output)
}

// Handle the enumeration variants
fn handle_enum_variant(enum_name: &Ident, variant: &EnumVariant) -> Result<TokenStream> {
    let name = &variant.name;
    let mut output = quote!{};
    
    // reading attributes:
    for attr in variant.attributes.iter() {
        // parse attribute arguments:
        let (ty, expr) = match &attr.value {
            AttributeValue::Empty => {
                let ty = parse_enum_variant_first_field_type(&variant.fields)?;

                (quote!{ Option<#ty> }, quote!{ if let Self::#name(v) = self { Some(v) }else{ None } })
            },
    
            AttributeValue::Equals(_, tokens) => {
                let ty = parse_enum_variant_first_field_type(&variant.fields)?;
                let expr = parse_attr_argument(&tokens[0])?;

                (ty, expr)
            },

            AttributeValue::Group(_, tokens) => {
                if tokens.len() > 1 {
                    parse_attr_arguments(&tokens)?
                } else {
                    let ty = parse_enum_variant_first_field_type(&variant.fields)?;
                    let expr = parse_attr_argument(&tokens[0])?;

                    (ty, expr)
                }
            }
        };

        // generate tokens:
        output.extend(quote! {
            impl ::core::convert::Into<#ty> for #enum_name {
                fn into(self) -> #ty {
                    #expr
                }
            }
        })
    }

    Ok(output)
}


// Parsing the enum variant fiest field type
fn parse_enum_variant_first_field_type(fields: &Fields) -> Result<TokenStream> {
    let ty = match &fields {
        Fields::Tuple(tuple_fields) => {
            if let Some((field, _)) = &tuple_fields.fields.get(0) {
                let ty = &field.ty;
                quote!{ #ty }
            } else {
                return Err(Error::IncorrectAttributeVariant)   
            }
        },

        Fields::Named(named_fields) => {
            if let Some((field, _)) = &named_fields.fields.get(0) {
                let ty = &field.ty;
                quote!{ #ty }
            } else {
                return Err(Error::IncorrectAttributeVariant)   
            }
        },

        _ => return Err(Error::IncorrectAttributeVariant)
    };

    Ok(ty)
}