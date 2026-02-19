// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_from_lit.rs</FILE> - <DESC>Parse syn::Lit into ScalarLit</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted scalar literal conversion</CLOG>

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
        other => Err(syn::Error::new(other.span(), "Unsupported literal for #[config]")),
    }
}

// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_from_lit.rs</FILE> - <DESC>Parse syn::Lit into ScalarLit</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
