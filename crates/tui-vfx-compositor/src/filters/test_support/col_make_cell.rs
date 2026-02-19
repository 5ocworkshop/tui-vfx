// <FILE>tui-vfx-compositor/src/filters/test_support/col_make_cell.rs</FILE> - <DESC>Create a default Cell for filter tests</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Add shared make_cell helper for filter tests</CLOG>

use tui_vfx_types::{Cell, Color, Modifiers};

pub(crate) fn make_cell() -> Cell {
    Cell::styled(' ', Color::WHITE, Color::BLACK, Modifiers::NONE)
}

// <FILE>tui-vfx-compositor/src/filters/test_support/col_make_cell.rs</FILE> - <DESC>Create a default Cell for filter tests</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
