// <FILE>tui-vfx-core-macros/src/fnc_field_meta_tokens.rs</FILE> - <DESC>Build tui_vfx_core::FieldMeta token expression</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Add None defaults for new FieldMeta fields (backward compatibility only)</CLOG>

use quote::quote;

use crate::fnc_scalar_lit_to_scalar_value::scalar_lit_to_scalar_value;
use crate::types::ConfigAttr;

pub(crate) fn field_meta_tokens(attr: &ConfigAttr) -> proc_macro2::TokenStream {
    let help = match &attr.help {
        Some(h) => quote!(Some(#h.to_string())),
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
    quote!(::tui_vfx_core::FieldMeta {
        help: #help,
        description: None,
        default: #default,
        range: #range,
        json_key: None,
        optional: false,
    })
}

// <FILE>tui-vfx-core-macros/src/fnc_field_meta_tokens.rs</FILE> - <DESC>Build tui_vfx_core::FieldMeta token expression</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>

