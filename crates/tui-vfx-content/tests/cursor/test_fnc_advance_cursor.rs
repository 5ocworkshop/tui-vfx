// <FILE>tui-vfx-content/tests/cursor/test_fnc_advance_cursor.rs</FILE> - <DESC>Tests for fnc_advance_cursor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: fnc_advance_cursor tests</WCTX>
// <CLOG>Initial tests (E1, E2, E3 position/history)</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::cursor::{fnc_advance_cursor, Cursor, CursorState};

fn ctx() -> SignalContext { SignalContext::new(0, 0) }

#[test]
fn first_advance_sets_position_and_no_history() {
    let mut state = CursorState::new();
    let cursor = Cursor::default();
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.016, &ctx());
    assert_eq!(state.position, Some((0, 0)));
    assert!(state.history.is_empty());
}

#[test]
fn e2_stationary_cursor_does_not_grow_history() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(10.0, 0); // Tint on, no cap
    for i in 0..10 {
        fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), i as f64 * 0.016, 0.016, &ctx());
    }
    assert!(state.history.is_empty());
}

#[test]
fn e1_teleport_records_destination_only() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(10.0, 0);
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((10, 5)), 0.1, 0.1, &ctx());
    assert_eq!(state.position, Some((10, 5)));
    // Only the old position (0,0) should be in history — not the ten cells between.
    assert_eq!(state.history.len(), 1);
    assert_eq!(state.history[0].0, 0);
    assert_eq!(state.history[0].1, 0);
}

#[test]
fn e3_revisiting_cell_replaces_old_entry() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(10.0, 0);
    // 0,0 → 0,1 → 0,2 → 0,1 (revisit) → 0,0 (revisit)
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.1, 0.1, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 2)), 0.2, 0.1, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.3, 0.1, &ctx()); // revisit (0,1) — removes old (0,1) entry, inserts fresh
    // History should not contain (0,1) twice.
    let hits = state.history.iter().filter(|e| (e.0, e.1) == (0, 1)).count();
    assert!(hits <= 1);
    // Most recent entry should be the cell we just left.
    assert_eq!(state.history.back().unwrap().0, 0);
    assert_eq!(state.history.back().unwrap().1, 2);
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_advance_cursor.rs</FILE> - <DESC>Tests for fnc_advance_cursor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
