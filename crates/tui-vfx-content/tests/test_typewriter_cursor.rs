// <FILE>tui-vfx-content/tests/test_typewriter_cursor.rs</FILE> - <DESC>Tests for TypewriterCursor convenience constructors</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>feat/content-ergonomics: TypewriterCursor convenience constructors</WCTX>
// <CLOG>Initial test file covering simple/block/underscore/pipe/caret presets</CLOG>

use unicode_segmentation::UnicodeSegmentation;

use tui_vfx_content::types::TypewriterCursor;

fn assert_default_shape_except_character(cursor: &TypewriterCursor) {
    let default = TypewriterCursor::default();
    assert_eq!(cursor.blink_interval, default.blink_interval);
    assert_eq!(cursor.show_while_typing, default.show_while_typing);
    assert_eq!(cursor.show_after_complete, default.show_after_complete);
}

fn assert_single_grapheme(s: &str) {
    let count = s.graphemes(true).count();
    assert_eq!(
        count, 1,
        "expected exactly one grapheme cluster, got {count} for {s:?}"
    );
}

#[test]
fn simple_sets_glyph_and_keeps_default_behavior() {
    let cursor = TypewriterCursor::simple('X');
    assert_eq!(cursor.character, "X");
    assert_default_shape_except_character(&cursor);
}

#[test]
fn block_returns_full_block_glyph() {
    let cursor = TypewriterCursor::block();
    assert_eq!(cursor.character, "█");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.character);
}

#[test]
fn underscore_returns_underscore_glyph() {
    let cursor = TypewriterCursor::underscore();
    assert_eq!(cursor.character, "_");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.character);
}

#[test]
fn pipe_returns_pipe_glyph() {
    let cursor = TypewriterCursor::pipe();
    assert_eq!(cursor.character, "|");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.character);
}

#[test]
fn caret_returns_left_half_block_glyph() {
    let cursor = TypewriterCursor::caret();
    assert_eq!(cursor.character, "▌");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.character);
}

#[test]
fn block_matches_default_shape() {
    // The default cursor is a block. block() must produce the same shape
    // as Default::default() for backward-compatibility expectations.
    assert_eq!(TypewriterCursor::block(), TypewriterCursor::default());
}

#[test]
fn simple_with_default_glyph_matches_default() {
    // simple('█') is equivalent to default(), aside from being the
    // explicit form. This pins the contract that simple() never alters
    // the non-character fields.
    assert_eq!(
        TypewriterCursor::simple('█'),
        TypewriterCursor::default()
    );
}

// <FILE>tui-vfx-content/tests/test_typewriter_cursor.rs</FILE> - <DESC>Tests for TypewriterCursor convenience constructors</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
