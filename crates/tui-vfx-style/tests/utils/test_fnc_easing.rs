// <FILE>tui-vfx-style/tests/utils/test_fnc_easing.rs</FILE> - <DESC>Unit tests for easing functions</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>Easing Curves Expansion: Full Bezier support in tui-style-fx</WCTX>
// <CLOG>Updated tests to use EasingCurve wrapper instead of bare EasingType</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::utils::apply_easing;

#[test]
fn test_linear() {
    assert_eq!(
        apply_easing(0.0, EasingCurve::Type(EasingType::Linear)),
        0.0
    );
    assert_eq!(
        apply_easing(0.5, EasingCurve::Type(EasingType::Linear)),
        0.5
    );
    assert_eq!(
        apply_easing(1.0, EasingCurve::Type(EasingType::Linear)),
        1.0
    );
}

#[test]
fn test_quad_in() {
    // t^2
    assert_eq!(
        apply_easing(0.5, EasingCurve::Type(EasingType::QuadIn)),
        0.25
    );
}

#[test]
fn test_quad_out() {
    // t * (2 - t) -> 0.5 * 1.5 = 0.75
    assert_eq!(
        apply_easing(0.5, EasingCurve::Type(EasingType::QuadOut)),
        0.75
    );
}

#[test]
fn test_cubic_in_out() {
    // < 0.5 -> 4 * t^3
    // 0.25 -> 4 * 0.015625 = 0.0625
    assert_eq!(
        apply_easing(0.25, EasingCurve::Type(EasingType::CubicInOut)),
        0.0625
    );
    // 0.5 -> 4 * 0.125 = 0.5
    assert_eq!(
        apply_easing(0.5, EasingCurve::Type(EasingType::CubicInOut)),
        0.5
    );
}

// <FILE>tui-vfx-style/tests/utils/test_fnc_easing.rs</FILE> - <DESC>Unit tests for easing functions</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
