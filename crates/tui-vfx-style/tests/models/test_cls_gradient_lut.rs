// <FILE>tui-vfx-style/tests/models/test_cls_gradient_lut.rs</FILE> - <DESC>Tests for GradientLUT</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing btop-inspired features</WCTX>
// <CLOG>Initial tests for GradientLUT</CLOG>

use tui_vfx_style::models::GradientLUT;
use tui_vfx_types::Color;

#[test]
fn test_2_point_gradient_start() {
    let lut = GradientLUT::new_2_point(Color::BLACK, Color::WHITE);
    assert_eq!(lut.get(0), Color::BLACK);
}

#[test]
fn test_2_point_gradient_end() {
    let lut = GradientLUT::new_2_point(Color::BLACK, Color::WHITE);
    assert_eq!(lut.get(100), Color::WHITE);
}

#[test]
fn test_2_point_gradient_middle() {
    let lut = GradientLUT::new_2_point(Color::rgb(0, 0, 0), Color::rgb(100, 100, 100));
    let mid = lut.get(50);
    // Should be approximately (50, 50, 50)
    assert!((mid.r as i32 - 50).abs() <= 1, "r={}", mid.r);
    assert!((mid.g as i32 - 50).abs() <= 1, "g={}", mid.g);
    assert!((mid.b as i32 - 50).abs() <= 1, "b={}", mid.b);
}

#[test]
fn test_3_point_gradient_start() {
    let lut = GradientLUT::new_3_point(
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(0, 255, 0),   // Green
    );
    assert_eq!(lut.get(0), Color::rgb(255, 0, 0));
}

#[test]
fn test_3_point_gradient_mid() {
    let lut = GradientLUT::new_3_point(
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(0, 255, 0),   // Green
    );
    assert_eq!(lut.get(50), Color::rgb(255, 255, 0));
}

#[test]
fn test_3_point_gradient_end() {
    let lut = GradientLUT::new_3_point(
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(0, 255, 0),   // Green
    );
    assert_eq!(lut.get(100), Color::rgb(0, 255, 0));
}

#[test]
fn test_3_point_gradient_quarter() {
    let lut = GradientLUT::new_3_point(
        Color::rgb(0, 0, 0),
        Color::rgb(100, 100, 100),
        Color::rgb(200, 200, 200),
    );
    // At 25%, should be between start (0,0,0) and mid (100,100,100)
    // Normalized within first half: 25/50 = 0.5 of first segment
    let quarter = lut.get(25);
    assert!((quarter.r as i32 - 50).abs() <= 1, "r={}", quarter.r);
    assert!((quarter.g as i32 - 50).abs() <= 1, "g={}", quarter.g);
    assert!((quarter.b as i32 - 50).abs() <= 1, "b={}", quarter.b);
}

#[test]
fn test_clamping_above_100() {
    let lut = GradientLUT::new_2_point(Color::BLACK, Color::WHITE);
    // Should clamp to 100
    assert_eq!(lut.get(150), Color::WHITE);
    assert_eq!(lut.get(255), Color::WHITE);
}

#[test]
fn test_non_rgb_colors_fallback() {
    // Using named colors that can be converted to RGB
    let lut = GradientLUT::new_2_point(Color::RED, Color::BLUE);
    // Should work - Red and Blue have known RGB mappings
    let _ = lut.get(50);
}

#[test]
fn test_gradient_lut_all_values() {
    let lut = GradientLUT::new_2_point(Color::rgb(0, 0, 0), Color::rgb(100, 0, 0));
    // Verify monotonic increase in red channel
    let mut prev_r = 0u8;
    for i in 0..=100 {
        let color = lut.get(i);
        assert!(
            color.r >= prev_r,
            "Red channel should increase monotonically at {}",
            i
        );
        prev_r = color.r;
    }
}

// <FILE>tui-vfx-style/tests/models/test_cls_gradient_lut.rs</FILE> - <DESC>Tests for GradientLUT</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
