// <FILE>tui-vfx-core-macros/src/fnc_derive_enum_schema.rs</FILE> - <DESC>Derive schema node for enums</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add None defaults for new schema fields (backward compatibility only)</CLOG>

use quote::quote;
use syn::{spanned::Spanned, DataEnum, Fields};

use crate::fnc_field_meta_tokens::field_meta_tokens;
use crate::fnc_parse_config_attrs::parse_config_attrs;

pub(crate) fn derive_enum_schema(ident: &syn::Ident, e: &DataEnum) -> syn::Result<proc_macro2::TokenStream> {
    let mut variants = Vec::new();

    for v in &e.variants {
        let variant_attr = parse_config_attrs(&v.attrs)?;
        if variant_attr.hidden {
            continue;
        }
        let v_name = v.ident.to_string();
        match &v.fields {
            Fields::Unit => {
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Unit {
                    name: #v_name.to_string(),
                    description: None,
                    json_value: None,
                }));
            }
            Fields::Unnamed(unnamed) => {
                let mut items = Vec::new();
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
                    items.push(quote!(::tui_vfx_core::SchemaField::new(
                        #name,
                        #schema,
                        #meta
                    )));
                }
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Tuple {
                    name: #v_name.to_string(),
                    description: None,
                    json_value: None,
                    items: vec![#(#items),*],
                }));
            }
            Fields::Named(named) => {
                let mut fields = Vec::new();
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
                    fields.push(quote!(::tui_vfx_core::SchemaField::new(
                        #name,
                        #schema,
                        #meta
                    )));
                }
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Struct {
                    name: #v_name.to_string(),
                    description: None,
                    json_value: None,
                    fields: vec![#(#fields),*],
                }));
            }
        }
    }

    Ok(quote!(::tui_vfx_core::SchemaNode::Enum {
        name: stringify!(#ident).to_string(),
        description: None,
        json_name: None,
        tag_field: None,
        variants: vec![#(#variants),*],
    }))
}

// <FILE>tui-vfx-core-macros/src/fnc_derive_enum_schema.rs</FILE> - <DESC>Derive schema node for enums</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>

