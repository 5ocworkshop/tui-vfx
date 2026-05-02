// <FILE>crates/tui-vfx-compost/src/runtime/mod.rs</FILE> - <DESC>Native runtime value resolution modules</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Runtime value resolution centralizes canonical ValueSource handling for validation and render-time bindings.</WCTX>
// <CLOG>0.2.0: MINOR — split specialized value-source helpers into OFPF-sized modules.</CLOG>

mod cls_resolved_value;
mod cls_runtime_context;
mod cls_runtime_value_error;
mod fnc_evaluate_signal_expression;
mod fnc_resolve_clock_value_source;
mod fnc_resolve_mapped_value_source;
mod fnc_resolve_numeric_value_source;
mod fnc_resolve_sampled_field_source;
mod fnc_resolve_value_source;

pub(crate) use cls_resolved_value::ResolvedValue;
pub(crate) use cls_runtime_context::RuntimeContext;
pub(crate) use cls_runtime_value_error::RuntimeValueError;
pub(crate) use fnc_evaluate_signal_expression::evaluate_signal_expression;
pub(crate) use fnc_resolve_clock_value_source::resolve_clock_value_source;
pub(crate) use fnc_resolve_mapped_value_source::resolve_mapped_value_source;
pub(crate) use fnc_resolve_numeric_value_source::resolve_numeric_value_source;
pub(crate) use fnc_resolve_sampled_field_source::resolve_sampled_field_source;
pub(crate) use fnc_resolve_value_source::resolve_value_source;

// <FILE>crates/tui-vfx-compost/src/runtime/mod.rs</FILE> - <DESC>Native runtime value resolution modules</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
