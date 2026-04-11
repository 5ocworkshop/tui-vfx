// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor configuration with signal-driven parameters</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>feat/content-ergonomics: TypewriterCursor convenience constructors</WCTX>
// <CLOG>Add simple/block/underscore/pipe/caret presets and Static defaults rustdoc</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Cursor configuration for Typewriter content effect.
///
/// All time-varying parameters use [`SignalOrFloat`] for either static or
/// dynamic behavior.
///
/// # Static defaults
///
/// For the common case ("I just want a cursor of shape X with the default
/// blink/visibility behavior") use the convenience constructors instead of
/// the full struct literal:
///
/// ```
/// use tui_vfx_content::types::TypewriterCursor;
///
/// let block = TypewriterCursor::block();        // █
/// let underscore = TypewriterCursor::underscore(); // _
/// let pipe = TypewriterCursor::pipe();          // |
/// let caret = TypewriterCursor::caret();        // ▌
/// let custom = TypewriterCursor::simple('◆');   // any single glyph
/// ```
///
/// All four [`SignalOrFloat`] fields accept `SignalOrFloat::Static(n)` for
/// the static case; reach for the signal-driven variants only when you need
/// per-frame variation (breathing cursors, dynamic blink rates, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct TypewriterCursor {
    /// Single character to display as cursor (e.g., "█", "_", "|")
    /// Empty string disables cursor
    pub character: String,

    /// Blink interval in milliseconds - can be static or dynamic signal
    /// Static: 500 means 500ms on, 500ms off
    /// Signal: Evaluated per-frame for organic/varying blink rates
    /// Values <= 0 mean always visible (no blinking)
    pub blink_interval: SignalOrFloat,

    /// Show cursor at typing position while text is being revealed
    /// 0.0 = hidden, 1.0 = visible, between = alpha blend (threshold at 0.5)
    /// Can be static or signal-driven for pulsing/fading effects
    pub show_while_typing: SignalOrFloat,

    /// Show cursor at end of text after typing completes
    /// 0.0 = hidden, 1.0 = visible, between = alpha blend (threshold at 0.5)
    /// Can be static or signal-driven for fade-out effects
    pub show_after_complete: SignalOrFloat,
}

impl Default for TypewriterCursor {
    fn default() -> Self {
        Self {
            character: "█".to_string(),
            blink_interval: SignalOrFloat::Static(500.0),
            show_while_typing: SignalOrFloat::Static(1.0), // Fully visible
            show_after_complete: SignalOrFloat::Static(1.0), // Fully visible
        }
    }
}

impl TypewriterCursor {
    /// Returns a cursor that displays `glyph` while the typewriter is still
    /// revealing characters and continues to show the glyph after the
    /// reveal completes.
    ///
    /// Equivalent to [`TypewriterCursor::default`] but with a configurable
    /// glyph. This is the one-line constructor for the common case "I just
    /// want a cursor of shape X".
    pub fn simple(glyph: char) -> Self {
        Self {
            character: glyph.to_string(),
            ..Self::default()
        }
    }

    /// Convenience constructor for a block cursor (`█`).
    ///
    /// Equivalent to `TypewriterCursor::simple('█')`, which itself is
    /// equivalent to [`TypewriterCursor::default`].
    pub fn block() -> Self {
        Self::simple('█')
    }

    /// Convenience constructor for an underscore cursor (`_`).
    ///
    /// Equivalent to `TypewriterCursor::simple('_')`.
    pub fn underscore() -> Self {
        Self::simple('_')
    }

    /// Convenience constructor for a pipe cursor (`|`).
    ///
    /// Equivalent to `TypewriterCursor::simple('|')`.
    pub fn pipe() -> Self {
        Self::simple('|')
    }

    /// Convenience constructor for a left half-block caret cursor (`▌`).
    ///
    /// Equivalent to `TypewriterCursor::simple('▌')`.
    pub fn caret() -> Self {
        Self::simple('▌')
    }
}

// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor configuration with signal-driven parameters</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
