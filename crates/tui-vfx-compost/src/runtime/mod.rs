// <FILE>crates/tui-vfx-compost/src/runtime/mod.rs</FILE> - <DESC>Native runtime value resolution modules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime value resolution centralizes canonical ValueSource handling for validation and future render-time bindings.</WCTX>
// <CLOG>0.1.0: INIT — add runtime context, resolved value, and value-source resolver.</CLOG>

mod cls_resolved_value;
mod cls_runtime_context;
mod cls_runtime_value_error;
mod fnc_resolve_value_source;

pub(crate) use cls_resolved_value::ResolvedValue;
pub(crate) use cls_runtime_context::RuntimeContext;
pub(crate) use cls_runtime_value_error::RuntimeValueError;
pub(crate) use fnc_resolve_value_source::resolve_value_source;

// <FILE>crates/tui-vfx-compost/src/runtime/mod.rs</FILE> - <DESC>Native runtime value resolution modules</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
