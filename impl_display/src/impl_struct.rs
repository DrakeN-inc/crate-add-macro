use crate::prelude::*;
use syn::DataStruct;


// Implementation for structure
pub(crate) fn impl_display_struct(name: Ident, data: DataStruct) -> TokenStream {
    let fields = data.fields
        .into_iter()
        .map(|v| v.ident)
        .collect::<Vec<_>>();

    quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut s = String::new();

                #(
                    s.push_str( &self.#fields.to_string() );
                    s.push('-');
                )*

                write!(f, "{s}")
            }
        }
    }
    .into()
}
