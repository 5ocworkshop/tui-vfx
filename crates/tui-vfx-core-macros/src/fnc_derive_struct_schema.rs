// <FILE>tui-vfx-core-macros/src/fnc_derive_struct_schema.rs</FILE> - <DESC>Derive schema node for structs</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add None defaults for new schema fields (backward compatibility only)</CLOG>

use quote::quote;
use syn::{spanned::Spanned, DataStruct, Fields};

use crate::fnc_field_meta_tokens::field_meta_tokens;
use crate::fnc_parse_config_attrs::parse_config_attrs;

pub(crate) fn derive_struct_schema(
    ident: &syn::Ident,
    s: &DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = match &s.fields {
        Fields::Named(named) => {
            let mut field_tokens = Vec::new();
            for f in &named.named {
                let name = f
                    .ident
                    .as_ref()
                    .ok_or_else(|| syn::Error::new(f.span(), "Expected named field"))?
                    .to_string();
                let attr = parse_config_attrs(&f.attrs)?;
                if attr.hidden {
                    continue;
                }
                let meta = field_meta_tokens(&attr);

                let ty = &f.ty;
                let schema = if attr.opaque {
                    quote!(::tui_vfx_core::SchemaNode::Opaque {
                        type_name: stringify!(#ty).replace(' ', ""),
                    })
                } else {
                    quote!(<#ty as ::tui_vfx_core::ConfigSchema>::schema())
                };
                field_tokens.push(quote!(::tui_vfx_core::SchemaField::new(
                    #name,
                    #schema,
                    #meta
                )));
            }
            quote!(vec![#(#field_tokens),*])
        }
        Fields::Unnamed(unnamed) => {
            let mut field_tokens = Vec::new();
            for (idx, f) in unnamed.unnamed.iter().enumerate() {
                let name = idx.to_string();
                let attr = parse_config_attrs(&f.attrs)?;
                if attr.hidden {
                    continue;
                }
                let meta = field_meta_tokens(&attr);

                let ty = &f.ty;
                let schema = if attr.opaque {
                    quote!(::tui_vfx_core::SchemaNode::Opaque {
                        type_name: stringify!(#ty).replace(' ', ""),
                    })
                } else {
                    quote!(<#ty as ::tui_vfx_core::ConfigSchema>::schema())
                };
                field_tokens.push(quote!(::tui_vfx_core::SchemaField::new(
                    #name,
                    #schema,
                    #meta
                )));
            }
            quote!(vec![#(#field_tokens),*])
        }
        Fields::Unit => quote!(vec![]),
    };

    Ok(quote!(::tui_vfx_core::SchemaNode::Struct {
        name: stringify!(#ident).to_string(),
        description: None,
        json_name: None,
        fields: #fields,
    }))
}

// <FILE>tui-vfx-core-macros/src/fnc_derive_struct_schema.rs</FILE> - <DESC>Derive schema node for structs</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>

