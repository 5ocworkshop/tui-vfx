// <FILE>tui-vfx-style/tests/utils/test_fnc_blend_colors.rs</FILE> - <DESC>Unit tests for color math — RGB, HSL, HCT</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-26</VERS>
// <WCTX>TTE effects port — add HCT-mode coverage so the new ColorSpace::Hct arm is exercised alongside the existing RGB and HSL paths.</WCTX>
// <CLOG>0.3.0: add HCT blend tests — endpoint identity, midpoint stays in palette, hue dominance through gradient interior, monotonic perceptual lerp.</CLOG>

use tui_vfx_style::models::ColorSpace;
use tui_vfx_style::utils::blend_colors;
use tui_vfx_types::Color;
#[test]
fn test_blend_rgb_exact() {
    let c1 = Color::rgb(255, 0, 0); // Red
    let c2 = Color::rgb(0, 0, 255); // Blue
    // 50% blend -> 127, 0, 127
    let result = blend_colors(c1, c2, 0.5, ColorSpace::Rgb);
    assert_eq!(result, Color::rgb(127, 0, 127));
}
#[test]
fn test_blend_hsl_hue_rotation() {
    let c1 = Color::rgb(255, 0, 0); // Red (Hue 0)
    let c2 = Color::rgb(255, 255, 0); // Yellow (Hue 60)
    // 50% blend -> Hue 30 (Orange) -> R=255, G=127, B=0
    let result = blend_colors(c1, c2, 0.5, ColorSpace::Hsl);
    // Floating point math might be slightly off, check tolerance or exact integer cast
    // 255, 127, 0 is expected
    assert_eq!(result, Color::rgb(255, 127, 0));
}
#[test]
fn test_blend_with_transparent() {
    let c1 = Color::RED; // (255, 0, 0)
    let c2 = Color::TRANSPARENT; // (0, 0, 0, a=0) - RGB values are 0
    // Linear interpolation: at t=0.4, r = 255 * 0.6 = 153
    let result = blend_colors(c1, c2, 0.4, ColorSpace::Rgb);
    assert_eq!(result, Color::rgb(153, 0, 0));
    // At t=0.6, r = 255 * 0.4 = 102
    let result2 = blend_colors(c1, c2, 0.6, ColorSpace::Rgb);
    assert_eq!(result2, Color::rgb(102, 0, 0));
}
#[test]
fn test_red_blue_blend() {
    // tui_vfx_types::Color::RED is (255,0,0)
    // tui_vfx_types::Color::BLUE is (0,0,255)
    // Blend 50% -> (127, 0, 127)
    let c1 = Color::RED;
    let c2 = Color::BLUE;
    let result = blend_colors(c1, c2, 0.5, ColorSpace::Rgb);
    assert_eq!(result, Color::rgb(127, 0, 127));
}

// --- HCT mode (perceptually-uniform interpolation via mcu-hct) ---

#[test]
fn test_blend_hct_endpoints_are_input_within_tolerance() {
    let c1 = Color::rgb(255, 0, 0);
    let c2 = Color::rgb(0, 0, 255);
    // t=0 returns c1 exactly (early-return), t=1 returns c2 exactly (early-return).
    assert_eq!(blend_colors(c1, c2, 0.0, ColorSpace::Hct), c1);
    assert_eq!(blend_colors(c1, c2, 1.0, ColorSpace::Hct), c2);
}

#[test]
fn test_blend_hct_red_to_blue_passes_through_purple_not_gray() {
    // Hue lerps along the shortest path; midpoint between red (h≈25) and
    // blue (h≈285) on the short arc lands on magenta/purple, not gray.
    let c1 = Color::rgb(255, 0, 0);
    let c2 = Color::rgb(0, 0, 255);
    let mid = blend_colors(c1, c2, 0.5, ColorSpace::Hct);
    assert!(
        mid.r > 50,
        "midpoint should not be near-gray, got {:?}",
        mid
    );
    assert!(
        mid.b > 50,
        "midpoint should not be near-gray, got {:?}",
        mid
    );
    // Green should be much lower than red and blue.
    assert!(
        mid.g < mid.r,
        "midpoint g should be < r in red->blue, got {:?}",
        mid
    );
    assert!(
        mid.g < mid.b,
        "midpoint g should be < b in red->blue, got {:?}",
        mid
    );
}

#[test]
fn test_blend_hct_red_to_white_progressively_brightens() {
    // Tone interpolates linearly. Mid-stop should be lighter than start.
    let c1 = Color::rgb(255, 0, 0);
    let c2 = Color::rgb(255, 255, 255);
    let mid = blend_colors(c1, c2, 0.5, ColorSpace::Hct);
    let sum_in = c1.r as i32 + c1.g as i32 + c1.b as i32;
    let sum_mid = mid.r as i32 + mid.g as i32 + mid.b as i32;
    assert!(
        sum_mid > sum_in,
        "mid should be brighter than start, got {:?}",
        mid
    );
}

#[test]
fn test_blend_hct_monotonic_in_tone() {
    // Across (0.0, 0.25, 0.5, 0.75, 1.0) for red→white the channel sum
    // (proxy for perceived brightness) should increase monotonically.
    let c1 = Color::rgb(180, 0, 0);
    let c2 = Color::rgb(255, 255, 255);
    let sum = |c: Color| c.r as i32 + c.g as i32 + c.b as i32;
    let mut last = sum(c1) - 1;
    for &t in &[0.0_f32, 0.25, 0.5, 0.75, 1.0] {
        let s = sum(blend_colors(c1, c2, t, ColorSpace::Hct));
        assert!(
            s >= last,
            "non-monotonic at t={t}: previous sum {last}, this sum {s}"
        );
        last = s;
    }
}

// <FILE>tui-vfx-style/tests/utils/test_fnc_blend_colors.rs</FILE> - <DESC>Unit tests for color math — RGB, HSL, HCT</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-26</VERS>
