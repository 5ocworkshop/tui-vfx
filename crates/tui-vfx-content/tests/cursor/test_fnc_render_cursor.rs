// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: render tests</WCTX>
// <CLOG>Initial tests — primary op (T16)</CLOG>

use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use tui_vfx_content::cursor::{
    fnc_advance_cursor, fnc_render_cursor, Cursor, CursorState, GrowInMode,
};

fn ctx() -> SignalContext { SignalContext::new(0, 0) }

#[test]
fn static_cursor_renders_full_glyph_and_alpha_1() {
    let mut state = CursorState::new();
    let cursor = Cursor::default();
    fnc_advance_cursor(&mut state, &cursor, Some((2, 5)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    let p = ops.primary.expect("expected primary op");
    assert_eq!(p.position, (2, 5));
    assert_eq!(p.glyph, "█");
    assert!((p.alpha - 1.0).abs() < 1e-6);
    assert!(ops.trail.is_empty());
}

#[test]
fn grow_in_midway_renders_partial_block() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::default();
    cursor.grow_in.mode = GrowInMode::Once;
    cursor.grow_in.duration_ms = SignalOrFloat::Static(200.0);
    cursor.visibility = SignalOrFloat::Static(1.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.1, 0.1, &ctx()); // ~50% through
    let ops = fnc_render_cursor(&state, &cursor, 0.1, &ctx());
    let p = ops.primary.unwrap();
    assert_ne!(p.glyph, "█");  // should be a partial glyph
    assert!(!p.glyph.is_empty());
    assert!(p.alpha > 0.0 && p.alpha < 1.0);
}

#[test]
fn hidden_state_returns_no_primary_op() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::default();
    cursor.visibility = SignalOrFloat::Static(0.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert!(ops.primary.is_none());
}

#[test]
fn empty_character_returns_no_primary_op() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::default();
    cursor.character = "".into();
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert!(ops.primary.is_none());
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
