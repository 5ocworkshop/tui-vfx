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

// --- T14: wake aging + max_cells cap ---

#[test]
fn entries_age_out_after_decay_seconds() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(1.0, 0); // 1s decay, no cap
    fnc_advance_cursor(&mut state, &cursor, Some((0, 0)), 0.0, 0.0, &ctx());
    fnc_advance_cursor(&mut state, &cursor, Some((0, 1)), 0.5, 0.5, &ctx());
    // (0,0) pushed at t=0.5 (old pos); now advance to t=2.0 — (0,0) entry age is 1.5s, past decay
    fnc_advance_cursor(&mut state, &cursor, Some((0, 2)), 2.0, 1.5, &ctx());
    // At t=2.0, the (0,0) entry (seen at 0.5) is 1.5s old and should be dropped.
    // (0,1) entry was pushed at t=2.0 and remains.
    assert_eq!(state.history.len(), 1);
    assert_eq!(state.history[0].0, 0);
    assert_eq!(state.history[0].1, 1);
}

#[test]
fn max_cells_caps_history_oldest_dropped() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(100.0, 3); // long decay, cap=3
    for col in 0..10 {
        fnc_advance_cursor(&mut state, &cursor, Some((0, col)), col as f64 * 0.01, 0.01, &ctx());
    }
    assert_eq!(state.history.len(), 3);
    // Oldest three should be dropped; expect entries 6, 7, 8 (0-indexed).
    let cols: Vec<u16> = state.history.iter().map(|e| e.1).collect();
    assert_eq!(cols, vec![6, 7, 8]);
}

#[test]
fn max_cells_zero_means_no_cap() {
    let mut state = CursorState::new();
    let cursor = Cursor::default().with_wake_tint(100.0, 0);
    for col in 0..20 {
        fnc_advance_cursor(&mut state, &cursor, Some((0, col)), col as f64 * 0.01, 0.01, &ctx());
    }
    assert_eq!(state.history.len(), 20 - 1); // last position isn't in history yet
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_advance_cursor.rs</FILE> - <DESC>Tests for fnc_advance_cursor</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
