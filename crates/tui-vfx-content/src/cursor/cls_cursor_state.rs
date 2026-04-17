// <FILE>tui-vfx-content/src/cursor/cls_cursor_state.rs</FILE> - <DESC>Runtime state for Cursor primitive</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up — derive Default on GrowInPhase using #[default] attribute</WCTX>
// <CLOG>PATCH: derive Default on GrowInPhase (Hidden); remove manual impl</CLOG>

use std::collections::VecDeque;

/// Phase of the grow-in animation state machine (see spec §4.1).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum GrowInPhase {
    #[default]
    Hidden,
    GrowingIn {
        elapsed_ms: f64,
    },
    Visible,
    GrowingOut {
        elapsed_ms: f64,
    },
}

/// Per-cursor runtime state. Callers own one of these per [`crate::cursor::Cursor`]
/// and pass it to [`crate::cursor::fnc_advance_cursor()`] each frame.
///
/// The state has no clock — time comes in via `now`/`dt` on each advance.
#[derive(Debug, Clone, Default)]
pub struct CursorState {
    /// Current cursor position in grid coordinates `(row, col)`.
    pub position: Option<(u16, u16)>,
    /// Ring of trail entries — `(row, col, first_seen_wall_clock_seconds)`.
    /// Managed by `fnc_advance_cursor`.
    pub history: VecDeque<(u16, u16, f64)>,
    /// Current grow-in phase.
    pub grow_in_phase: GrowInPhase,
    /// Effective visibility `[0..1]` observed on the previous advance.
    /// Used to detect 0→1 and 1→0 transitions.
    pub last_effective_visibility: f32,
    /// Set once the grow-in animation has fired — used by
    /// [`crate::cursor::GrowInMode::Once`] to suppress subsequent firings.
    pub grow_in_has_fired_once: bool,
}

impl CursorState {
    pub fn new() -> Self {
        Self::default()
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_state.rs</FILE> - <DESC>Runtime state for Cursor primitive</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
