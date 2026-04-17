// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_scan_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_scan_glyph</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-braille: extend peer tests with BraillePulse (phase endpoints, midpoint, full-cycle coverage, non-braille base override) and BrailleRowFlip (square-wave alternation, base override).</WCTX>
// <CLOG>MINOR: add braille_pulse_* and braille_row_flip_* tests matching the in-module coverage.</CLOG>

use tui_vfx_content::cursor::{ScanMode, fnc_cursor_scan_glyph};

#[test]
fn off_is_passthrough() {
    assert_eq!(fnc_cursor_scan_glyph("█", 0.25, ScanMode::Off), "█");
    assert_eq!(fnc_cursor_scan_glyph("|", 0.5, ScanMode::Off), "|");
}

#[test]
fn pulse_endpoints_and_mid() {
    assert_eq!(fnc_cursor_scan_glyph("█", 0.0, ScanMode::Pulse), "▁");
    assert_eq!(fnc_cursor_scan_glyph("█", 0.5, ScanMode::Pulse), "█");
    assert_eq!(fnc_cursor_scan_glyph("█", 1.0, ScanMode::Pulse), "▁");
}

#[test]
fn half_block_bounce_thirds() {
    assert_eq!(
        fnc_cursor_scan_glyph("█", 0.0, ScanMode::HalfBlockBounce),
        "▀"
    );
    assert_eq!(
        fnc_cursor_scan_glyph("█", 0.5, ScanMode::HalfBlockBounce),
        "█"
    );
    assert_eq!(
        fnc_cursor_scan_glyph("█", 0.99, ScanMode::HalfBlockBounce),
        "▄"
    );
}

#[test]
fn non_block_passthrough_both_modes() {
    for base in ["|", "_", "▌", "◆"] {
        for mode in [ScanMode::Pulse, ScanMode::HalfBlockBounce] {
            for p in [0.0, 0.25, 0.5, 0.75, 1.0] {
                assert_eq!(fnc_cursor_scan_glyph(base, p, mode), base);
            }
        }
    }
}

#[test]
fn out_of_range_phase_is_clamped() {
    assert_eq!(fnc_cursor_scan_glyph("█", -0.1, ScanMode::Pulse), "▁");
    assert_eq!(fnc_cursor_scan_glyph("█", 1.5, ScanMode::Pulse), "▁");
    assert_eq!(
        fnc_cursor_scan_glyph("█", -0.1, ScanMode::HalfBlockBounce),
        "▀"
    );
    assert_eq!(
        fnc_cursor_scan_glyph("█", 2.0, ScanMode::HalfBlockBounce),
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
    let g = fnc_cursor_scan_glyph("X", 0.0, ScanMode::BraillePulse);
    assert_eq!(g, "⣿");
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
        fnc_cursor_scan_glyph("X", 0.1, ScanMode::BrailleRowFlip),
        "⠉"
    );
    assert_eq!(
        fnc_cursor_scan_glyph("X", 0.9, ScanMode::BrailleRowFlip),
        "⠛"
    );
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_scan_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_scan_glyph</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
