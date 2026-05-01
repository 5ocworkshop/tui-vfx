// <FILE>crates/tui-vfx-compost/src/source/mod.rs</FILE> - <DESC>Native v3.1 source materialization</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Source module turns load-validated canonical source inputs into cell grids.</WCTX>
// <CLOG>0.1.0: INIT — add source grid materialization helpers.</CLOG>

mod col_literal_source_input;
mod fnc_source_grid_from_inputs;

pub(crate) use col_literal_source_input::{literal_color, literal_integer, literal_text};
pub(crate) use fnc_source_grid_from_inputs::source_grid_from_inputs;

// <FILE>crates/tui-vfx-compost/src/source/mod.rs</FILE> - <DESC>Native v3.1 source materialization</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
