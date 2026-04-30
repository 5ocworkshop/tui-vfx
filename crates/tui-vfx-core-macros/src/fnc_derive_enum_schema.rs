// <FILE>tui-vfx-core-macros/src/fnc_derive_enum_schema.rs</FILE> - <DESC>Emit a `::tui_vfx_core::SchemaNode::Enum { ... }` token-stream for a `Data::Enum` derive input. Reads top-level /// doc (description), top-level #[serde(rename / rename_all / tag)] (json_name, json_value-per-variant via apply_rename_all, tag_field), per-variant /// doc (variant description), per-variant #[serde(rename)] (variant json_value), and per-variant fields with the same shape as derive_struct_schema.</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — replace abandoned-refactor stub body with the live version from lib.rs:495-656. Live version has 3-arg signature (ident, e, attrs), top-level description/json_name/tag_field emission, per-variant description (from /// doc) and json_value (from rename or rename_all), and per-field json_key wiring.</WCTX>
// <CLOG>0.3.0: MAJOR — replace stub body with live lib.rs version. Signature changed from `derive_enum_schema(ident, e)` to `derive_enum_schema(ident, e, attrs)`. Adds top-level + per-variant description/json_value emission, per-field json_key, /// doc comment integration. 0.2.0: stub.</CLOG>

use quote::quote;
use syn::{Attribute, DataEnum, Fields, spanned::Spanned};

use crate::fnc_apply_rename_all::apply_rename_all;
use crate::fnc_extract_doc_comments::extract_doc_comments;
use crate::fnc_field_meta_tokens::field_meta_tokens;
use crate::fnc_parse_config_attrs::parse_config_attrs;
use crate::fnc_parse_serde_attrs::parse_serde_attrs;

pub(crate) fn derive_enum_schema(
    ident: &syn::Ident,
    e: &DataEnum,
    attrs: &[Attribute],
) -> syn::Result<proc_macro2::TokenStream> {
    let enum_serde = parse_serde_attrs(attrs)?;
    let rename_all = enum_serde.rename_all.as_deref();
    let mut variants = Vec::new();

    for v in &e.variants {
        let variant_attr = parse_config_attrs(&v.attrs)?;
        if variant_attr.hidden {
            continue;
        }
        let variant_serde = parse_serde_attrs(&v.attrs)?;
        let v_name = v.ident.to_string();
        let variant_doc = extract_doc_comments(&v.attrs);
        let variant_desc_token = match variant_doc {
            Some(d) => quote!(Some(#d.to_string())),
            None => quote!(None),
        };

        // Compute json_value
        let json_value = if let Some(rename) = &variant_serde.rename {
            quote!(Some(#rename.to_string()))
        } else {
            let transformed = apply_rename_all(&v_name, rename_all);
            if transformed != v_name {
                quote!(Some(#transformed.to_string()))
            } else {
                quote!(None)
            }
        };

        match &v.fields {
            Fields::Unit => {
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Unit {
                    name: #v_name.to_string(),
                    description: #variant_desc_token,
                    json_value: #json_value,
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
                    items.push(quote!(::tui_vfx_core::SchemaField::new(
                        #name,
                        #schema,
                        #meta
                    )));
                }
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Tuple {
                    name: #v_name.to_string(),
                    description: #variant_desc_token,
                    json_value: #json_value,
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
                    let field_serde = parse_serde_attrs(&f.attrs)?;
                    if field_serde.skip {
                        continue;
                    }
                    let doc = extract_doc_comments(&f.attrs);
                    let ty = &f.ty;
                    let meta = field_meta_tokens(&attr, doc, &field_serde, ty);

                    // Compute json_key for variant fields (use variant's rename_all if present)
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
                    fields.push(quote!({
                        let mut field = ::tui_vfx_core::SchemaField::new(
                            #name,
                            #schema,
                            #meta
                        );
                        field.json_key = #json_key;
                        field
                    }));
                }
                variants.push(quote!(::tui_vfx_core::SchemaVariant::Struct {
                    name: #v_name.to_string(),
                    description: #variant_desc_token,
                    json_value: #json_value,
                    fields: vec![#(#fields),*],
                }));
            }
        }
    }

    let description = extract_doc_comments(attrs);
    let desc_token = match description {
        Some(d) => quote!(Some(#d.to_string())),
        None => quote!(None),
    };

    let json_name = if let Some(rename) = &enum_serde.rename {
        quote!(Some(#rename.to_string()))
    } else {
        quote!(None)
    };

    let tag_field = if let Some(tag) = &enum_serde.tag {
        quote!(Some(#tag.to_string()))
    } else {
        quote!(None)
    };

    Ok(quote!(::tui_vfx_core::SchemaNode::Enum {
        name: stringify!(#ident).to_string(),
        description: #desc_token,
        json_name: #json_name,
        tag_field: #tag_field,
        variants: vec![#(#variants),*],
    }))
}

// <FILE>tui-vfx-core-macros/src/fnc_derive_enum_schema.rs</FILE> - <DESC>Emit SchemaNode::Enum token-stream (live 3-arg signature)</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-28</VERS>
