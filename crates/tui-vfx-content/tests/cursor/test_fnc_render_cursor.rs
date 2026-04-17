// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up (field_reassign_with_default)</WCTX>
// <CLOG>PATCH: rewrite hidden/empty-char cursor constructions to struct-literal form</CLOG>

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
    let cursor = Cursor {
        visibility: SignalOrFloat::Static(0.0),
        ..Cursor::default()
    };
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert!(ops.primary.is_none());
}

#[test]
fn empty_character_returns_no_primary_op() {
    let mut state = CursorState::new();
    let cursor = Cursor {
        character: "".into(),
        ..Cursor::default()
    };
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert!(ops.primary.is_none());
}

// --- T17: wake Tint trail painting ---

#[test]
fn tint_wake_emits_trail_ops_with_decaying_alpha() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(1.0, 0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 2)), 0.5, 0.4, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
    assert_eq!(ops.trail.len(), 2);
    // All trail ops are glyph=None in Tint mode.
    for op in &ops.trail {
        assert!(op.glyph.is_none());
    }
    // Oldest entry should have lower alpha than newest.
    let oldest_alpha = ops.trail[0].alpha;
    let newest_alpha = ops.trail[1].alpha;
    assert!(oldest_alpha < newest_alpha);
    for op in &ops.trail {
        assert!((0.0..=1.0).contains(&op.alpha));
    }
}

#[test]
fn e7_trail_decays_while_cursor_hidden() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::default().with_wake_tint(1.0, 0);
    cursor.visibility = SignalOrFloat::Static(1.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    cursor.visibility = SignalOrFloat::Static(0.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.5, 0.4, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
    assert!(ops.primary.is_none()); // cursor hidden
    assert_eq!(ops.trail.len(), 1);
    assert!(ops.trail[0].alpha > 0.0 && ops.trail[0].alpha < 1.0);
}

#[test]
fn e11_wake_off_emits_no_trail_ops() {
    let mut state = CursorState::new();
    let cursor = Cursor::default(); // WakeMode::Off
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.1, &ctx());
    assert!(ops.trail.is_empty());
}

// --- T18: wake Ghost trail painting ---

#[test]
fn ghost_wake_emits_trail_ops_with_cursor_character() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_ghost(1.0, 0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.1, &ctx());
    assert_eq!(ops.trail.len(), 1);
    assert_eq!(ops.trail[0].glyph.as_deref(), Some("█"));
}

// --- T19: edge-case tests (E10) ---

#[test]
fn e10_empty_character_with_wake_still_decays_trail() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::default().with_wake_tint(1.0, 0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    cursor.character = "".into();
    let ops = fnc_render_cursor(&state, &cursor, 0.1, &ctx());
    assert!(ops.primary.is_none());       // no primary
    assert_eq!(ops.trail.len(), 1);       // existing trail persists
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
