// <FILE>crates/tui-vfx-compost/src/shaders/mod.rs</FILE> - <DESC>Native shader primitive implementations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Only migrated shader slices are wired here; no legacy SpatialShaderType adapter.</WCTX>
// <CLOG>0.1.0: INIT — add linearGradient native shader module.</CLOG>

mod cls_linear_gradient_node;
mod col_shader_input;
mod fnc_linear_gradient_style;

pub(crate) use cls_linear_gradient_node::LinearGradientNode;
pub(crate) use col_shader_input::{enum_input, gradient_input, number_input};
pub(crate) use fnc_linear_gradient_style::linear_gradient_style;

// <FILE>crates/tui-vfx-compost/src/shaders/mod.rs</FILE> - <DESC>Native shader primitive implementations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
