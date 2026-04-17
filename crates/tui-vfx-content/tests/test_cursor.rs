// <FILE>tui-vfx-content/tests/test_cursor.rs</FILE> - <DESC>Aggregator for cursor module tests</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>feat/cursor-primitive T28: register test_fnc_build_cursor_shader aggregator entry</WCTX>
// <CLOG>Add T28 test module (fnc_build_cursor_shader)</CLOG>

#[path = "cursor/test_cls_cursor_blink.rs"]
mod test_cls_cursor_blink;

#[path = "cursor/test_cls_cursor_grow_in.rs"]
mod test_cls_cursor_grow_in;

#[path = "cursor/test_cls_cursor_wake.rs"]
mod test_cls_cursor_wake;

#[path = "cursor/test_cls_cursor.rs"]
mod test_cls_cursor;

#[path = "cursor/test_cls_cursor_state.rs"]
mod test_cls_cursor_state;

#[path = "cursor/test_cls_cursor_paint_ops.rs"]
mod test_cls_cursor_paint_ops;

#[path = "cursor/test_fnc_cursor_grow_in_glyph.rs"]
mod test_fnc_cursor_grow_in_glyph;

#[path = "cursor/test_fnc_advance_cursor.rs"]
mod test_fnc_advance_cursor;

#[path = "cursor/test_fnc_render_cursor.rs"]
mod test_fnc_render_cursor;

#[path = "cursor/test_fnc_typewriter_cursor_position.rs"]
mod test_fnc_typewriter_cursor_position;

#[path = "cursor/test_fnc_splice_cursor_into_text.rs"]
mod test_fnc_splice_cursor_into_text;

#[path = "cursor/test_fnc_apply_ghost_glyphs_to_grid.rs"]
mod test_fnc_apply_ghost_glyphs_to_grid;

#[path = "cursor/test_fnc_build_cursor_shader.rs"]
mod test_fnc_build_cursor_shader;

// <FILE>tui-vfx-content/tests/test_cursor.rs</FILE> - <DESC>Aggregator for cursor module tests</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
