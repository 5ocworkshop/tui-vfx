// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor config composing the general Cursor primitive</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>feat/cursor-primitive: compose general Cursor via #[serde(flatten)]</WCTX>
// <CLOG>Refactor: blink_interval → cursor.blink.interval_ms (aliased), character → cursor.character (flattened). Legacy JSON and Rust ctors unchanged.</CLOG>

use crate::cursor::{Cursor, CursorBlink};
use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Typewriter-specific cursor configuration.
///
/// Wraps the general [`Cursor`] primitive and adds two typewriter-specific
/// visibility fields (`show_while_typing`, `show_after_complete`).
///
/// # Backward compatibility
///
/// JSON from pre-2.0 code still parses — `character` and `blink_interval`
/// were hoisted into the flattened [`Cursor`] and its [`crate::cursor::CursorBlink`].
/// All new fields (`grow_in`, `wake`, `visibility`) default to no-ops, so
/// rendering output is identical to v1.1.0 unless the author opts in.
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
/// # Example — plain block cursor (identical to v1.1.0)
///
/// ```
/// use tui_vfx_content::types::TypewriterCursor;
/// let cursor = TypewriterCursor::block();
/// assert_eq!(cursor.cursor.character, "█");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct TypewriterCursor {
    /// The general cursor primitive. Flattened into JSON so pre-2.0 recipes parse unchanged.
    #[serde(flatten)]
    pub cursor: Cursor,

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
            cursor: Cursor {
                // Keep v1.1.0 default: blink every 500ms.
                blink: CursorBlink {
                    interval_ms: SignalOrFloat::Static(500.0),
                },
                ..Cursor::default()
            },
            show_while_typing: SignalOrFloat::Static(1.0),
            show_after_complete: SignalOrFloat::Static(1.0),
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
            cursor: Cursor {
                character: glyph.to_string(),
                blink: CursorBlink {
                    interval_ms: SignalOrFloat::Static(500.0),
                },
                ..Cursor::default()
            },
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

// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor config composing the general Cursor primitive</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
