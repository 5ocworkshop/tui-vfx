// <FILE>tui-vfx-core-macros/src/fnc_parse_config_attrs.rs</FILE> - <DESC>Parse #[config(...)] attributes into ConfigAttr</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted attribute parser</CLOG>

use syn::{Attribute, Expr};

use crate::fnc_parse_scalar_lit::parse_scalar_lit;
use crate::types::ConfigAttr;

pub(crate) fn parse_config_attrs(attrs: &[Attribute]) -> syn::Result<ConfigAttr> {
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

// <FILE>tui-vfx-core-macros/src/fnc_parse_config_attrs.rs</FILE> - <DESC>Parse #[config(...)] attributes into ConfigAttr</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

