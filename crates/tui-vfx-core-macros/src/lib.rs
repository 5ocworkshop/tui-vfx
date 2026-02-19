// <FILE>tui-vfx-core-macros/src/lib.rs</FILE> - <DESC>Proc-macro derives for ConfigSchema</DESC>
// <VERS>VERSION: 0.4.1 - 2025-12-31</VERS>
// <WCTX>Schema reference auto-generation - Phase 5</WCTX>
// <CLOG>Fix parsing of serde(default = "fn") and serde(skip_serializing_if = "fn")</CLOG>

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Lit, parse_macro_input,
    spanned::Spanned,
};

#[proc_macro_derive(ConfigSchema, attributes(config))]
pub fn derive_config_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_config_schema(&input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

#[derive(Default, Debug, Clone)]
struct ConfigAttr {
    hidden: bool,
    opaque: bool,
    help: Option<String>,
    default: Option<ScalarLit>,
    min: Option<ScalarLit>,
    max: Option<ScalarLit>,
}

#[derive(Debug, Clone)]
enum ScalarLit {
    Bool(bool),
    Number(String),
    String(String),
    Char(char),
}

#[derive(Default, Debug, Clone)]
struct SerdeAttr {
    rename: Option<String>,     // #[serde(rename = "...")]
    rename_all: Option<String>, // #[serde(rename_all = "snake_case")]
    skip: bool,                 // #[serde(skip)]
    default: bool,              // #[serde(default)]
    tag: Option<String>,        // #[serde(tag = "type")]
}

fn parse_config_attrs(attrs: &[Attribute]) -> syn::Result<ConfigAttr> {
    let mut out = ConfigAttr::default();
    for attr in attrs {
        if !attr.path().is_ident("config") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("hidden") {
                out.hidden = true;
                return Ok(());
            }
            if meta.path.is_ident("opaque") {
                out.opaque = true;
                return Ok(());
            }
            if meta.path.is_ident("help") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.help = Some(lit.value());
                return Ok(());
            }

            if meta.path.is_ident("default") {
                let expr: Expr = meta.value()?.parse()?;
                out.default = Some(parse_scalar_lit(&expr)?);
                return Ok(());
            }
            if meta.path.is_ident("min") {
                let expr: Expr = meta.value()?.parse()?;
                out.min = Some(parse_scalar_lit(&expr)?);
                return Ok(());
            }
            if meta.path.is_ident("max") {
                let expr: Expr = meta.value()?.parse()?;
                out.max = Some(parse_scalar_lit(&expr)?);
                return Ok(());
            }

            Err(meta.error("Unsupported #[config(...)] argument"))
        })?;
    }
    Ok(out)
}

fn parse_scalar_lit(expr: &Expr) -> syn::Result<ScalarLit> {
    match expr {
        Expr::Lit(l) => scalar_lit_from_lit(&l.lit),
        Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) => {
            let Expr::Lit(inner) = &*u.expr else {
                return Err(syn::Error::new(expr.span(), "Expected numeric literal"));
            };
            let s = match &inner.lit {
                Lit::Int(i) => clean_number(i.base10_digits()),
                Lit::Float(f) => clean_number(f.base10_digits()),
                other => {
                    return Err(syn::Error::new(other.span(), "Expected numeric literal"));
                }
            };
            Ok(ScalarLit::Number(format!("-{}", s)))
        }
        _ => Err(syn::Error::new(
            expr.span(),
            "Expected a literal (bool/char/string/number)",
        )),
    }
}

fn scalar_lit_from_lit(lit: &Lit) -> syn::Result<ScalarLit> {
    match lit {
        Lit::Bool(b) => Ok(ScalarLit::Bool(b.value)),
        Lit::Char(c) => Ok(ScalarLit::Char(c.value())),
        Lit::Str(s) => Ok(ScalarLit::String(s.value())),
        Lit::Int(i) => Ok(ScalarLit::Number(clean_number(i.base10_digits()))),
        Lit::Float(f) => Ok(ScalarLit::Number(clean_number(f.base10_digits()))),
        other => Err(syn::Error::new(
            other.span(),
            "Unsupported literal for #[config]",
        )),
    }
}

fn clean_number(digits: &str) -> String {
    digits.replace('_', "")
}

/// Extract doc comments from attributes
fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &meta.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value().trim().to_string());
                        }
                    }
                }
            }
            None
        })
        .collect();

    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n"))
    }
}

