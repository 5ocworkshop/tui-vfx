// <FILE>tui-vfx-content/src/cursor/fnc_cursor_scan_glyph.rs</FILE> - <DESC>Map scan phase to glyph for a block cursor</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-braille: add phase→glyph mapping for two new braille-glyph scan modes. BraillePulse sine-eases over the four row-stacked fills (⣿ ⠿ ⠛ ⠉) with phase 0/1 at densest (⣿) and phase 0.5 at sparsest (⠉). BrailleRowFlip is a slow square-wave alternation between ⠉ and ⠛. Both modes REPLACE the base glyph unconditionally (unlike Pulse/HalfBlockBounce which only rewrite the block `█` base).</WCTX>
// <CLOG>MINOR: route ScanMode::BraillePulse and ScanMode::BrailleRowFlip through new scan_braille_pulse_glyph / scan_braille_row_flip_glyph helpers. Both override base glyph. Peer tests cover endpoints, midpoint, non-braille base override, and square-wave boundaries.</CLOG>

use super::ScanMode;

const PULSE_BLOCKS: &[&str] = &["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/// Row-stacked braille fills indexed by row-count 1..=4.
/// Index 0: 1 row (⠉, 2 dots), …, index 3: 4 rows (⣿, 8 dots).
const BRAILLE_ROW_FILLS: &[&str] = &["⠉", "⠛", "⠿", "⣿"];

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
/// - [`ScanMode::BraillePulse`] (overrides base): sine-eased cycle through
///   the four row-stacked braille fills. Phase `0` / `1` → `⣿` (8 dots),
///   phase `0.5` → `⠉` (2 dots, top row only). The `base` glyph is
///   **replaced** — no passthrough — because the braille cell set has no
///   sensible mapping for non-braille base characters.
/// - [`ScanMode::BrailleRowFlip`] (overrides base): square-wave alternation
///   between `⠉` (phase `<0.5`) and `⠛` (phase `≥0.5`). Like
///   `BraillePulse`, `base` is replaced unconditionally.
///
/// `phase` is clamped to `[0, 1]`; callers typically compute it as
/// `(now_ms % period_ms) / period_ms`.
pub fn fnc_cursor_scan_glyph(base: &str, phase: f32, mode: ScanMode) -> String {
    match mode {
        ScanMode::Off => base.to_string(),
        ScanMode::Pulse => scan_pulse_glyph(base, phase),
        ScanMode::HalfBlockBounce => scan_half_block_bounce_glyph(base, phase),
        ScanMode::BraillePulse => scan_braille_pulse_glyph(phase),
        ScanMode::BrailleRowFlip => scan_braille_row_flip_glyph(phase),
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

fn scan_braille_pulse_glyph(phase: f32) -> String {
    let p = phase.clamp(0.0, 1.0);
    // Sine-eased cycle: cos(2π·phase)·0.5 + 0.5 ∈ [0, 1].
    // Phase 0.0 and 1.0 → 1.0 (max rows); phase 0.5 → 0.0 (min rows).
    let sine = (p * std::f32::consts::TAU).cos() * 0.5 + 0.5;
    // Map [0, 1] onto row counts 1..=4 (indices 0..=3).
    let idx = (sine * 3.0).round().clamp(0.0, 3.0) as usize;
    BRAILLE_ROW_FILLS[idx].to_string()
}

fn scan_braille_row_flip_glyph(phase: f32) -> String {
    let p = phase.clamp(0.0, 1.0);
    if p < 0.5 {
        "⠉".to_string()
    } else {
        "⠛".to_string()
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
        assert!(
            ["▄", "▅"].contains(&g.as_str()),
            "unexpected mid glyph: {g:?}"
        );
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
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.0, ScanMode::HalfBlockBounce),
            "▀"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.2, ScanMode::HalfBlockBounce),
            "▀"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.4, ScanMode::HalfBlockBounce),
            "█"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.5, ScanMode::HalfBlockBounce),
            "█"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.7, ScanMode::HalfBlockBounce),
            "▄"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 0.95, ScanMode::HalfBlockBounce),
            "▄"
        );
    }

    #[test]
    fn half_block_bounce_non_block_passthrough() {
        for base in ["|", "_", "▌", "◆"] {
            assert_eq!(
                fnc_cursor_scan_glyph(base, 0.1, ScanMode::HalfBlockBounce),
                base
            );
            assert_eq!(
                fnc_cursor_scan_glyph(base, 0.5, ScanMode::HalfBlockBounce),
                base
            );
            assert_eq!(
                fnc_cursor_scan_glyph(base, 0.9, ScanMode::HalfBlockBounce),
                base
            );
        }
    }

    #[test]
    fn out_of_range_phase_clamps() {
        // Negative and >1 phases are clamped, not unwrapped.
        assert_eq!(fnc_cursor_scan_glyph("█", -0.5, ScanMode::Pulse), "▁");
        assert_eq!(fnc_cursor_scan_glyph("█", 1.5, ScanMode::Pulse), "▁");
        assert_eq!(
            fnc_cursor_scan_glyph("█", -0.5, ScanMode::HalfBlockBounce),
            "▀"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("█", 1.5, ScanMode::HalfBlockBounce),
            "▄"
        );
    }

    #[test]
    fn braille_pulse_at_phase_zero_is_full_8_dots() {
        let g = fnc_cursor_scan_glyph("⣿", 0.0, ScanMode::BraillePulse);
        assert_eq!(g, "⣿");
    }

    #[test]
    fn braille_pulse_at_phase_half_is_minimum_1_row() {
        let g = fnc_cursor_scan_glyph("⣿", 0.5, ScanMode::BraillePulse);
        assert_eq!(g, "⠉");
    }

    #[test]
    fn braille_pulse_returns_to_full_at_phase_one() {
        let g = fnc_cursor_scan_glyph("⣿", 1.0, ScanMode::BraillePulse);
        assert_eq!(g, "⣿");
    }

    #[test]
    fn braille_pulse_overrides_non_braille_base() {
        // The mode replaces the glyph regardless of base.
        let g = fnc_cursor_scan_glyph("X", 0.0, ScanMode::BraillePulse);
        assert_eq!(g, "⣿");
    }

    #[test]
    fn braille_pulse_covers_all_four_row_fills_across_cycle() {
        // Sweep phase; each of the four glyphs must appear at some point.
        let mut seen = [false; 4];
        for i in 0..=100 {
            let p = i as f32 / 100.0;
            let g = fnc_cursor_scan_glyph("⣿", p, ScanMode::BraillePulse);
            match g.as_str() {
                "⠉" => seen[0] = true,
                "⠛" => seen[1] = true,
                "⠿" => seen[2] = true,
                "⣿" => seen[3] = true,
                other => panic!("unexpected braille glyph: {other:?}"),
            }
        }
        assert!(
            seen.iter().all(|s| *s),
            "braille pulse failed to cover all four row-fills: {seen:?}"
        );
    }

    #[test]
    fn braille_row_flip_alternates() {
        assert_eq!(
            fnc_cursor_scan_glyph("⠉", 0.0, ScanMode::BrailleRowFlip),
            "⠉"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("⠉", 0.25, ScanMode::BrailleRowFlip),
            "⠉"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("⠉", 0.5, ScanMode::BrailleRowFlip),
            "⠛"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("⠉", 0.75, ScanMode::BrailleRowFlip),
            "⠛"
        );
    }

    #[test]
    fn braille_row_flip_overrides_non_braille_base() {
        assert_eq!(
            fnc_cursor_scan_glyph("X", 0.0, ScanMode::BrailleRowFlip),
            "⠉"
        );
        assert_eq!(
            fnc_cursor_scan_glyph("X", 0.99, ScanMode::BrailleRowFlip),
            "⠛"
        );
    }
}

// <FILE>tui-vfx-content/src/cursor/fnc_cursor_scan_glyph.rs</FILE> - <DESC>Map scan phase to glyph for a block cursor</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
