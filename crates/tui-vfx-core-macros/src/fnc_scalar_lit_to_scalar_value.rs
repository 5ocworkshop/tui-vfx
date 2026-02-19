// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_to_scalar_value.rs</FILE> - <DESC>Convert ScalarLit into tui_vfx_core::ScalarValue tokens</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted ScalarLit -> ScalarValue token mapping</CLOG>

use quote::quote;

use crate::types::ScalarLit;

pub(crate) fn scalar_lit_to_scalar_value(lit: &ScalarLit) -> proc_macro2::TokenStream {
    match lit {
        ScalarLit::Bool(b) => quote!(::tui_vfx_core::ScalarValue::Bool(#b)),
        ScalarLit::Char(c) => quote!(::tui_vfx_core::ScalarValue::Char(#c)),
        ScalarLit::String(s) => quote!(::tui_vfx_core::ScalarValue::String(#s.to_string())),
        ScalarLit::Number(n) => quote!(::tui_vfx_core::ScalarValue::Number(#n.to_string())),
    }
}

// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_to_scalar_value.rs</FILE> - <DESC>Convert ScalarLit into tui_vfx_core::ScalarValue tokens</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

