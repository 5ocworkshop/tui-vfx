// <FILE>tui-vfx-content/tests/cursor/test_fnc_splice_cursor_into_text.rs</FILE> - <DESC>Tests for splicing cursor glyph into revealed text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: splice helper tests</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::cursor::fnc_splice_cursor_into_text;

#[test]
fn splice_at_end_appends_glyph() {
    let out = fnc_splice_cursor_into_text("abc", 3, "█");
    assert_eq!(out, "abc█");
}

#[test]
fn splice_at_zero_prepends_glyph() {
    let out = fnc_splice_cursor_into_text("abc", 0, "█");
    assert_eq!(out, "█bc");
}

#[test]
fn splice_in_middle_replaces_grapheme_with_glyph() {
    // Cursor "overwrites" the grapheme at position 1 (character 'b').
    let out = fnc_splice_cursor_into_text("abc", 1, "█");
    assert_eq!(out, "a█c");
}

#[test]
fn empty_glyph_returns_input_unchanged() {
    let out = fnc_splice_cursor_into_text("abc", 1, "");
    assert_eq!(out, "abc");
}

#[test]
fn out_of_bounds_index_appends_at_end() {
    let out = fnc_splice_cursor_into_text("abc", 99, "█");
    assert_eq!(out, "abc█");
}

#[test]
fn partial_block_glyph() {
    let out = fnc_splice_cursor_into_text("abc", 3, "▃");
    assert_eq!(out, "abc▃");
}
// <FILE>tui-vfx-content/tests/cursor/test_fnc_splice_cursor_into_text.rs</FILE> - <DESC>Tests for splicing cursor glyph into revealed text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
