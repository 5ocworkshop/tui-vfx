// <FILE>tui-vfx-content/tests/test_typewriter_cursor.rs</FILE> - <DESC>Tests for TypewriterCursor convenience constructors</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>feat/cursor-primitive: compose Cursor via #[serde(flatten)]</WCTX>
// <CLOG>Migrate field access to .cursor.*; add backcompat JSON + legacy ctor regression tests</CLOG>

use unicode_segmentation::UnicodeSegmentation;

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::types::TypewriterCursor;

fn assert_default_shape_except_character(cursor: &TypewriterCursor) {
    let default = TypewriterCursor::default();
    assert_eq!(
        cursor.cursor.blink.interval_ms,
        default.cursor.blink.interval_ms
    );
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
    assert_eq!(cursor.cursor.character, "X");
    assert_default_shape_except_character(&cursor);
}

#[test]
fn block_returns_full_block_glyph() {
    let cursor = TypewriterCursor::block();
    assert_eq!(cursor.cursor.character, "█");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.cursor.character);
}

#[test]
fn underscore_returns_underscore_glyph() {
    let cursor = TypewriterCursor::underscore();
    assert_eq!(cursor.cursor.character, "_");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.cursor.character);
}

#[test]
fn pipe_returns_pipe_glyph() {
    let cursor = TypewriterCursor::pipe();
    assert_eq!(cursor.cursor.character, "|");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.cursor.character);
}

#[test]
fn caret_returns_left_half_block_glyph() {
    let cursor = TypewriterCursor::caret();
    assert_eq!(cursor.cursor.character, "▌");
    assert_default_shape_except_character(&cursor);
    assert_single_grapheme(&cursor.cursor.character);
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
    assert_eq!(TypewriterCursor::simple('█'), TypewriterCursor::default());
}

#[test]
fn backcompat_parses_v1_1_0_json() {
    // Frozen JSON representing a TypewriterCursor written by the v1.1.0 code.
    let json = r#"{
        "character": "█",
        "blink_interval": 500,
        "show_while_typing": 1.0,
        "show_after_complete": 1.0
    }"#;
    let parsed: TypewriterCursor = serde_json::from_str(json).unwrap();
    assert_eq!(parsed.cursor.character, "█");
    // The blink_interval alias lifts into cursor.blink.interval_ms:
    match parsed.cursor.blink.interval_ms {
        SignalOrFloat::Static(v) => assert_eq!(v, 500.0),
        _ => panic!(),
    }
    match parsed.show_while_typing {
        SignalOrFloat::Static(v) => assert_eq!(v, 1.0),
        _ => panic!(),
    }
    match parsed.show_after_complete {
        SignalOrFloat::Static(v) => assert_eq!(v, 1.0),
        _ => panic!(),
    }
    // All new fields default to no-ops.
    assert_eq!(
        parsed.cursor.grow_in,
        tui_vfx_content::cursor::GrowIn::default()
    );
    assert_eq!(parsed.cursor.wake, tui_vfx_content::cursor::Wake::default());
}

#[test]
fn legacy_convenience_constructors_still_work() {
    let b = TypewriterCursor::block();
    assert_eq!(b.cursor.character, "█");
    // Default show semantics unchanged.
    assert!(matches!(b.show_while_typing, SignalOrFloat::Static(1.0)));
    assert!(matches!(b.show_after_complete, SignalOrFloat::Static(1.0)));
    assert!(matches!(
        b.cursor.blink.interval_ms,
        SignalOrFloat::Static(500.0)
    ));
}

// <FILE>tui-vfx-content/tests/test_typewriter_cursor.rs</FILE> - <DESC>Tests for TypewriterCursor convenience constructors</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