/// Parse serde attributes
fn parse_serde_attrs(attrs: &[Attribute]) -> syn::Result<SerdeAttr> {
    let mut out = SerdeAttr::default();
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.rename = Some(lit.value());
            } else if meta.path.is_ident("rename_all") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.rename_all = Some(lit.value());
            } else if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("default") {
                // #[serde(default)] or #[serde(default = "function_name")]
                // We only care about the boolean flag, ignore the function name
                out.default = true;
                // Try to consume the value if present, but ignore errors
                let _ = meta.value().and_then(|v| v.parse::<syn::Expr>());
            } else if meta.path.is_ident("tag") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                out.tag = Some(lit.value());
            } else if meta.path.is_ident("skip_serializing_if") {
                // Ignore skip_serializing_if = "function_name"
                // Just consume the value to avoid parse errors
                let _ = meta.value().and_then(|v| v.parse::<syn::Expr>());
            } else if meta.path.is_ident("alias") {
                // Ignore alias = "name"
                // Just consume the value to avoid parse errors
                let _ = meta.value().and_then(|v| v.parse::<syn::LitStr>());
            } else if meta.path.is_ident("deserialize_with")
                || meta.path.is_ident("serialize_with")
                || meta.path.is_ident("with")
                || meta.path.is_ident("bound")
                || meta.path.is_ident("borrow")
                || meta.path.is_ident("getter")
                || meta.path.is_ident("other")
                || meta.path.is_ident("from")
                || meta.path.is_ident("try_from")
                || meta.path.is_ident("into")
                || meta.path.is_ident("content")
                || meta.path.is_ident("untagged")
                || meta.path.is_ident("flatten")
                || meta.path.is_ident("transparent")
                || meta.path.is_ident("deny_unknown_fields")
                || meta.path.is_ident("crate")
                || meta.path.is_ident("expecting")
            {
                // Consume the value if present for known serde attributes we don't use
                let _ = meta.value().and_then(|v| v.parse::<syn::Expr>());
            }
            // Ignore other serde attributes (flag-style with no value)
            Ok(())
        })?;
    }
    Ok(out)
}

/// Convert Rust identifier to snake_case for JSON
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Apply rename_all transformation to a field name
fn apply_rename_all(name: &str, rename_all: Option<&str>) -> String {
    match rename_all {
        Some("snake_case") => to_snake_case(name),
        Some("camelCase") => {
            let snake = to_snake_case(name);
            let parts: Vec<&str> = snake.split('_').collect();
            if parts.is_empty() {
                return String::new();
            }
            let mut result = parts[0].to_string();
            for part in &parts[1..] {
                if !part.is_empty() {
                    let mut chars = part.chars();
                    if let Some(first) = chars.next() {
                        result.push(first.to_uppercase().next().unwrap());
                        result.push_str(chars.as_str());
                    }
                }
            }
            result
        }
        Some("PascalCase") => {
            let snake = to_snake_case(name);
            snake
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        Some(first) => {
                            format!("{}{}", first.to_uppercase(), chars.as_str())
                        }
                        None => String::new(),
                    }
                })
                .collect()
        }
        Some("SCREAMING_SNAKE_CASE") => to_snake_case(name).to_uppercase(),
        _ => name.to_string(),
    }
}

/// Check if a type is Option<T>
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn scalar_lit_to_scalar_value(lit: &ScalarLit) -> proc_macro2::TokenStream {
    match lit {
        ScalarLit::Bool(b) => quote!(::tui_vfx_core::ScalarValue::Bool(#b)),
        ScalarLit::Char(c) => quote!(::tui_vfx_core::ScalarValue::Char(#c)),
        ScalarLit::String(s) => quote!(::tui_vfx_core::ScalarValue::String(#s.to_string())),
        ScalarLit::Number(n) => quote!(::tui_vfx_core::ScalarValue::Number(#n.to_string())),
    }
}

fn field_meta_tokens(
    attr: &ConfigAttr,
    doc: Option<String>,
    serde: &SerdeAttr,
    ty: &syn::Type,
) -> proc_macro2::TokenStream {
    let help = match &attr.help {
        Some(h) => quote!(Some(#h.to_string())),
        None => quote!(None),
    };
    let description = match doc {
        Some(d) => quote!(Some(#d.to_string())),
        None => quote!(None),
    };
    let default = match &attr.default {
        Some(d) => {
            let v = scalar_lit_to_scalar_value(d);
            quote!(Some(#v))
        }
        None => quote!(None),
    };
    let range = match (&attr.min, &attr.max) {
        (None, None) => quote!(None),
        (min, max) => {
            let min_ts = match min {
                Some(m) => {
                    let v = scalar_lit_to_scalar_value(m);
                    quote!(Some(#v))
                }
                None => quote!(None),
            };
            let max_ts = match max {
                Some(m) => {
                    let v = scalar_lit_to_scalar_value(m);
                    quote!(Some(#v))
                }
                None => quote!(None),
            };
            quote!(Some(::tui_vfx_core::Range::new(#min_ts, #max_ts)))
        }
    };

    let optional = is_option_type(ty) || serde.default;

    quote!(::tui_vfx_core::FieldMeta {
        help: #help,
        description: #description,
        default: #default,
        range: #range,
        json_key: None,
        optional: #optional,
    })
}

fn impl_config_schema(input: &DeriveInput) -> syn::Result<TokenStream> {
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

fn derive_struct_schema(
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

fn derive_enum_schema(
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

// <FILE>tui-vfx-core-macros/src/lib.rs</FILE> - <DESC>Proc-macro derives for ConfigSchema</DESC>
// <VERS>END OF VERSION: 0.4.1 - 2025-12-31</VERS>
