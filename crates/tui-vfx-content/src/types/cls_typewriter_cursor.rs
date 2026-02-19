// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor configuration with signal-driven parameters</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>feat-20251224-170136: Complete signal-driven content effects</WCTX>
// <CLOG>Initial creation with signal-driven blink_interval, show_while_typing, show_after_complete</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Cursor configuration for Typewriter content effect
///
/// All time-varying parameters use SignalOrFloat for static or dynamic behavior.
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

// <FILE>tui-vfx-content/src/types/cls_typewriter_cursor.rs</FILE> - <DESC>TypewriterCursor configuration with signal-driven parameters</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
