// <FILE>tui-vfx-core-macros/src/lib.rs</FILE> - <DESC>Crate root for the ConfigSchema proc-macro derive. Thin re-export hub: declares the OFPF-prefixed sibling modules and re-exports the proc-macro entry. All implementation logic lives in the per-function fnc_* / col_* siblings; types live in types.rs.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-013 — restore OFPF discipline. Previously this file carried 659 lines of inline logic plus 9 abandoned-refactor sibling stubs that never compiled (no `mod` declarations). Now it is a 30-line re-export hub matching every other lib.rs in the workspace.</WCTX>
// <CLOG>1.0.0: MAJOR — replace 659-line inline body with `mod` declarations for 14 OFPF-prefixed siblings (col_clean_number, col_is_option_type, col_to_snake_case, fnc_apply_rename_all, fnc_derive_enum_schema, fnc_derive_struct_schema, fnc_extract_doc_comments, fnc_field_meta_tokens, fnc_impl_config_schema, fnc_parse_config_attrs, fnc_parse_scalar_lit, fnc_parse_serde_attrs, fnc_scalar_lit_from_lit, fnc_scalar_lit_to_scalar_value) plus types. The proc-macro entry `derive_config_schema` stays in lib.rs (proc-macro-derive entries must be in the proc-macro crate's root) but delegates to fnc_impl_config_schema::impl_config_schema. 0.4.2: collapse nested if-let chains in extract_doc_comments and is_option_type to satisfy clippy::collapsible_if under -D warnings.</CLOG>

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod col_clean_number;
mod col_is_option_type;
mod col_to_snake_case;
mod fnc_apply_rename_all;
mod fnc_derive_enum_schema;
mod fnc_derive_struct_schema;
mod fnc_extract_doc_comments;
mod fnc_field_meta_tokens;
mod fnc_impl_config_schema;
mod fnc_parse_config_attrs;
mod fnc_parse_scalar_lit;
mod fnc_parse_serde_attrs;
mod fnc_scalar_lit_from_lit;
mod fnc_scalar_lit_to_scalar_value;
mod types;

use fnc_impl_config_schema::impl_config_schema;

#[proc_macro_derive(ConfigSchema, attributes(config))]
pub fn derive_config_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_config_schema(&input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

// <FILE>tui-vfx-core-macros/src/lib.rs</FILE> - <DESC>Crate root for the ConfigSchema proc-macro derive — thin re-export hub</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
