// <FILE>tui-vfx-core-macros/src/col_is_option_type.rs</FILE> - <DESC>Pure leaf helper: heuristic check whether a `syn::Type` is `Option<T>` (last path segment named "Option"). Used to set `FieldMeta.optional = true` for derive-emitted Option fields.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — relocate is_option_type out of inline lib.rs.</WCTX>
// <CLOG>1.0.0: initial — body lifted from lib.rs:278-285 verbatim.</CLOG>

/// Check if a type is `Option<T>`.
///
/// Heuristic: matches the last path segment's identifier against `"Option"`.
/// Will not match aliased imports (`type MyOption = Option;`); that case is
/// rare enough in derive-eligible types that the heuristic is acceptable.
pub(crate) fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident == "Option";
    }
    false
}

// <FILE>tui-vfx-core-macros/src/col_is_option_type.rs</FILE> - <DESC>Pure leaf helper: detect Option<T> by last path segment</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
