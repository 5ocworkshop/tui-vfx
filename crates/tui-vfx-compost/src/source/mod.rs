// <FILE>crates/tui-vfx-compost/src/source/mod.rs</FILE> - <DESC>Native v3.1 source materialization</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Source module dispatches load-validated canonical source descriptors into element-local grids.</WCTX>
// <CLOG>0.2.1: PATCH — keep source module exports limited to the descriptor dispatch seam.
// 0.2.0: MINOR — add source descriptor materialization seam.
// 0.1.0: INIT — add source grid materialization helpers.</CLOG>

mod col_literal_source_input;
mod fnc_materialize_source;
mod fnc_source_grid_from_inputs;

pub(crate) use fnc_materialize_source::materialize_source;

// <FILE>crates/tui-vfx-compost/src/source/mod.rs</FILE> - <DESC>Native v3.1 source materialization</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
