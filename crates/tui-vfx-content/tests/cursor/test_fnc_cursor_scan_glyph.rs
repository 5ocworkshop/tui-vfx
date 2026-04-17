// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_scan_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_scan_glyph</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-scan: peer tests covering the phase→glyph mapping — Pulse triangle endpoints, HalfBlockBounce thirds, non-block passthrough, out-of-range clamp.</WCTX>
// <CLOG>Initial tests matching the in-module coverage so the integration test path also compiles these cases.</CLOG>

use tui_vfx_content::cursor::{fnc_cursor_scan_glyph, ScanMode};

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
    assert_eq!(fnc_cursor_scan_glyph("█", 0.0, ScanMode::HalfBlockBounce), "▀");
    assert_eq!(fnc_cursor_scan_glyph("█", 0.5, ScanMode::HalfBlockBounce), "█");
    assert_eq!(fnc_cursor_scan_glyph("█", 0.99, ScanMode::HalfBlockBounce), "▄");
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
    assert_eq!(fnc_cursor_scan_glyph("█", -0.1, ScanMode::HalfBlockBounce), "▀");
    assert_eq!(fnc_cursor_scan_glyph("█", 2.0, ScanMode::HalfBlockBounce), "▄");
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_scan_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_scan_glyph</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
