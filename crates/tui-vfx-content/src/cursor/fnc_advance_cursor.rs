// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-primitive: wake aging + max_cells cap</WCTX>
// <CLOG>T14: age out history entries older than decay_seconds; cap at max_cells</CLOG>

use super::{Cursor, CursorState, WakeMode};
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
///
/// This implementation (T14) handles steps 1–4. Step 5 is added in T15.
pub fn fnc_advance_cursor(
    state: &mut CursorState,
    cursor: &Cursor,
    new_position: Option<(u16, u16)>,
    now: f64,
    _dt: f64,
    _ctx: &SignalContext,
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
            .evaluate(now, _ctx)
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
}

// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
