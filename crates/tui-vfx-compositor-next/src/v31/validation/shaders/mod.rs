// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/mod.rs</FILE> - <DESC>Direct v3.1 shader validation modules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Each integrated shader owns its direct-input validator in an OFPF-sized file.</WCTX>
// <CLOG>0.1.0: INIT — add per-shader validation dispatch.</CLOG>

mod fnc_validate_border_sweep_direct_inputs;
mod fnc_validate_focus_field_direct_inputs;
mod fnc_validate_glisten_band_direct_inputs;
mod fnc_validate_highlighter_direct_inputs;
mod fnc_validate_linear_gradient_direct_inputs;
mod orc_validate_shader_direct_inputs;

pub(crate) use orc_validate_shader_direct_inputs::validate_shader_direct_inputs;

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/mod.rs</FILE> - <DESC>Direct v3.1 shader validation modules</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
