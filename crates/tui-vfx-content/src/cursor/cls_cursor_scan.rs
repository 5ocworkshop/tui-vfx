// <FILE>tui-vfx-content/src/cursor/cls_cursor_scan.rs</FILE> - <DESC>Scan config for Cursor primitive (steady-cursor shape cycling)</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-braille: add two braille-glyph scan modes (BraillePulse, BrailleRowFlip) that cycle the cursor through row-stacked braille fills. Unlike Pulse/HalfBlockBounce these modes OVERRIDE the base glyph entirely (no passthrough) — the braille character set has no natural passthrough for non-braille base chars.</WCTX>
// <CLOG>MINOR: Add ScanMode::BraillePulse and ScanMode::BrailleRowFlip variants with serde snake_case (`braille_pulse`, `braille_row_flip`). Existing ScanMode variants and CursorScan struct unchanged.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use serde::{Deserialize, Serialize};

/// Controls how the cursor glyph cycles while the cursor is visible and
/// parked (i.e. grow-in has finished).
///
/// Applies only to block (`█`) cursors; non-block cursors (e.g. `|`, `_`,
/// `▌`) ignore scan entirely and render their base glyph.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
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
    /// Sine-eased cycle through the four row-stacked braille fills
    /// `⣿ → ⠿ → ⠛ → ⠉ → ⠛ → ⠿ → ⣿`. Phase `0` / `1` land on the densest
    /// glyph (8 dots); phase `0.5` lands on the sparsest (2 dots, top row
    /// only). Reads as a sub-cell braille breath.
    ///
    /// Unlike [`ScanMode::Pulse`] and [`ScanMode::HalfBlockBounce`], this
    /// mode **replaces** the cursor's base `character` regardless of what it
    /// was — the output is always one of the four braille row-fills. Set the
    /// cursor's base character to match what you want phase 0 to look like
    /// (e.g. `⣿`) so the "resting" frame and the scanned frame agree.
    BraillePulse,
    /// Slow square-wave alternation between a 2-dot (`⠉`, top row only) and
    /// 4-dot (`⠛`, top two rows) braille fill. Phase `<0.5` → `⠉`; phase
    /// `≥0.5` → `⠛`. Designed for long `period_ms` (~1.8–2.4s) so it reads
    /// as a calm indicator, not a flicker.
    ///
    /// Like [`ScanMode::BraillePulse`] this mode replaces the base
    /// `character` unconditionally — the output is always one of the two
    /// row-stacked braille glyphs.
    BrailleRowFlip,
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

        let json = r#"{"mode":"braille_pulse","period_ms":2800.0}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse");
        assert_eq!(parsed.mode, ScanMode::BraillePulse);

        let json = r#"{"mode":"braille_row_flip","period_ms":2000.0}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse");
        assert_eq!(parsed.mode, ScanMode::BrailleRowFlip);

        let json = r#"{}"#;
        let parsed: CursorScan = serde_json::from_str(json).expect("parse default");
        assert_eq!(parsed, CursorScan::default());
    }
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_scan.rs</FILE> - <DESC>Scan config for Cursor primitive (steady-cursor shape cycling)</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
