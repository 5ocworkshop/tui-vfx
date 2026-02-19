// <FILE>tui-vfx-geometry/tests/test_widgets.rs</FILE>
// <DESC>Test entry point for widgets module</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Dogfooding: ensure grid mapping and hit-testing stay stable</WCTX>
// <CLOG>Added widgets test linker</CLOG>

#[path = "widgets/test_col_numpad_mapping.rs"]
mod test_col_numpad_mapping;

#[path = "widgets/test_fnc_hit_test_numpad_grid.rs"]
mod test_fnc_hit_test_numpad_grid;

#[path = "widgets/test_fnc_resolve_direction_selection_motion.rs"]
mod test_fnc_resolve_direction_selection_motion;

// <FILE>tui-vfx-geometry/tests/test_widgets.rs</FILE>
// <DESC>Test entry point for widgets module</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
