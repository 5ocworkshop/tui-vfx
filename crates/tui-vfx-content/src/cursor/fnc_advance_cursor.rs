// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>feat/cursor-primitive: grow-in phase state machine</WCTX>
// <CLOG>T15: grow-in phase transitions (Hidden/GrowingIn/Visible/GrowingOut)</CLOG>

use super::{Cursor, CursorState, GrowInMode, GrowInPhase, WakeMode};
use mixed_signals::prelude::SignalContext;

/// Advance cursor state by one frame.
///
/// Steps (see spec §4.4):
/// 1. If position changes, push the old position + `now` onto the history ring.
///    On revisit of a cell already in history, the older entry is removed first.
/// 2. Age out entries older than `wake.decay_seconds`.
/// 3. Cap history length at `wake.max_cells` (when > 0).
/// 4. Update position.
/// 5. Recompute grow-in phase against effective visibility.
pub fn fnc_advance_cursor(
    state: &mut CursorState,
    cursor: &Cursor,
    new_position: Option<(u16, u16)>,
    now: f64,
    dt: f64,
    ctx: &SignalContext,
) {
    let wake_enabled = !matches!(cursor.wake.mode, WakeMode::Off);

    if let (Some(old), Some(new)) = (state.position, new_position) {
        if old != new && wake_enabled {
            // E3: remove stale entry for old position (prevents double entries on revisit).
            state.history.retain(|e| (e.0, e.1) != old);
            state.history.push_back((old.0, old.1, now));
        }
    }

    // Age out entries older than decay_seconds.
    if wake_enabled {
        let decay = cursor
            .wake
            .decay_seconds
            .evaluate(now, ctx)
            .unwrap_or(0.0)
            .max(0.0) as f64;
        if decay > 0.0 {
            state.history.retain(|e| now - e.2 <= decay);
        } else {
            // decay_seconds = 0 is canonical "disable wake" (spec E11).
            state.history.clear();
        }
        // Cap history.
        let cap = cursor.wake.max_cells as usize;
        if cap > 0 {
            while state.history.len() > cap {
                state.history.pop_front();
            }
        }
    } else {
        // Wake mode switched to Off — drain any lingering trail.
        state.history.clear();
    }

    state.position = new_position;

    // --- Grow-in phase state machine ---
    let duration_ms = cursor
        .grow_in
        .duration_ms
        .evaluate(now, ctx)
        .unwrap_or(0.0)
        .max(0.0) as f64;
    let grow_out_ms = cursor
        .grow_in
        .grow_out_ms
        .evaluate(now, ctx)
        .unwrap_or(0.0)
        .max(0.0) as f64;
    let vis_base = cursor.visibility.evaluate(now, ctx).unwrap_or(0.0);
    let vis = if vis_base.is_finite() { vis_base.clamp(0.0, 1.0) } else { 0.0 };

    let dt_ms = (dt * 1000.0).max(0.0);
    let prev_vis = state.last_effective_visibility;
    state.last_effective_visibility = vis;

    let show_transition = prev_vis <= 0.0 && vis > 0.0;
    let hide_transition = prev_vis > 0.0 && vis <= 0.0;

    let grow_in_active = matches!(cursor.grow_in.mode, GrowInMode::Once | GrowInMode::EveryShow)
        && duration_ms > 0.0;
    let grow_out_active = grow_out_ms > 0.0;

    state.grow_in_phase = match state.grow_in_phase {
        GrowInPhase::Hidden => {
            if show_transition && grow_in_active {
                let once_suppressed = matches!(cursor.grow_in.mode, GrowInMode::Once)
                    && state.grow_in_has_fired_once;
                if once_suppressed {
                    GrowInPhase::Visible
                } else {
                    state.grow_in_has_fired_once = true;
                    GrowInPhase::GrowingIn { elapsed_ms: 0.0 }
                }
            } else if vis > 0.0 {
                GrowInPhase::Visible
            } else {
                GrowInPhase::Hidden
            }
        }
        GrowInPhase::GrowingIn { elapsed_ms } => {
            let next = elapsed_ms + dt_ms;
            if hide_transition {
                if grow_out_active {
                    GrowInPhase::GrowingOut { elapsed_ms: 0.0 }
                } else {
                    GrowInPhase::Hidden
                }
            } else if next >= duration_ms {
                GrowInPhase::Visible
            } else {
                GrowInPhase::GrowingIn { elapsed_ms: next }
            }
        }
        GrowInPhase::Visible => {
            if hide_transition {
                if grow_out_active {
                    GrowInPhase::GrowingOut { elapsed_ms: 0.0 }
                } else {
                    GrowInPhase::Hidden
                }
            } else {
                GrowInPhase::Visible
            }
        }
        GrowInPhase::GrowingOut { elapsed_ms } => {
            let next = elapsed_ms + dt_ms;
            if show_transition {
                // Re-show during grow-out: jump back into grow-in if active, else Visible.
                if grow_in_active {
                    let once_suppressed = matches!(cursor.grow_in.mode, GrowInMode::Once)
                        && state.grow_in_has_fired_once;
                    if once_suppressed {
                        GrowInPhase::Visible
                    } else {
                        state.grow_in_has_fired_once = true;
                        GrowInPhase::GrowingIn { elapsed_ms: 0.0 }
                    }
                } else {
                    GrowInPhase::Visible
                }
            } else if next >= grow_out_ms {
                GrowInPhase::Hidden
            } else {
                GrowInPhase::GrowingOut { elapsed_ms: next }
            }
        }
    };
}

// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
