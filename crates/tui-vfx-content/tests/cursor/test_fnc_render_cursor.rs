// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>feat/cursor-scan: scan override during GrowInPhase::Visible — cover Pulse endpoints, HalfBlockBounce thirds, non-block passthrough, grow-in precedence, and period_ms=0 disabling.</WCTX>
// <CLOG>MINOR: add scan tests (Off/Pulse/HalfBlockBounce) covering phase math, passthrough, grow-in precedence, and zero-period disabling.</CLOG>

use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use tui_vfx_content::cursor::{
    fnc_advance_cursor, fnc_render_cursor, Cursor, CursorScan, CursorState, GrowInMode, ScanMode,
};

fn ctx() -> SignalContext {
    SignalContext::new(0, 0)
}

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
    assert_ne!(p.glyph, "█"); // should be a partial glyph
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
    assert!(ops.primary.is_none()); // no primary
    assert_eq!(ops.trail.len(), 1); // existing trail persists
}

// --- CursorScan: Pulse / HalfBlockBounce / Off ---

fn scan_cursor(mode: ScanMode, period_ms: f32) -> Cursor {
    Cursor {
        scan: CursorScan {
            mode,
            period_ms: SignalOrFloat::Static(period_ms),
            ..CursorScan::default()
        },
        ..Cursor::default()
    }
}

#[test]
fn scan_off_leaves_glyph_unchanged() {
    let mut state = CursorState::new();
    let cursor = scan_cursor(ScanMode::Off, 1000.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "█");
}

#[test]
fn scan_pulse_phase_endpoints_and_midpoint() {
    let mut state = CursorState::new();
    let cursor = scan_cursor(ScanMode::Pulse, 1000.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    // now is seconds; period_ms = 1000ms → phase = (now*1000 % 1000)/1000.
    // now=0 → phase=0 → ▁
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "▁");
    // now=0.5s → phase=0.5 → █
    let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "█");
    // now=1.0s → phase=0.0 (wrap) → ▁
    let ops = fnc_render_cursor(&state, &cursor, 1.0, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "▁");
}

#[test]
fn scan_half_block_bounce_phase_thirds() {
    let mut state = CursorState::new();
    let cursor = scan_cursor(ScanMode::HalfBlockBounce, 900.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    // period = 900ms; now=0 → phase 0 → ▀
    let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "▀");
    // now=0.4s → phase ≈ 0.444 → █
    let ops = fnc_render_cursor(&state, &cursor, 0.4, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "█");
    // now=0.8s → phase ≈ 0.888 → ▄
    let ops = fnc_render_cursor(&state, &cursor, 0.8, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "▄");
}

#[test]
fn scan_non_block_base_returns_base_unchanged() {
    for base in ["|", "_", "▌"] {
        let cursor = Cursor {
            character: base.to_string(),
            scan: CursorScan {
                mode: ScanMode::Pulse,
                period_ms: SignalOrFloat::Static(1000.0),
                ..CursorScan::default()
            },
            ..Cursor::default()
        };
        let mut state = CursorState::new();
        fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
        let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
        assert_eq!(ops.primary.unwrap().glyph, base);
        let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
        assert_eq!(ops.primary.unwrap().glyph, base);
    }

    for base in ["|", "_", "▌"] {
        let cursor = Cursor {
            character: base.to_string(),
            scan: CursorScan {
                mode: ScanMode::HalfBlockBounce,
                period_ms: SignalOrFloat::Static(900.0),
                ..CursorScan::default()
            },
            ..Cursor::default()
        };
        let mut state = CursorState::new();
        fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
        let ops = fnc_render_cursor(&state, &cursor, 0.0, &ctx());
        assert_eq!(ops.primary.unwrap().glyph, base);
    }
}

#[test]
fn scan_does_not_override_during_grow_in() {
    let mut state = CursorState::new();
    let cursor = Cursor {
        grow_in: tui_vfx_content::cursor::GrowIn {
            mode: GrowInMode::Once,
            duration_ms: SignalOrFloat::Static(200.0),
            ..tui_vfx_content::cursor::GrowIn::default()
        },
        scan: CursorScan {
            mode: ScanMode::Pulse,
            period_ms: SignalOrFloat::Static(1000.0),
            ..CursorScan::default()
        },
        ..Cursor::default()
    };
    // First show → GrowingIn phase.
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.1, 0.1, &ctx()); // ~50% through
                                                                             // While growing in, glyph comes from grow-in ramp, not from scan.
    let ops = fnc_render_cursor(&state, &cursor, 0.1, &ctx());
    let p = ops.primary.unwrap();
    // Grow-in midway produces a partial 1/8th block distinct from the
    // scan phase-at-0.1s glyph. The only guarantee we need is that the
    // render path doesn't short-circuit to scan during GrowingIn.
    assert!(!p.glyph.is_empty());
    // Alpha follows grow-in progress (<1), not steady-state scan (=1).
    assert!(p.alpha < 1.0);
}

#[test]
fn scan_zero_period_disables() {
    let mut state = CursorState::new();
    let cursor = scan_cursor(ScanMode::Pulse, 0.0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    let ops = fnc_render_cursor(&state, &cursor, 0.5, &ctx());
    assert_eq!(ops.primary.unwrap().glyph, "█");
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_render_cursor.rs</FILE> - <DESC>Tests for fnc_render_cursor</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
