// <FILE>tui-vfx-style/tests/utils/test_fnc_blend_colors.rs</FILE> - <DESC>Unit tests for color math</DESC>
// <VERS>VERSION: 0.2.0 - 2026-01-12</VERS>
// <WCTX>L2/L3 abstraction: tui-style-fx uses mixed-types</WCTX>
// <CLOG>Fixed test_blend_with_transparent to expect linear interpolation</CLOG>

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

// <FILE>tui-vfx-style/tests/utils/test_fnc_blend_colors.rs</FILE> - <DESC>Unit tests for color math</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2026-01-12</VERS>
