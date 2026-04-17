// <FILE>tui-vfx-content/src/cursor/fnc_cursor_scan_glyph.rs</FILE> - <DESC>Map scan phase to glyph for a block cursor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-scan: the pure phase→glyph mapping for the new CursorScan axis. Pulse is a triangle wave through the 1/8th-block ramp; HalfBlockBounce is a three-step ▀ █ ▄ cycle. Non-block base cursors (|, _, ▌) are returned unchanged in both modes — non-block glyphs have no native "shape ramp" and would look mangled if overwritten.</WCTX>
// <CLOG>Initial impl — pub fn fnc_cursor_scan_glyph(base, phase, mode) -> String. Off passthrough, Pulse triangle over eight 1/8th blocks (▁..█..▁), HalfBlockBounce three-step ▀/█/▄ at thirds of the period. Peer test coverage: all three modes, phase endpoints and thirds, non-block passthrough.</CLOG>

use super::ScanMode;

const PULSE_BLOCKS: &[&str] = &["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/// Map a `phase ∈ [0, 1)` into the scan glyph for `base`.
///
/// Behavior:
/// - [`ScanMode::Off`] → `base` unchanged.
/// - [`ScanMode::Pulse`] (block only): phase drives a triangle wave over
///   the eight-frame 1/8th-block ramp `▁▂▃▄▅▆▇█`. Half the cycle grows up
///   from `▁` to `█`; the second half decays back down. Phase `0.0`
///   returns `▁`, phase `0.5` returns `█`, phase `1.0` returns `▁`.
///   Non-block base characters return `base` unchanged.
/// - [`ScanMode::HalfBlockBounce`] (block only): three-step cycle.
///   `phase ∈ [0, 1/3)` → `▀` (upper half), `[1/3, 2/3)` → `█`
///   (full — brief "both"), `[2/3, 1)` → `▄` (lower half). Non-block base
///   characters return `base` unchanged.
///
/// `phase` is clamped to `[0, 1]`; callers typically compute it as
/// `(now_ms % period_ms) / period_ms`.
pub fn fnc_cursor_scan_glyph(base: &str, phase: f32, mode: ScanMode) -> String {
    match mode {
        ScanMode::Off => base.to_string(),
        ScanMode::Pulse => scan_pulse_glyph(base, phase),
        ScanMode::HalfBlockBounce => scan_half_block_bounce_glyph(base, phase),
    }
}

fn scan_pulse_glyph(base: &str, phase: f32) -> String {
    if base != "█" {
        return base.to_string();
    }
    let p = phase.clamp(0.0, 1.0);
    // Triangle wave: 0 → 1 → 0 over the unit period.
    let tri = if p <= 0.5 { p * 2.0 } else { (1.0 - p) * 2.0 };
    // Map [0, 1] onto the 8-frame ramp (indices 0..=7).
    let n = PULSE_BLOCKS.len();
    let idx = ((tri * (n - 1) as f32).round() as usize).min(n - 1);
    PULSE_BLOCKS[idx].to_string()
}

fn scan_half_block_bounce_glyph(base: &str, phase: f32) -> String {
    if base != "█" {
        return base.to_string();
    }
    let p = phase.clamp(0.0, 1.0);
    // Clamping at 1.0 should still fall into the lower-half bucket, not
    // wrap to the upper-half one.
    if p < 1.0 / 3.0 {
        "▀".to_string()
    } else if p < 2.0 / 3.0 {
        "█".to_string()
    } else {
        "▄".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_returns_base_unchanged() {
        assert_eq!(fnc_cursor_scan_glyph("█", 0.0, ScanMode::Off), "█");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.5, ScanMode::Off), "█");
        assert_eq!(fnc_cursor_scan_glyph("|", 0.25, ScanMode::Off), "|");
    }

    #[test]
    fn pulse_endpoints_and_midpoint() {
        assert_eq!(fnc_cursor_scan_glyph("█", 0.0, ScanMode::Pulse), "▁");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.5, ScanMode::Pulse), "█");
        assert_eq!(fnc_cursor_scan_glyph("█", 1.0, ScanMode::Pulse), "▁");
    }

    #[test]
    fn pulse_midway_up_is_partial_block() {
        // phase 0.25 → triangle 0.5 → mid of ramp (▄ or ▅).
        let g = fnc_cursor_scan_glyph("█", 0.25, ScanMode::Pulse);
        assert!(["▄", "▅"].contains(&g.as_str()), "unexpected mid glyph: {g:?}");
    }

    #[test]
    fn pulse_non_block_passthrough() {
        for base in ["|", "_", "▌", "◆"] {
            assert_eq!(fnc_cursor_scan_glyph(base, 0.0, ScanMode::Pulse), base);
            assert_eq!(fnc_cursor_scan_glyph(base, 0.5, ScanMode::Pulse), base);
            assert_eq!(fnc_cursor_scan_glyph(base, 1.0, ScanMode::Pulse), base);
        }
    }

    #[test]
    fn half_block_bounce_three_step_cycle() {
        assert_eq!(fnc_cursor_scan_glyph("█", 0.0, ScanMode::HalfBlockBounce), "▀");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.2, ScanMode::HalfBlockBounce), "▀");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.4, ScanMode::HalfBlockBounce), "█");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.5, ScanMode::HalfBlockBounce), "█");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.7, ScanMode::HalfBlockBounce), "▄");
        assert_eq!(fnc_cursor_scan_glyph("█", 0.95, ScanMode::HalfBlockBounce), "▄");
    }

    #[test]
    fn half_block_bounce_non_block_passthrough() {
        for base in ["|", "_", "▌", "◆"] {
            assert_eq!(fnc_cursor_scan_glyph(base, 0.1, ScanMode::HalfBlockBounce), base);
            assert_eq!(fnc_cursor_scan_glyph(base, 0.5, ScanMode::HalfBlockBounce), base);
            assert_eq!(fnc_cursor_scan_glyph(base, 0.9, ScanMode::HalfBlockBounce), base);
        }
    }

    #[test]
    fn out_of_range_phase_clamps() {
        // Negative and >1 phases are clamped, not unwrapped.
        assert_eq!(fnc_cursor_scan_glyph("█", -0.5, ScanMode::Pulse), "▁");
        assert_eq!(fnc_cursor_scan_glyph("█", 1.5, ScanMode::Pulse), "▁");
        assert_eq!(fnc_cursor_scan_glyph("█", -0.5, ScanMode::HalfBlockBounce), "▀");
        assert_eq!(fnc_cursor_scan_glyph("█", 1.5, ScanMode::HalfBlockBounce), "▄");
    }
}

// <FILE>tui-vfx-content/src/cursor/fnc_cursor_scan_glyph.rs</FILE> - <DESC>Map scan phase to glyph for a block cursor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
