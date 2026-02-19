// <FILE>tui-vfx-style/tests/utils/test_fnc_color_ops.rs</FILE> - <DESC>Unit tests for color ops</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
// <WCTX>Turn 7 Audit Resolution</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_style::utils::{darken, degrade_color, rgb_to_indexed};
use tui_vfx_types::Color;
#[test]
fn test_darken() {
    let c = Color::rgb(100, 100, 100);
    // Darken by 50% -> (50, 50, 50)
    assert_eq!(darken(c, 0.5), Color::rgb(50, 50, 50));
}
#[test]
fn test_rgb_to_indexed() {
    // Pure Red -> Index 196 in 256 color map
    // Formula: 16 + 36*5 + 6*0 + 0 = 16 + 180 = 196
    assert_eq!(rgb_to_indexed(255, 0, 0), 196);
    // Pure Green -> Index 46
    // Formula: 16 + 36*0 + 6*5 + 0 = 16 + 30 = 46
    assert_eq!(rgb_to_indexed(0, 255, 0), 46);
}
#[test]
fn test_degrade_color() {
    let c = Color::rgb(255, 0, 0);
    // degrade_color now returns RGB approximation from 6x6x6 cube
    // Index 196 = 16 + 36*5 + 6*0 + 0 -> r=5, g=0, b=0 -> RGB(255, 0, 0)
    assert_eq!(degrade_color(c), Color::rgb(255, 0, 0));
    // Transparent should pass through
    assert_eq!(degrade_color(Color::TRANSPARENT), Color::TRANSPARENT);
}

// <FILE>tui-vfx-style/tests/utils/test_fnc_color_ops.rs</FILE> - <DESC>Unit tests for color ops</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
