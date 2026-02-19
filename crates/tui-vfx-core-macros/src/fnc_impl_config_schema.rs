// <FILE>tui-vfx-core-macros/src/fnc_impl_config_schema.rs</FILE> - <DESC>Generate ConfigSchema impl tokens</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted top-level derive implementation</CLOG>

use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput};

use crate::fnc_derive_enum_schema::derive_enum_schema;
use crate::fnc_derive_struct_schema::derive_struct_schema;

pub(crate) fn impl_config_schema(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let schema_body = match &input.data {
        Data::Struct(s) => derive_struct_schema(ident, s)?,
        Data::Enum(e) => derive_enum_schema(ident, e)?,
        Data::Union(u) => {
            return Err(syn::Error::new(
                u.union_token.span(),
                "ConfigSchema cannot be derived for unions",
            ));
        }
    };

    Ok(quote!(
        impl #impl_generics ::tui_vfx_core::ConfigSchema for #ident #ty_generics #where_clause {
            fn schema() -> ::tui_vfx_core::SchemaNode {
                #schema_body
            }
        }

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn schema() -> ::tui_vfx_core::SchemaNode {
                <Self as ::tui_vfx_core::ConfigSchema>::schema()
            }
        }
    )
    .into())
}

// <FILE>tui-vfx-core-macros/src/fnc_impl_config_schema.rs</FILE> - <DESC>Generate ConfigSchema impl tokens</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

