// <FILE>tui-vfx-style/tests/models/test_cls_style_transition.rs</FILE> - <DESC>Integration test for StyleTransition</DESC>
// <VERS>VERSION: 0.3.0 - 2025-12-16T20:54:55Z</VERS>
// <WCTX>Turn 4 Implementation</WCTX>
// <CLOG>Added interpolation tests</CLOG>

use tui_vfx_style::models::StyleTransition;
use tui_vfx_style::traits::StyleInterpolator;
use tui_vfx_types::{Color, Style};
#[test]
fn test_can_instantiate_transition_defaults() {
    let start = Style::fg(Color::RED);
    let end = Style::fg(Color::BLUE);
    let transition = StyleTransition::new(start, end);
    assert_eq!(transition.start.fg, Color::RED);
    assert_eq!(transition.end.fg, Color::BLUE);
}
#[test]
fn test_calculate_interpolation() {
    let start = Style::fg(Color::rgb(0, 0, 0));
    let end = Style::fg(Color::rgb(100, 100, 100));
    let transition = StyleTransition::new(start, end);
    let base = Style::default();
    let result = transition.calculate(0.5, base);
    assert_eq!(result.fg, Color::rgb(50, 50, 50));
}
#[test]
fn test_calculate_with_ansi_fallback() {
    let start = Style::fg(Color::RED); // Red (255,0,0)
    let end = Style::fg(Color::BLUE); // Blue (0,0,255)
    let transition = StyleTransition::new(start, end);
    let base = Style::default();
    let result = transition.calculate(0.5, base);
    // Should blend to (127, 0, 127)
    assert_eq!(result.fg, Color::rgb(127, 0, 127));
}

// <FILE>tui-vfx-style/tests/models/test_cls_style_transition.rs</FILE> - <DESC>Integration test for StyleTransition</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2025-12-16T20:54:55Z</VERS>
