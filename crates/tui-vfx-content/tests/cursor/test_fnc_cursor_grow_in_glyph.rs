// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_grow_in_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_grow_in_glyph</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: grow-in glyph mapping</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::cursor::{fnc_cursor_grow_in_glyph, GrowDirection};

#[test]
fn block_up_maps_progress_to_eighth_blocks() {
    // 9 buckets: [invisible, ▁, ▂, ▃, ▄, ▅, ▆, ▇, █]
    let (g0, _) = fnc_cursor_grow_in_glyph("█", 0.0, GrowDirection::Up);
    let (g1, _) = fnc_cursor_grow_in_glyph("█", 0.125, GrowDirection::Up);
    let (g4, _) = fnc_cursor_grow_in_glyph("█", 0.5, GrowDirection::Up);
    let (g8, _) = fnc_cursor_grow_in_glyph("█", 1.0, GrowDirection::Up);
    assert_eq!(g0, ""); // invisible
    assert_eq!(g1, "▁");
    assert_eq!(g4, "▄");
    assert_eq!(g8, "█");
}

#[test]
fn block_down_uses_upper_block_set() {
    let (g_mid, _) = fnc_cursor_grow_in_glyph("█", 0.5, GrowDirection::Down);
    // Any non-empty glyph from the upper-block sequence is acceptable;
    // the critical invariant is g_mid != g for Up at the same progress.
    let (g_up_mid, _) = fnc_cursor_grow_in_glyph("█", 0.5, GrowDirection::Up);
    assert_ne!(g_mid, g_up_mid);
    assert!(!g_mid.is_empty());
}

#[test]
fn block_center_three_step() {
    let (g0, _) = fnc_cursor_grow_in_glyph("█", 0.0, GrowDirection::Center);
    let (g33, _) = fnc_cursor_grow_in_glyph("█", 0.34, GrowDirection::Center);
    let (g66, _) = fnc_cursor_grow_in_glyph("█", 0.67, GrowDirection::Center);
    let (g100, _) = fnc_cursor_grow_in_glyph("█", 1.0, GrowDirection::Center);
    assert_eq!(g0, "");
    assert_eq!(g33, "▄");
    assert_eq!(g66, "▆");
    assert_eq!(g100, "█");
}

#[test]
fn non_block_glyph_alpha_only() {
    let (g, a) = fnc_cursor_grow_in_glyph("|", 0.5, GrowDirection::Up);
    assert_eq!(g, "|");
    assert!((a - 0.5).abs() < 1e-6);
}

#[test]
fn non_block_underscore_alpha_only() {
    let (g, a) = fnc_cursor_grow_in_glyph("_", 0.25, GrowDirection::Center);
    assert_eq!(g, "_");
    assert!((a - 0.25).abs() < 1e-6);
}

#[test]
fn progress_out_of_range_is_clamped() {
    let (_, a_neg) = fnc_cursor_grow_in_glyph("█", -0.5, GrowDirection::Up);
    let (_, a_big) = fnc_cursor_grow_in_glyph("█", 5.0, GrowDirection::Up);
    assert_eq!(a_neg, 0.0);
    assert_eq!(a_big, 1.0);
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_cursor_grow_in_glyph.rs</FILE> - <DESC>Tests for fnc_cursor_grow_in_glyph</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
