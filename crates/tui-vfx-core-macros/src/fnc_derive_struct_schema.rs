// <FILE>tui-vfx-core-macros/src/fnc_derive_struct_schema.rs</FILE> - <DESC>Emit a `::tui_vfx_core::SchemaNode::Struct { ... }` token-stream for a `Data::Struct` derive input. Reads top-level /// doc comment (description), top-level #[serde(rename)] (json_name), per-field #[config(...)] (ConfigAttr), per-field /// doc comment (description), per-field #[serde(rename / rename_all)] (json_key), and field types (Option<T> → optional). Emits a vec![SchemaField] body with json_key applied per-field.</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — replace abandoned-refactor stub body with the live version from lib.rs:381-493. Live version has 3-arg signature (ident, s, attrs), top-level description/json_name emission from doc comments + serde, per-field json_key wiring, and pulls /// doc comments via extract_doc_comments.</WCTX>
// <CLOG>0.3.0: MAJOR — replace stub body with live lib.rs version. Signature changed from `derive_struct_schema(ident, s)` to `derive_struct_schema(ident, s, attrs)`. Adds top-level description/json_name emission, per-field json_key emission, /// doc comment integration. 0.2.0: stub.</CLOG>

use quote::quote;
use syn::{spanned::Spanned, Attribute, DataStruct, Fields};

use crate::fnc_apply_rename_all::apply_rename_all;
use crate::fnc_extract_doc_comments::extract_doc_comments;
use crate::fnc_field_meta_tokens::field_meta_tokens;
use crate::fnc_parse_config_attrs::parse_config_attrs;
use crate::fnc_parse_serde_attrs::parse_serde_attrs;

pub(crate) fn derive_struct_schema(
    ident: &syn::Ident,
    s: &DataStruct,
    attrs: &[Attribute],
) -> syn::Result<proc_macro2::TokenStream> {
    let struct_serde = parse_serde_attrs(attrs)?;
    let rename_all = struct_serde.rename_all.as_deref();

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
                let field_serde = parse_serde_attrs(&f.attrs)?;
                if field_serde.skip {
                    continue;
                }
                let doc = extract_doc_comments(&f.attrs);
                let ty = &f.ty;
                let meta = field_meta_tokens(&attr, doc, &field_serde, ty);

                // Compute json_key
                let json_key = if let Some(rename) = &field_serde.rename {
                    quote!(Some(#rename.to_string()))
                } else {
                    let transformed = apply_rename_all(&name, rename_all);
                    if transformed != name {
                        quote!(Some(#transformed.to_string()))
                    } else {
                        quote!(None)
                    }
                };

                let schema = if attr.opaque {
                    quote!(::tui_vfx_core::SchemaNode::Opaque {
                        type_name: stringify!(#ty).replace(' ', ""),
                    })
                } else {
                    quote!(<#ty as ::tui_vfx_core::ConfigSchema>::schema())
                };
                field_tokens.push(quote!({
                    let mut field = ::tui_vfx_core::SchemaField::new(
                        #name,
                        #schema,
                        #meta
                    );
                    field.json_key = #json_key;
                    field
                }));
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
                let field_serde = parse_serde_attrs(&f.attrs)?;
                if field_serde.skip {
                    continue;
                }
                let doc = extract_doc_comments(&f.attrs);
                let ty = &f.ty;
                let meta = field_meta_tokens(&attr, doc, &field_serde, ty);

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

    let description = extract_doc_comments(attrs);
    let desc_token = match description {
        Some(d) => quote!(Some(#d.to_string())),
        None => quote!(None),
    };

    let json_name = if let Some(rename) = &struct_serde.rename {
        quote!(Some(#rename.to_string()))
    } else {
        quote!(None)
    };

    Ok(quote!(::tui_vfx_core::SchemaNode::Struct {
        name: stringify!(#ident).to_string(),
        description: #desc_token,
        json_name: #json_name,
        fields: #fields,
    }))
}

// <FILE>tui-vfx-core-macros/src/fnc_derive_struct_schema.rs</FILE> - <DESC>Emit SchemaNode::Struct token-stream (live 3-arg signature)</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-28</VERS>
