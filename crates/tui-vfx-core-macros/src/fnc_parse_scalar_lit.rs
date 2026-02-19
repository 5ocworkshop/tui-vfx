// <FILE>tui-vfx-core-macros/src/fnc_parse_scalar_lit.rs</FILE> - <DESC>Parse syn::Expr into ScalarLit</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted scalar literal expression parsing</CLOG>

use syn::{spanned::Spanned, Expr, Lit};

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

// <FILE>tui-vfx-core-macros/src/fnc_parse_scalar_lit.rs</FILE> - <DESC>Parse syn::Expr into ScalarLit</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

