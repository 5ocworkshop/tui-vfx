// <FILE>tui-vfx-content/src/cursor/cls_cursor_scan.rs</FILE> - <DESC>Scan config for Cursor primitive (steady-cursor shape cycling)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-scan: add a third cursor animation axis (alongside grow-in and wake) that modulates the cursor glyph through a bounded shape cycle while the cursor is parked. Pulse cycles through the 1/8th-block ramp (▁..█..▁) to read as a soft breath; HalfBlockBounce flips between ▀ █ ▄ for a mechanical scanner feel.</WCTX>
// <CLOG>Initial impl — ScanMode enum (Off/Pulse/HalfBlockBounce, #[default] Off, snake_case serde), CursorScan struct with mode/period_ms/curve (all SignalOrFloat), defaults to no-op, CursorScan::noop() alias. Matches the shape of GrowIn/Wake config structs.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Controls how the cursor glyph cycles while the cursor is visible and
/// parked (i.e. grow-in has finished).
///
/// Applies only to block (`█`) cursors; non-block cursors (e.g. `|`, `_`,
/// `▌`) ignore scan entirely and render their base glyph.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ScanMode {
    /// No scan — cursor renders its base glyph. (Default.)
    #[default]
    Off,
    /// Triangle-wave through the 1/8th-block ramp `▁▂▃▄▅▆▇█` and back.
    /// Half the cycle grows up, half comes down. Reads as a gentle breath.
    Pulse,
    /// Three-step cycle: upper half (`▀`), full (`█`, brief "both"),
    /// lower half (`▄`). Reads as a mechanical scanner.
    HalfBlockBounce,
}

/// Scan animation config for a [`crate::cursor::Cursor`].
///
/// All fields default to a no-op: [`ScanMode::Off`] + zero period.
/// Calling [`CursorScan::default`] produces the same static-cursor
/// behavior as a cursor with no animation.
///
/// Scan fires only while the cursor is in
/// [`crate::cursor::GrowInPhase::Visible`] — grow-in takes precedence.
///
/// # Example
///
/// ```
/// use tui_vfx_content::cursor::{CursorScan, ScanMode};
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let static_cursor = CursorScan::default();  // no scan
/// let pulsing = CursorScan {
///     mode: ScanMode::Pulse,
///     period_ms: SignalOrFloat::Static(1500.0),
///     ..CursorScan::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct CursorScan {
    pub mode: ScanMode,
    /// Full cycle duration in ms. `0` (default) disables the scan regardless
    /// of `mode`.
    pub period_ms: SignalOrFloat,
    /// Easing curve sampled with `t in 0..1`, returning eased phase in `0..1`.
    /// Default `Static(1.0)` is treated as linear identity.
    pub curve: SignalOrFloat,
}

impl Default for CursorScan {
    fn default() -> Self {
        Self {
            mode: ScanMode::Off,
            period_ms: SignalOrFloat::Static(0.0),
            curve: SignalOrFloat::Static(1.0),
        }
    }
}

impl CursorScan {
    /// Explicit alias for [`CursorScan::default`] — no scan.
    pub fn noop() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_off_with_zero_period() {
        let s = CursorScan::default();
        assert_eq!(s.mode, ScanMode::Off);
        match s.period_ms {
            SignalOrFloat::Static(v) => assert_eq!(v, 0.0),
            _ => panic!("period_ms should be Static(0.0) by default"),
        }
    }

    #[test]
    fn noop_equals_default() {
        assert_eq!(CursorScan::noop(), CursorScan::default());
    }

    #[test]
    fn scan_mode_default_is_off() {
        assert_eq!(ScanMode::default(), ScanMode::Off);
    }

    #[test]
    fn serde_roundtrip_snake_case() {
        let json = r#"{"mode":"pulse","period_ms":1500.0,"curve":1.0}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse");
        assert_eq!(parsed.mode, ScanMode::Pulse);

        let json = r#"{"mode":"half_block_bounce","period_ms":900.0}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse");
        assert_eq!(parsed.mode, ScanMode::HalfBlockBounce);

        let json = r#"{}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse default");
        assert_eq!(parsed, CursorScan::default());
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_scan.rs</FILE> - <DESC>Scan config for Cursor primitive (steady-cursor shape cycling)</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
