// <FILE>tui-vfx-content/src/cursor/cls_cursor.rs</FILE> - <DESC>Cursor primitive config</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-scan: add a third cursor animation axis — CursorScan — that modulates the glyph through a bounded shape cycle while the cursor is parked. Grow-in → wake → scan is the animation precedence; scan runs only in the Visible phase.</WCTX>
// <CLOG>MINOR: Add `scan: CursorScan` field, default = CursorScan::default() (ScanMode::Off + period_ms 0). #[serde(default)] at the struct level carries the backcompat — pre-0.2 JSON parses unchanged.</CLOG>

use super::{CursorBlink, CursorScan, GrowIn, GrowInMode, Wake, WakeMode};
use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// General-purpose cursor primitive.
///
/// Authors configure this; runtime bookkeeping lives in
/// [`crate::cursor::CursorState`].
///
/// All animation fields default to a no-op. `Cursor::default()` is a plain
/// static block cursor with no blink, no grow-in, and no wake — identical to
/// the pre-refactor [`crate::types::TypewriterCursor::block`] behavior.
///
/// # Example (no animation, static cursor)
///
/// ```
/// use tui_vfx_content::cursor::Cursor;
/// let cursor = Cursor::default();
/// assert_eq!(cursor.character, "█");
/// ```
///
/// # Example (opt-in grow-in)
///
/// ```
/// use tui_vfx_content::cursor::{Cursor, GrowIn, GrowInMode};
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let cursor = Cursor {
///     grow_in: GrowIn { mode: GrowInMode::Once, duration_ms: SignalOrFloat::Static(180.0), ..GrowIn::default() },
///     ..Cursor::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct Cursor {
    /// Glyph to display. Empty string suppresses the cursor entirely (spec E10).
    pub character: String,
    /// Base visibility 0..1. Multiplied with any consumer-side phase visibility
    /// (e.g. typewriter's `show_while_typing` / `show_after_complete`) to
    /// produce the effective visibility the grow-in state machine observes.
    pub visibility: SignalOrFloat,
    /// Blink configuration. Flattened into the serialized JSON so the legacy
    /// `blink_interval` key (see [`CursorBlink::interval_ms`]'s serde alias)
    /// parses at the top level of a [`Cursor`] — required for pre-2.0
    /// [`crate::types::TypewriterCursor`] JSON to round-trip.
    #[serde(flatten)]
    pub blink: CursorBlink,
    pub grow_in: GrowIn,
    pub wake: Wake,
    pub scan: CursorScan,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            character: "█".to_string(),
            visibility: SignalOrFloat::Static(1.0),
            blink: CursorBlink::default(),
            grow_in: GrowIn::default(),
            wake: Wake::default(),
            scan: CursorScan::default(),
        }
    }
}

impl Cursor {
    /// Returns a cursor with the provided single glyph and otherwise-default config.
    pub fn simple(glyph: char) -> Self {
        Self { character: glyph.to_string(), ..Self::default() }
    }

    /// Convenience constructor for a block cursor (`█`). Equivalent to [`Cursor::default`].
    pub fn block() -> Self {
        Self::simple('█')
    }

    /// Convenience constructor for an underscore cursor (`_`).
    pub fn underscore() -> Self {
        Self::simple('_')
    }

    /// Convenience constructor for a pipe cursor (`|`).
    pub fn pipe() -> Self {
        Self::simple('|')
    }

    /// Convenience constructor for a left half-block caret (`▌`).
    pub fn caret() -> Self {
        Self::simple('▌')
    }

    /// Returns a copy with grow-in enabled in [`GrowInMode::Once`] with the given duration.
    pub fn with_grow_in(mut self, duration_ms: f32) -> Self {
        self.grow_in.mode = GrowInMode::Once;
        self.grow_in.duration_ms = SignalOrFloat::Static(duration_ms);
        self
    }

    /// Returns a copy with a [`WakeMode::Tint`] trail of the given decay and cap.
    /// `max_cells = 0` means no cap.
    pub fn with_wake_tint(mut self, decay_seconds: f32, max_cells: u32) -> Self {
        self.wake.mode = WakeMode::Tint;
        self.wake.decay_seconds = SignalOrFloat::Static(decay_seconds);
        self.wake.max_cells = max_cells;
        self
    }

    /// Returns a copy with a [`WakeMode::Ghost`] trail of the given decay and cap.
    /// `max_cells = 0` means no cap.
    pub fn with_wake_ghost(mut self, decay_seconds: f32, max_cells: u32) -> Self {
        self.wake.mode = WakeMode::Ghost;
        self.wake.decay_seconds = SignalOrFloat::Static(decay_seconds);
        self.wake.max_cells = max_cells;
        self
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor.rs</FILE> - <DESC>Cursor primitive config</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
