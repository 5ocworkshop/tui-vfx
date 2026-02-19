// <FILE>tui-vfx-style/tests/models/test_cls_fade_to_black.rs</FILE>
// <DESC>Tests for fade-to-black interpolation and fade_effect combinator</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>mixed-types migration</WCTX>
// <CLOG>Updated to use tui_vfx_types Color/Style API</CLOG>

use tui_vfx_types::{Color, Style};

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::{FadeApplyTo, FadeDirection, FadeToBlack, fade_effect};
use tui_vfx_style::traits::StyleInterpolator;

#[test]
fn test_fade_to_black_interpolates_foreground() {
    let fade = FadeToBlack::fade_out();
    let base = Style::fg(Color::WHITE);

    let result = fade.calculate(0.5, base);

    assert_eq!(result.fg, Color::rgb(127, 127, 127));
}

#[test]
fn test_fade_to_black_apply_to_foreground_only() {
    let fade = FadeToBlack::fade_out().with_apply_to(FadeApplyTo::Foreground);
    let base = Style::fg(Color::WHITE).with_bg(Color::RED);

    let result = fade.calculate(0.5, base);

    assert_eq!(result.fg, Color::rgb(127, 127, 127));
    assert_eq!(result.bg, Color::RED);
}

#[test]
fn test_fade_effect_combinator_fades_inner_not_base() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Inner;

    impl StyleInterpolator for Inner {
        fn calculate(&self, _t: f64, base: Style) -> Style {
            // Ignore base fg and force Green.
            base.with_fg(Color::GREEN)
        }
    }

    let fade = FadeToBlack::fade_out();
    let composed = fade_effect(Inner, fade);

    let base = Style::fg(Color::RED);
    let result = composed.calculate(0.5, base);

    // Green (0,255,0), halfway to black yields (0,127,0).
    assert_eq!(result.fg, Color::rgb(0, 127, 0));
}

// =============================================================================
// Apply_to Variant Tests
// =============================================================================

#[test]
fn test_fade_out_apply_to_background_only() {
    let fade = FadeToBlack::fade_out().with_apply_to(FadeApplyTo::Background);
    let base = Style::fg(Color::WHITE).with_bg(Color::BLUE);

    let result = fade.calculate(0.5, base);

    // Foreground should remain unchanged
    assert_eq!(result.fg, Color::WHITE);
    // Background should fade: Blue (0,0,255) -> halfway to black = (0,0,127)
    let bg = result.bg;
    assert_eq!(bg.r, 0);
    assert_eq!(bg.g, 0);
    // Should be approximately 127
    assert!(
        bg.b > 0 && bg.b < 255,
        "Expected mid-range blue, got {}",
        bg.b
    );
}

#[test]
fn test_fade_out_apply_to_both() {
    let fade = FadeToBlack::fade_out().with_apply_to(FadeApplyTo::Both);
    let base = Style::fg(Color::WHITE).with_bg(Color::RED);

    let result = fade.calculate(0.5, base);

    // Both foreground and background should fade halfway towards black
    let fg = result.fg;
    assert!(fg.r > 0 && fg.r < 255);
    assert!(fg.g > 0 && fg.g < 255);
    assert!(fg.b > 0 && fg.b < 255);

    let bg = result.bg;
    assert!(bg.r > 0 && bg.r < 255); // Red channel should be between 0 and 255
    assert_eq!(bg.g, 0); // Green should remain 0
    assert_eq!(bg.b, 0); // Blue should remain 0
}

#[test]
fn test_fade_in_apply_to_foreground_only() {
    let fade = FadeToBlack::fade_in().with_apply_to(FadeApplyTo::Foreground);
    let base = Style::fg(Color::WHITE).with_bg(Color::GREEN);

    let result = fade.calculate(0.5, base);

    // Foreground should fade in: halfway from black to white
    assert_eq!(result.fg, Color::rgb(127, 127, 127));
    // Background should remain unchanged
    assert_eq!(result.bg, Color::GREEN);
}

