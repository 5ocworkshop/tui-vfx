// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor.rs</FILE> - <DESC>Tests for Cursor config</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: Cursor struct tests</WCTX>
// <CLOG>Initial tests — struct, convenience ctors, builders</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::{Cursor, CursorBlink, GrowIn, GrowInMode, Wake, WakeMode};

#[test]
fn default_is_static_block_cursor() {
    let c = Cursor::default();
    assert_eq!(c.character, "█");
    assert!(matches!(c.visibility, SignalOrFloat::Static(1.0)));
    assert_eq!(c.blink, CursorBlink::default());
    assert_eq!(c.grow_in, GrowIn::default());
    assert_eq!(c.wake, Wake::default());
}

#[test]
fn serde_roundtrip() {
    let c = Cursor::default();
    let json = serde_json::to_string(&c).unwrap();
    let back: Cursor = serde_json::from_str(&json).unwrap();
    assert_eq!(c, back);
}

#[test]
fn minimal_json_deserializes_to_defaults() {
    let c: Cursor = serde_json::from_str("{}").unwrap();
    assert_eq!(c, Cursor::default());
}

#[test]
fn block_underscore_pipe_caret_have_expected_glyphs() {
    assert_eq!(Cursor::block().character, "█");
    assert_eq!(Cursor::underscore().character, "_");
    assert_eq!(Cursor::pipe().character, "|");
    assert_eq!(Cursor::caret().character, "▌");
}

#[test]
fn simple_uses_provided_glyph_and_keeps_defaults() {
    let c = Cursor::simple('◆');
    assert_eq!(c.character, "◆");
    assert_eq!(c.blink, CursorBlink::default());
    assert_eq!(c.grow_in, GrowIn::default());
    assert_eq!(c.wake, Wake::default());
}

#[test]
fn convenience_ctors_equal_default_with_swapped_char() {
    let mut expected = Cursor::default();
    expected.character = "_".to_string();
    assert_eq!(Cursor::underscore(), expected);
}

#[test]
fn with_grow_in_sets_once_mode_and_duration() {
    let c = Cursor::block().with_grow_in(200.0);
    assert_eq!(c.grow_in.mode, GrowInMode::Once);
    assert!(matches!(c.grow_in.duration_ms, SignalOrFloat::Static(200.0)));
}

#[test]
fn with_wake_tint_sets_tint_mode_decay_and_cap() {
    let c = Cursor::block().with_wake_tint(1.5, 8);
    assert_eq!(c.wake.mode, WakeMode::Tint);
    assert!(matches!(c.wake.decay_seconds, SignalOrFloat::Static(1.5)));
    assert_eq!(c.wake.max_cells, 8);
}

#[test]
fn with_wake_ghost_sets_ghost_mode_decay_and_cap() {
    let c = Cursor::block().with_wake_ghost(2.0, 6);
    assert_eq!(c.wake.mode, WakeMode::Ghost);
    assert!(matches!(c.wake.decay_seconds, SignalOrFloat::Static(2.0)));
    assert_eq!(c.wake.max_cells, 6);
}

#[test]
fn builders_preserve_other_fields() {
    let c = Cursor::caret().with_grow_in(100.0).with_wake_tint(0.5, 4);
    assert_eq!(c.character, "▌");
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor.rs</FILE> - <DESC>Tests for Cursor config</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
