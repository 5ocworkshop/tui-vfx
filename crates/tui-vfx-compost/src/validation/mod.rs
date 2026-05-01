// <FILE>crates/tui-vfx-compost/src/validation/mod.rs</FILE> - <DESC>Native v3.1 load validation modules</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Validation accepts only native direct-render support at recipe load time.</WCTX>
// <CLOG>0.2.0: MINOR — add scene element policy validation.
// 0.1.0: INIT — add source and shader validation modules.</CLOG>

mod col_direct_input;
mod fnc_validate_scene_element_policies;
mod fnc_validate_source_inputs;
mod orc_validate_render_contract;
mod shaders;

pub(crate) use col_direct_input::{
    require_declared_inputs_literal, require_enum_value, require_literal_input,
    require_number_input,
};
pub(crate) use fnc_validate_scene_element_policies::validate_scene_element_policies;
pub(crate) use fnc_validate_source_inputs::validate_source_inputs;
pub(crate) use orc_validate_render_contract::validate_render_contract;

// <FILE>crates/tui-vfx-compost/src/validation/mod.rs</FILE> - <DESC>Native v3.1 load validation modules</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