#[test]
fn test_fade_in_direction() {
    let fade = FadeToBlack {
        direction: FadeDirection::In,
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::Linear),
    };
    let base = Style::fg(Color::WHITE);

    // At t=0.0, fade in should show black (start of fade in)
    let result_start = fade.calculate(0.0, base);
    // darken() returns Color with RGB values
    assert_eq!(result_start.fg.r, 0);
    assert_eq!(result_start.fg.g, 0);
    assert_eq!(result_start.fg.b, 0);

    // At t=1.0, fade in should show full color (end of fade in)
    let result_end = fade.calculate(1.0, base);
    assert_eq!(result_end.fg, Color::WHITE);
}

#[test]
fn test_fade_out_direction() {
    let fade = FadeToBlack {
        direction: FadeDirection::Out,
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::Linear),
    };
    let base = Style::fg(Color::WHITE);

    // At t=0.0, fade out should show full color (start of fade out)
    let result_start = fade.calculate(0.0, base);
    assert_eq!(result_start.fg, Color::WHITE);

    // At t=1.0, fade out should show black (end of fade out)
    let result_end = fade.calculate(1.0, base);
    // darken() returns Color with RGB values
    assert_eq!(result_end.fg.r, 0);
    assert_eq!(result_end.fg.g, 0);
    assert_eq!(result_end.fg.b, 0);
}

// =============================================================================
// Easing Curve Tests
// =============================================================================

#[test]
fn test_fade_with_quad_in() {
    let fade = FadeToBlack {
        direction: FadeDirection::Out,
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::QuadIn),
    };
    let base = Style::fg(Color::WHITE);

    // With QuadIn, t=0.5 should ease to approximately 0.25
    // So fade amount = 0.25, color should be closer to white than black
    let result = fade.calculate(0.5, base);

    // White (255,255,255) with 0.25 fade = (191, 191, 191)
    // The exact value depends on the easing function implementation
    let color = result.fg;
    // Should be brighter than linear (127) but darker than white (255)
    assert!(color.r > 127 && color.r < 255, "r={}", color.r);
    assert!(color.g > 127 && color.g < 255, "g={}", color.g);
    assert!(color.b > 127 && color.b < 255, "b={}", color.b);
}

#[test]
fn test_fade_with_cubic_out() {
    let fade = FadeToBlack {
        direction: FadeDirection::In,
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Type(EasingType::CubicOut),
    };
    let base = Style::fg(Color::WHITE);

    // With CubicOut, t=0.5 should ease to approximately 0.875
    // For fade in, amount = 1.0 - eased_t = 0.125
    let result = fade.calculate(0.5, base);

    let color = result.fg;
    // Should be brighter than linear (127) due to ease out
    assert!(color.r > 127, "r={}", color.r);
    assert!(color.g > 127, "g={}", color.g);
    assert!(color.b > 127, "b={}", color.b);
}

#[test]
fn test_fade_with_bezier_curve() {
    // Custom Bezier curve
    let fade = FadeToBlack {
        direction: FadeDirection::Out,
        apply_to: FadeApplyTo::Both,
        ease: EasingCurve::Bezier {
            x1: 0.42,
            y1: 0.0,
            x2: 0.58,
            y2: 1.0,
        },
    };
    let base = Style::fg(Color::WHITE);

    let result = fade.calculate(0.5, base);
    let fg = result.fg;
    assert_eq!(fg.r, fg.g);
    assert_eq!(fg.g, fg.b);
    assert!(
        fg.r > 0 && fg.r < 255,
        "expected mid-tone fade, got r={}",
        fg.r
    );
}

// <FILE>tui-vfx-style/tests/models/test_cls_fade_to_black.rs</FILE>
// <DESC>Tests for fade-to-black interpolation and fade_effect combinator</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
