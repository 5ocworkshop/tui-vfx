// <FILE>tui-vfx-core-macros/src/fnc_extract_doc_comments.rs</FILE> - <DESC>Extract `///` doc-comment text from a syn::Attribute slice and join the lines as the `description` field of a generated SchemaNode/SchemaVariant/FieldMeta. Returns None when no doc comments are present.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — relocate extract_doc_comments out of inline lib.rs.</WCTX>
// <CLOG>1.0.0: initial — body lifted from lib.rs:135-155 verbatim.</CLOG>

use syn::Attribute;

/// Extract doc comments from attributes.
///
/// Walks the attribute slice, picks out `#[doc = "..."]` entries (the form
/// `///` comments lower to), trims and joins them with newlines. Returns
/// `None` when no doc-comment attributes are present so callers can emit
/// `description: None` cleanly.
pub(crate) fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let syn::Meta::NameValue(meta) = &attr.meta
                && let syn::Expr::Lit(expr_lit) = &meta.value
                && let syn::Lit::Str(lit_str) = &expr_lit.lit
            {
                return Some(lit_str.value().trim().to_string());
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

// <FILE>tui-vfx-core-macros/src/fnc_extract_doc_comments.rs</FILE> - <DESC>Extract /// doc-comment text from syn::Attribute slice</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
