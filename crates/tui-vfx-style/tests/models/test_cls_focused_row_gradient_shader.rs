// <FILE>tui-vfx-style/tests/models/test_cls_focused_row_gradient_shader.rs</FILE> - <DESC>Tests for FocusedRowGradient shader</DESC>
// <VERS>VERSION: 1.3.1</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared ShaderContext helper</CLOG>
use crate::common::make_ctx;

use tui_vfx_style::models::{ApplyToColor, ColorConfig, FocusedRowGradientShader};
use tui_vfx_style::traits::StyleShader;
use tui_vfx_types::{Color, Style};

#[test]
fn test_selected_row_is_bright_color() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 5,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;
    let selected_y = 5; // round((10-1) * 0.5) = round(4.5) = 5

    let result = shader.style_at(&make_ctx(0, selected_y, 20, height, 0.0), base);
    assert_eq!(result.fg, Color::WHITE);
}

#[test]
fn test_max_distance_is_dim_color() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 5,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Row 0 is 5 rows from selected (row 5), which equals falloff_distance
    let result = shader.style_at(&make_ctx(0, 0, 20, height, 0.0), base);
    assert_eq!(result.fg, Color::BLACK);
}

#[test]
fn test_gradient_interpolation() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 4,
        bright_color: ColorConfig::Rgb {
            r: 200,
            g: 200,
            b: 200,
        },
        dim_color: ColorConfig::Rgb { r: 0, g: 0, b: 0 },
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;
    let _selected_y = 5;

    // Row 3 is 2 rows from selected (distance = 2), normalized = 2/4 = 0.5
    // Should be approximately (100, 100, 100)
    let result = shader.style_at(&make_ctx(0, 3, 20, height, 0.0), base);
    let fg = result.fg;
    assert!(
        (fg.r as i32 - 100).abs() <= 5,
        "Red channel should be ~100, got {}",
        fg.r
    );
    assert!(
        (fg.g as i32 - 100).abs() <= 5,
        "Green channel should be ~100, got {}",
        fg.g
    );
    assert!(
        (fg.b as i32 - 100).abs() <= 5,
        "Blue channel should be ~100, got {}",
        fg.b
    );
}

#[test]
fn test_symmetrical_above_and_below() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 5,
        bright_color: ColorConfig::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
        dim_color: ColorConfig::Rgb { r: 0, g: 0, b: 0 },
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 20;
    let selected_y = 10;

    // Distance 3 above and below should produce same color
    let above = shader.style_at(&make_ctx(0, selected_y - 3, 20, height, 0.0), base);
    let below = shader.style_at(&make_ctx(0, selected_y + 3, 20, height, 0.0), base);

    assert_eq!(
        above.fg, below.fg,
        "Color should be symmetrical above/below"
    );
}

#[test]
fn test_apply_to_background() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 5,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Background,
    };

    let base = Style::bg(Color::gray(128));
    let height = 10;
    let selected_y = 5;

    let result = shader.style_at(&make_ctx(0, selected_y, 20, height, 0.0), base);
    assert_eq!(result.bg, Color::WHITE);
    // Foreground should remain transparent (default)
    assert_eq!(result.fg, Color::TRANSPARENT);
}

#[test]
fn test_apply_to_both() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 5,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Both,
    };

    let base = Style::fg(Color::gray(128)).with_bg(Color::gray(64));
    let height = 10;
    let selected_y = 5;

    let result = shader.style_at(&make_ctx(0, selected_y, 20, height, 0.0), base);
    assert_eq!(result.fg, Color::WHITE);
    assert_eq!(result.bg, Color::WHITE);
}

#[test]
fn test_zero_height_graceful() {
    let shader = FocusedRowGradientShader::default();
    let base = Style::fg(Color::gray(128));

    // Should not panic with zero height
    let result = shader.style_at(&make_ctx(0, 0, 20, 0, 0.0), base);
    assert!(result.fg != Color::TRANSPARENT);
}

#[test]
fn test_zero_falloff_uses_dim_immediately() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.5,
        falloff_distance: 0,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Any row not exactly at selected should be dim
    let result = shader.style_at(&make_ctx(0, 4, 20, height, 0.0), base);
    assert_eq!(result.fg, Color::BLACK);

    // Selected row should still be bright
    let selected = shader.style_at(&make_ctx(0, 5, 20, height, 0.0), base);
    assert_eq!(selected.fg, Color::WHITE);
}

#[test]
fn test_top_row_selection() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 0.0, // Top
        falloff_distance: 3,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Row 0 should be brightest
    let top = shader.style_at(&make_ctx(0, 0, 20, height, 0.0), base);
    assert_eq!(top.fg, Color::WHITE);
}

#[test]
fn test_bottom_row_selection() {
    let shader = FocusedRowGradientShader {
        selected_row: None,
        selected_row_ratio: 1.0, // Bottom
        falloff_distance: 3,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Row 9 (last) should be brightest
    let bottom = shader.style_at(&make_ctx(0, 9, 20, height, 0.0), base);
    assert_eq!(bottom.fg, Color::WHITE);
}

#[test]
fn test_selected_row_takes_precedence() {
    let shader = FocusedRowGradientShader {
        selected_row: Some(3),   // Explicitly select row 3
        selected_row_ratio: 0.9, // This should be ignored
        falloff_distance: 5,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Row 3 should be brightest (selected_row takes precedence over ratio)
    let at_selected = shader.style_at(&make_ctx(0, 3, 20, height, 0.0), base);
    assert_eq!(at_selected.fg, Color::WHITE);

    // Row 9 should NOT be bright (even though ratio 0.9 would suggest it)
    let at_ratio = shader.style_at(&make_ctx(0, 9, 20, height, 0.0), base);
    // Distance from row 9 to row 3 is 6, which exceeds falloff_distance of 5
    assert_eq!(at_ratio.fg, Color::BLACK);
}

#[test]
fn test_selected_row_clamped_to_bounds() {
    let shader = FocusedRowGradientShader {
        selected_row: Some(100), // Way out of bounds
        selected_row_ratio: 0.5,
        falloff_distance: 3,
        bright_color: ColorConfig::White,
        dim_color: ColorConfig::Black,
        apply_to: ApplyToColor::Foreground,
    };

    let base = Style::fg(Color::gray(128));
    let height = 10;

    // Row 9 (last valid row) should be brightest since 100 clamps to 9
    let last_row = shader.style_at(&make_ctx(0, 9, 20, height, 0.0), base);
    assert_eq!(last_row.fg, Color::WHITE);
}

// <FILE>tui-vfx-style/tests/models/test_cls_focused_row_gradient_shader.rs</FILE> - <DESC>Tests for FocusedRowGradient shader</DESC>
// <VERS>END OF VERSION: 1.3.1</VERS>
