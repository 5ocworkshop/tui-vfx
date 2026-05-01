// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/mod.rs</FILE> - <DESC>Direct v3.1 load-validation modules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Mirror compositor OFPF module layout: orchestration, source validation, shader validation, and leaf input helpers are separate.</WCTX>
// <CLOG>0.1.0: INIT — add OFPF-shaped validation module tree.</CLOG>

mod col_direct_input;
mod fnc_validate_direct_source_inputs;
mod orc_validate_direct_render_contract;
mod shaders;

pub(crate) use orc_validate_direct_render_contract::validate_direct_render_contract;

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/mod.rs</FILE> - <DESC>Direct v3.1 load-validation modules</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
