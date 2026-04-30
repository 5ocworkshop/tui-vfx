// <FILE>tui-vfx-core-macros/src/fnc_parse_scalar_lit.rs</FILE> - <DESC>Parse a `syn::Expr` (literal or unary-negation-of-literal) into a ScalarLit. Used by parse_config_attrs when extracting `#[config(default = X / min = Y / max = Z)]` arg expressions.</DESC>
// <VERS>VERSION: 0.2.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — promote from abandoned-refactor stub to live module. Body matches lib.rs:93-114 verbatim.</WCTX>
// <CLOG>0.2.0: MAJOR — promote from abandoned-refactor stub to live module. Body unchanged. Reachable from lib.rs in US-013.</CLOG>

use syn::{Expr, Lit, spanned::Spanned};

use crate::col_clean_number::clean_number;
use crate::fnc_scalar_lit_from_lit::scalar_lit_from_lit;
use crate::types::ScalarLit;

pub(crate) fn parse_scalar_lit(expr: &Expr) -> syn::Result<ScalarLit> {
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

// <FILE>tui-vfx-core-macros/src/fnc_parse_scalar_lit.rs</FILE> - <DESC>Parse syn::Expr → ScalarLit</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2026-04-28</VERS>
