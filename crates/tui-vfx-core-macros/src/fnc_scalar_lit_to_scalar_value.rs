// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_to_scalar_value.rs</FILE> - <DESC>Convert a ScalarLit into the `::tui_vfx_core::ScalarValue::{Bool, Char, String, Number}` token-stream form used by the derive macro when emitting FieldMeta default/min/max literals.</DESC>
// <VERS>VERSION: 0.2.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — promote from abandoned-refactor stub to live module. Body matches lib.rs:287-294 verbatim.</WCTX>
// <CLOG>0.2.0: MAJOR — promote from abandoned-refactor stub to live module. Body unchanged. Reachable from lib.rs in US-013.</CLOG>

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

// <FILE>tui-vfx-core-macros/src/fnc_scalar_lit_to_scalar_value.rs</FILE> - <DESC>ScalarLit → tui_vfx_core::ScalarValue token form</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2026-04-28</VERS>
