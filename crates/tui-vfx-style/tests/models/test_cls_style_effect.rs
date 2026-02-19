// <FILE>tui-vfx-style/tests/models/test_cls_style_effect.rs</FILE> - <DESC>Integration tests for StyleEffect</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>L2/L3 abstraction: tui-style-fx uses mixed-types</WCTX>
// <CLOG>Updated rainbow test for bright Color::RED behavior</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::{FadeApplyTo, StyleEffect};
use tui_vfx_style::traits::StyleInterpolator;
use tui_vfx_types::{Color, Style};
#[test]
fn test_rainbow_hue_shift() {
    let base = Style::fg(Color::RED);
    let effect = StyleEffect::Rainbow { speed: 0.5 };
    let result = effect.calculate(1.0, base);
    // Color::RED is bright red (255,0,0), so rainbow shift produces bright cyan
    // Allow slight rounding variance in HSL conversion
    assert!(result.fg.r == 0);
    assert!(result.fg.g >= 254); // Allow for rounding
    assert!(result.fg.b >= 254);
}
#[test]
fn test_fade_out() {
    let base = Style::fg(Color::WHITE);
    let effect = StyleEffect::FadeOut {
        apply_to: FadeApplyTo::Foreground,
        ease: EasingCurve::Type(EasingType::Linear),
    };
    let result = effect.calculate(0.5, base);
    assert_eq!(result.fg, Color::rgb(127, 127, 127));
}
#[test]
fn test_pulse() {
    let base = Style::fg(Color::RED);
    let target = Color::BLUE;
    let effect = StyleEffect::Pulse {
        frequency: 1.0,
        color: target,
    };
    let result = effect.calculate(0.25, base);
    assert_eq!(result.fg, Color::BLUE);
    let result_trough = effect.calculate(0.75, base);
    assert_eq!(result_trough.fg, Color::RED);
}
#[test]
fn test_glitch_deterministic() {
    let base = Style::default();
    let effect = StyleEffect::Glitch {
        seed: 12345,
        intensity: 1.0,
        italic_start: None,
        italic_end: None,
    };
    // With intensity 1.0, it should always apply a modifier
    let result1 = effect.calculate(0.1, base);
    let result2 = effect.calculate(0.1, base);
    // Determinism check
    assert_eq!(result1, result2);
    // Should have some modifier (4 choices: BOLD, UNDERLINED, ITALIC, REVERSED)
    let mods = result1.mods;
    assert!(mods.bold || mods.underline || mods.italic || mods.reverse);
}
#[test]
fn test_glitch_zero_intensity() {
    let base = Style::default();
    let effect = StyleEffect::Glitch {
        seed: 12345,
        intensity: 0.0,
        italic_start: None,
        italic_end: None,
    };
    let result = effect.calculate(0.5, base);
    assert_eq!(result, base);
}

// <FILE>tui-vfx-style/tests/models/test_cls_style_effect.rs</FILE> - <DESC>Integration tests for StyleEffect</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
