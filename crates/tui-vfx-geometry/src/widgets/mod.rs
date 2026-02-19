// <FILE>tui-vfx-geometry/src/widgets/mod.rs</FILE>
// <DESC>Shared TUI widgets for geometry primitives (anchor/direction grids)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Dogfooding: share numpad-style 3x3 grid rendering and hit-testing</WCTX>
// <CLOG>Added numpad grid primitives</CLOG>

pub mod col_numpad_mapping;
pub mod fnc_hit_test_numpad_3x3;
pub mod fnc_hit_test_triplet_grids;
pub mod fnc_resolve_direction_selection_motion;
pub mod types;

// NOTE: Rendering widgets (render_direction_grid_lines, render_dwell_grid_lines)
// have been moved to mixed-ratatui (L3 adapter) as they depend on ratatui types.
// See mixed-ratatui/src/widgets/ for the ratatui-specific implementations.

pub use col_numpad_mapping::{
    anchor_from_numpad_digit, direction_from_numpad_digit, numpad_digit_from_anchor,
};
pub use fnc_hit_test_numpad_3x3::hit_test_numpad_3x3;
pub use fnc_hit_test_triplet_grids::{TripletGridPart, hit_test_triplet_grids};
pub use fnc_resolve_direction_selection_motion::resolve_direction_selection_motion;
pub use types::{
    ArrowOrientation, DirectionNumpadSelection, DirectionSelectionMotion, ExitDirectionSelection,
    TripletGridFocus,
};

// <FILE>tui-vfx-geometry/src/widgets/mod.rs</FILE>
// <DESC>Shared TUI widgets for geometry primitives (anchor/direction grids)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
