// <FILE>tui-vfx-core-macros/src/fnc_impl_config_schema.rs</FILE> - <DESC>Top-level derive entry: dispatch to derive_struct_schema or derive_enum_schema based on the input's data shape, then wrap the resulting SchemaNode body in an `impl ConfigSchema for #ident` block plus an inherent `pub fn schema()` for the consumer ergonomics.</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — replace abandoned-refactor stub body with the live version from lib.rs:350-379. The live signatures pass `&input.attrs` to derive_struct_schema and derive_enum_schema so they can read top-level /// doc comments and serde rename_all/tag attrs.</WCTX>
// <CLOG>0.3.0: MAJOR — replace stub body with live lib.rs version. derive_struct_schema and derive_enum_schema now receive `&input.attrs` as a third arg (was 2-arg in the stub). 0.1.1: stub with 2-arg derive_*_schema calls.</CLOG>

use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput};

use crate::fnc_derive_enum_schema::derive_enum_schema;
use crate::fnc_derive_struct_schema::derive_struct_schema;

pub(crate) fn impl_config_schema(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let schema_body = match &input.data {
        Data::Struct(s) => derive_struct_schema(ident, s, &input.attrs)?,
        Data::Enum(e) => derive_enum_schema(ident, e, &input.attrs)?,
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

// <FILE>tui-vfx-core-macros/src/fnc_impl_config_schema.rs</FILE> - <DESC>Top-level derive entry; dispatches to derive_struct_schema / derive_enum_schema</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-28</VERS>
