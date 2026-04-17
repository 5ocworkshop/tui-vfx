// <FILE>tui-vfx-content/src/cursor/cls_cursor_wake.rs</FILE> - <DESC>Wake config for Cursor primitive</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add gap_cells field so the trail can be optionally disconnected from the live cursor; default 0 keeps the trail visually adjacent to the cursor</WCTX>
// <CLOG>MINOR: gap_cells: u32 (default 0). 0 = trail starts in the cell immediately behind the cursor; N > 0 inserts an N-cell unpainted gap before the trail begins.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::ColorConfig;

/// Controls how the cursor trail paints.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WakeMode {
    /// No trail. (Default.)
    #[default]
    Off,
    /// Trail paints color tint on whatever glyph is beneath. Cursor contributes color only.
    Tint,
    /// Trail paints a fading copy of the cursor character itself.
    /// **Limitation:** ignores wide-glyph content beneath — see spec E9.
    Ghost,
}

/// Wake trail configuration for a [`crate::cursor::Cursor`].
///
/// All fields default to a no-op: [`WakeMode::Off`] + zero decay.
/// Setting `decay_seconds = 0` with any other mode is also treated as off
/// (see spec E11).
///
/// # Example
///
/// ```
/// use tui_vfx_content::cursor::{Wake, WakeMode};
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let static_cursor = Wake::default();                  // no trail
/// let tinted = Wake {
///     mode: WakeMode::Tint,
///     decay_seconds: SignalOrFloat::Static(1.5),
///     max_cells: 8,
///     ..Wake::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct Wake {
    pub mode: WakeMode,
    /// Per-cell decay duration in seconds. `0` = off (regardless of `mode`).
    pub decay_seconds: SignalOrFloat,
    /// Hard cap on trail length. `0` = no cap (time-only).
    pub max_cells: u32,
    /// Cells of unpainted gap between the live cursor and the start of the
    /// trail. `0` (default) keeps the trail visually connected to the cursor —
    /// the cell immediately behind the cursor is fully tinted. Larger values
    /// insert N unpainted cells before the trail begins, for a detached
    /// "meteor tail" look.
    pub gap_cells: u32,
    /// Decay curve sampled with age-normalized `t in 0..1`, returning alpha in `0..1`.
    /// Default `Static(1.0)` is treated as linear by `fnc_render_cursor`.
    pub curve: SignalOrFloat,
    /// Color tint used by both Tint and Ghost modes. Theme-aware via `ColorConfig`.
    pub tint: ColorConfig,
}

impl Default for Wake {
    fn default() -> Self {
        Self {
            mode: WakeMode::Off,
            decay_seconds: SignalOrFloat::Static(0.0),
            max_cells: 0,
            gap_cells: 0,
            curve: SignalOrFloat::Static(1.0),
            tint: ColorConfig::default(),
        }
    }
}

impl Wake {
    /// Explicit alias for [`Wake::default`] — no trail.
    pub fn noop() -> Self {
        Self::default()
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_wake.rs</FILE> - <DESC>Wake config for Cursor primitive</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
