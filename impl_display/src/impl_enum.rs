use crate::prelude::*;
use proc_macro2::Literal;
use syn::{ DataEnum, Variant, Meta, MetaNameValue, Expr, ExprLit, Lit };


// Implementation for enumeration
pub(crate) fn impl_display_enum(name: Ident, data: DataEnum) -> TokenStream {
    // get names:
    let names = data.variants
        .iter()
        .map(|v| v.ident.clone())
        .collect::<Vec<_>>();

    // prepare values:
    let values = data.variants
        .into_iter()
        .map(|v| prepare_value(v))
        .collect::<Vec<_>>();

    quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}",
                    match &self {
                        #(
                            Self::#names => format!(#values),
                        )*
                    }
                )
            }
        }
    }
    .into()
}

// Preparing the enum variant values
fn prepare_value(var: Variant) -> Literal {
    if !var.attrs.is_empty() {
        if let Meta::NameValue(MetaNameValue { value, ..}) = &var.attrs[0].meta {
            if let Expr::Lit(ExprLit{ lit, .. }) = value {
                if let Lit::Str(s) = lit {
                    return s.token();
                }
            }
        } else {
        }
    } else {
        return Literal::string(&var.ident.to_string());
    }

    panic!("Expected the literally string like this #[display = \"...\"]")
}
