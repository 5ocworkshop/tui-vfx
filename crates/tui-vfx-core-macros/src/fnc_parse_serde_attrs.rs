// <FILE>tui-vfx-core-macros/src/fnc_parse_serde_attrs.rs</FILE> - <DESC>Parse the subset of `#[serde(...)]` attributes the ConfigSchema derive consumes (rename, rename_all, skip, default, tag) into a SerdeAttr. Other serde attributes are intentionally consumed-and-discarded so syn doesn't error out on unknown nested-meta args.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — relocate parse_serde_attrs out of inline lib.rs.</WCTX>
// <CLOG>1.0.0: initial — body lifted from lib.rs:158-216 verbatim.</CLOG>

use syn::Attribute;

use crate::types::SerdeAttr;

/// Parse serde attributes.
///
/// Only the five attributes the ConfigSchema derive cares about are stored
/// (rename, rename_all, skip, default, tag). The rest are consumed silently
/// to avoid `parse_nested_meta` errors on unknown args.
pub(crate) fn parse_serde_attrs(attrs: &[Attribute]) -> syn::Result<SerdeAttr> {
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

// <FILE>tui-vfx-core-macros/src/fnc_parse_serde_attrs.rs</FILE> - <DESC>Parse the serde-attribute subset the derive consumes</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
