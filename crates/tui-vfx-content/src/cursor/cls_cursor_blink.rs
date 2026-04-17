// <FILE>tui-vfx-content/src/cursor/cls_cursor_blink.rs</FILE> - <DESC>Blink config for Cursor primitive</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: CursorBlink</WCTX>
// <CLOG>Initial impl</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Blink configuration for a [`crate::cursor::Cursor`].
///
/// `interval_ms` is the on-duration in milliseconds, followed by the same off-duration.
/// A value of `0` (the default) disables blinking — the cursor is always visible.
///
/// # Example
///
/// ```
/// use tui_vfx_content::cursor::CursorBlink;
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let steady = CursorBlink::default(); // never blinks
/// let slow = CursorBlink { interval_ms: SignalOrFloat::Static(500.0) };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct CursorBlink {
    /// On/off duration in milliseconds. `0` = no blink (always visible).
    #[serde(alias = "blink_interval")] // legacy TypewriterCursor field name
    pub interval_ms: SignalOrFloat,
}

impl Default for CursorBlink {
    fn default() -> Self {
        Self { interval_ms: SignalOrFloat::Static(0.0) }
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_blink.rs</FILE> - <DESC>Blink config for Cursor primitive</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
