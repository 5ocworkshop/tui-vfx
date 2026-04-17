// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: position + history push</WCTX>
// <CLOG>Initial impl — position update + history push (Tasks 1–11 scope)</CLOG>

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
/// This implementation (Tasks 1–11) handles steps 1 and 4. Steps 2, 3, 5 are
/// added in subsequent tasks.
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

    state.position = new_position;
}

// <FILE>tui-vfx-content/src/cursor/fnc_advance_cursor.rs</FILE> - <DESC>Advance cursor state one frame</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
