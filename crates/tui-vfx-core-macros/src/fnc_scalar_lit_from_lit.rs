// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_from_lit.rs</FILE> - <DESC>Convert a `syn::Lit` literal into the macro-internal `ScalarLit` form. Used by parse_scalar_lit when extracting `#[config(default = X)]` / `#[config(min = X)]` / `#[config(max = X)]` literal values.</DESC>
// <VERS>VERSION: 0.2.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — promote from abandoned-refactor stub to live module. Body matches the live lib.rs version (logically identical; differs only in the formatter's line-wrap style for the error arm).</WCTX>
// <CLOG>0.2.0: MAJOR — promote from abandoned-refactor stub to live module. Body unchanged in semantics. Will be reachable from lib.rs via `mod fnc_scalar_lit_from_lit;` in US-013.</CLOG>

use syn::Lit;

use crate::col_clean_number::clean_number;
use crate::types::ScalarLit;

pub(crate) fn scalar_lit_from_lit(lit: &Lit) -> syn::Result<ScalarLit> {
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

// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_from_lit.rs</FILE> - <DESC>Convert syn::Lit → ScalarLit</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2026-04-28</VERS>
