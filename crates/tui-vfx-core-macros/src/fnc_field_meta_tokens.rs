// <FILE>tui-vfx-core-macros/src/fnc_field_meta_tokens.rs</FILE> - <DESC>Build the `::tui_vfx_core::FieldMeta { ... }` token-stream form for a derived field. Reads ConfigAttr (help/default/min/max), the field's `///` doc comment (description), the SerdeAttr (default → optional), and the field's syn::Type (Option<T> → optional). Emits the full FieldMeta with json_key intentionally None — caller wires json_key per field after rename_all is applied.</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — replace abandoned-refactor stub body with the live version from lib.rs:296-348. Live version takes 4 args (attr, doc, serde, ty) instead of the stub's 1 (attr); also emits description (from doc) and optional (from is_option_type / serde.default).</WCTX>
// <CLOG>0.3.0: MAJOR — replace stub body with live lib.rs version. Signature changed from `field_meta_tokens(attr)` to `field_meta_tokens(attr, doc, serde, ty)` per the live macro's expanded attribute support. Adds description and optional emission. 0.2.0: stub with simplified 1-arg signature.</CLOG>

use quote::quote;

use crate::col_is_option_type::is_option_type;
use crate::fnc_scalar_lit_to_scalar_value::scalar_lit_to_scalar_value;
use crate::types::{ConfigAttr, SerdeAttr};

pub(crate) fn field_meta_tokens(
    attr: &ConfigAttr,
    doc: Option<String>,
    serde: &SerdeAttr,
    ty: &syn::Type,
) -> proc_macro2::TokenStream {
    let help = match &attr.help {
        Some(h) => quote!(Some(#h.to_string())),
        None => quote!(None),
    };
    let description = match doc {
        Some(d) => quote!(Some(#d.to_string())),
        None => quote!(None),
    };
    let default = match &attr.default {
        Some(d) => {
            let v = scalar_lit_to_scalar_value(d);
            quote!(Some(#v))
        }
        None => quote!(None),
    };
    let range = match (&attr.min, &attr.max) {
        (None, None) => quote!(None),
        (min, max) => {
            let min_ts = match min {
                Some(m) => {
                    let v = scalar_lit_to_scalar_value(m);
                    quote!(Some(#v))
                }
                None => quote!(None),
            };
            let max_ts = match max {
                Some(m) => {
                    let v = scalar_lit_to_scalar_value(m);
                    quote!(Some(#v))
                }
                None => quote!(None),
            };
            quote!(Some(::tui_vfx_core::Range::new(#min_ts, #max_ts)))
        }
    };

    let optional = is_option_type(ty) || serde.default;

    quote!(::tui_vfx_core::FieldMeta {
        help: #help,
        description: #description,
        default: #default,
        range: #range,
        json_key: None,
        optional: #optional,
    })
}

// <FILE>tui-vfx-core-macros/src/fnc_field_meta_tokens.rs</FILE> - <DESC>Build FieldMeta token expression (live 4-arg signature)</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-28</VERS>
