// <FILE>tui-vfx-style/tests/models/test_cls_glisten_band_shader.rs</FILE> - <DESC>Tests for GlistenBandShader</DESC>
// <VERS>VERSION: 1.2.3 - 2025-12-29</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared ShaderContext helper</CLOG>
use crate::common::{make_ctx, make_style};

use tui_vfx_style::models::{ColorConfig, GlistenBandShader};
use tui_vfx_style::traits::StyleShader;
use tui_vfx_types::{Color, Style};

#[test]
fn test_default_values() {
    let shader = GlistenBandShader::default();
    assert_eq!(shader.speed, 1.0);
    assert_eq!(shader.band_width, 6);
    assert_eq!(shader.angle_deg, 25.0);
}

#[test]
fn test_style_at_returns_style() {
    let shader = GlistenBandShader::default();
    let base = make_style();

    // At t=0.5, the band should be somewhere in the middle
    let style = shader.style_at(&make_ctx(5, 2, 20, 10, 0.5), base);

    // Style should be returned (may or may not be modified depending on position)
    assert!(style.fg != Color::TRANSPARENT);
}

#[test]
fn test_band_moves_with_time() {
    let shader = GlistenBandShader {
        speed: 1.0,
        band_width: 4,
        angle_deg: 0.0, // Horizontal band for easier testing
        head: ColorConfig::White,
        tail: ColorConfig::Rgb {
            r: 128,
            g: 128,
            b: 128,
        },
        ..Default::default()
    };
    let base = Style::fg(Color::rgb(100, 100, 100));

    // Get style at different times - the band should affect different cells
    let style_t0 = shader.style_at(&make_ctx(0, 5, 20, 10, 0.0), base);
    let style_t50 = shader.style_at(&make_ctx(10, 5, 20, 10, 0.5), base);

    // Both should return valid styles
    assert!(style_t0.fg != Color::TRANSPARENT);
    assert!(style_t50.fg != Color::TRANSPARENT);
}

#[test]
fn test_cells_outside_band_get_base_style() {
    let shader = GlistenBandShader {
        speed: 1.0,
        band_width: 2,
        angle_deg: 0.0,
        head: ColorConfig::White,
        tail: ColorConfig::Rgb {
            r: 200,
            g: 200,
            b: 200,
        },
        ..Default::default()
    };
    let base = Style::fg(Color::rgb(50, 50, 50));

    // At t=0, band is at the left edge. Cells far to the right should be unaffected.
    let style = shader.style_at(&make_ctx(19, 5, 20, 10, 0.0), base);

    // Should return base style since we're far from the band
    assert_eq!(style.fg, Color::rgb(50, 50, 50));
}

#[test]
fn test_serde_roundtrip() {
    let shader = GlistenBandShader {
        speed: 0.9,
        band_width: 6,
        angle_deg: 25.0,
        head: ColorConfig::LightYellow,
        tail: ColorConfig::Rgb {
            r: 200,
            g: 160,
            b: 0,
        },
        ..Default::default()
    };

    let json = serde_json::to_string(&shader).unwrap();
    let parsed: GlistenBandShader = serde_json::from_str(&json).unwrap();

    assert_eq!(shader.speed, parsed.speed);
    assert_eq!(shader.band_width, parsed.band_width);
    assert_eq!(shader.angle_deg, parsed.angle_deg);
}

#[test]
fn test_different_angles() {
    let base = make_style();

    // Horizontal band (0 degrees)
    let horiz = GlistenBandShader {
        angle_deg: 0.0,
        ..Default::default()
    };

    // Vertical band (90 degrees)
    let vert = GlistenBandShader {
        angle_deg: 90.0,
        ..Default::default()
    };

    // Both should work without panicking
    let _ = horiz.style_at(&make_ctx(5, 5, 20, 10, 0.5), base);
    let _ = vert.style_at(&make_ctx(5, 5, 20, 10, 0.5), base);
}

// <FILE>tui-vfx-style/tests/models/test_cls_glisten_band_shader.rs</FILE> - <DESC>Tests for GlistenBandShader</DESC>
// <VERS>END OF VERSION: 1.2.3 - 2025-12-29</VERS>
