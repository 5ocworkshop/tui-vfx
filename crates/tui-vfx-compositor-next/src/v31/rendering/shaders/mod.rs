// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/mod.rs</FILE> - <DESC>Direct v3.1 shader render modules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Each signed shader primitive owns a small builder file; dispatch remains additive and reviewable.</WCTX>
// <CLOG>0.1.0: INIT — add per-shader direct render module tree.</CLOG>

mod col_shader_value_input;
mod fnc_border_sweep_shader;
mod fnc_focus_field_shader;
mod fnc_glisten_band_shader;
mod fnc_highlighter_shader;
mod fnc_linear_gradient_shader;
mod orc_append_shader_node_to_composition;

pub(crate) use orc_append_shader_node_to_composition::append_shader_node_to_composition;

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/mod.rs</FILE> - <DESC>Direct v3.1 shader render modules</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
