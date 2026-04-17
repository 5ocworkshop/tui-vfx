// <FILE>tui-vfx-content/src/cursor/cls_cursor.rs</FILE> - <DESC>Cursor primitive config</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>feat/cursor-braille: add static row-stacked braille convenience constructors (2/4/6/8 dots) so recipes and consumers can spawn ⠉ ⠛ ⠿ ⣿ cursors without a full struct literal.</WCTX>
// <CLOG>MINOR: Add braille_2 / braille_4 / braille_6 / braille_8 ctors producing ⠉ ⠛ ⠿ ⣿ respectively. Each sets `character` and otherwise returns a Cursor::default() — no other fields altered.</CLOG>

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
        Self {
            character: glyph.to_string(),
            ..Self::default()
        }
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

    /// Convenience constructor for a row-stacked 2-dot braille cursor (`⠉`).
    ///
    /// Braille characters in `U+2800..=U+28FF` encode an 8-dot 2×4 grid. The
    /// `braille_N` family produces a row-stacked fill: dots accumulate from the
    /// top row downward. `braille_2` fills only row 1 (the top row) — 2 dots.
    pub fn braille_2() -> Self {
        Self {
            character: "⠉".to_string(),
            ..Self::default()
        }
    }

    /// Convenience constructor for a row-stacked 4-dot braille cursor (`⠛`).
    ///
    /// Fills rows 1 and 2 — 4 dots total, the top half of the braille cell.
    pub fn braille_4() -> Self {
        Self {
            character: "⠛".to_string(),
            ..Self::default()
        }
    }

    /// Convenience constructor for a row-stacked 6-dot braille cursor (`⠿`).
    ///
    /// Fills rows 1 through 3 — 6 dots total, the top three-quarters of the
    /// braille cell.
    pub fn braille_6() -> Self {
        Self {
            character: "⠿".to_string(),
            ..Self::default()
        }
    }

    /// Convenience constructor for a fully-filled 8-dot braille cursor (`⣿`).
    ///
    /// All four rows filled — the densest braille glyph, visually equivalent
    /// to a solid block but at braille's 2×4 sub-cell density.
    pub fn braille_8() -> Self {
        Self {
            character: "⣿".to_string(),
            ..Self::default()
        }
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
// <VERS>END OF VERSION: 0.3.0</VERS>
