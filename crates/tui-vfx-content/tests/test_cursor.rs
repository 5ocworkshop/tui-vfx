// <FILE>tui-vfx-content/tests/test_cursor.rs</FILE> - <DESC>Aggregator for cursor module tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-primitive: test aggregator</WCTX>
// <CLOG>Add T12 + T13 test modules (backfill + advance cursor)</CLOG>

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

// <FILE>tui-vfx-content/tests/test_cursor.rs</FILE> - <DESC>Aggregator for cursor module tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
