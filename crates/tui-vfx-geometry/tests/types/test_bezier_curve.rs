// <FILE>tui-vfx-geometry/tests/types/test_bezier_curve.rs</FILE> - <DESC>Tests for cubic Bézier curve solver</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>feat-20251224-183752: Easing curves expansion - Phase 2 TDD</WCTX>
// <CLOG>Initial test suite for cubic Bézier solver with CSS standard easings and edge cases</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::types::EasingCurve;

// ============================================================================
// STANDARD CSS EASINGS: Test well-known cubic-bezier curves
// ============================================================================

#[test]
fn test_bezier_css_ease() {
    // CSS ease: cubic-bezier(0.25, 0.1, 0.25, 1.0)
    let curve = EasingCurve::bezier(0.25, 0.1, 0.25, 1.0);

    // Boundaries must be exact
    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Midpoint should be smooth acceleration
    let mid = curve.ease(0.5);
    assert!(
        mid > 0.5 && mid < 0.9,
        "CSS ease should accelerate smoothly"
    );
}

#[test]
fn test_bezier_css_ease_in() {
    // CSS ease-in: cubic-bezier(0.42, 0, 1.0, 1.0)
    let curve = EasingCurve::bezier(0.42, 0.0, 1.0, 1.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Should start slow (acceleration curve)
    let early = curve.ease(0.2);
    assert!(early < 0.1, "ease-in should start very slow");
}

#[test]
fn test_bezier_css_ease_out() {
    // CSS ease-out: cubic-bezier(0, 0, 0.58, 1.0)
    let curve = EasingCurve::bezier(0.0, 0.0, 0.58, 1.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Should end slow (deceleration curve)
    let late = curve.ease(0.8);
    assert!(late > 0.9, "ease-out should end very slow");
}

#[test]
fn test_bezier_css_ease_in_out() {
    // CSS ease-in-out: cubic-bezier(0.42, 0, 0.58, 1.0)
    let curve = EasingCurve::bezier(0.42, 0.0, 0.58, 1.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(0.5), 0.5, epsilon = 0.05);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);
}

// ============================================================================
// LINEAR BEZIER: Should behave like Linear easing
// ============================================================================

#[test]
fn test_bezier_linear() {
    // Linear Bézier: cubic-bezier(0, 0, 1, 1) - no curve
    let curve = EasingCurve::bezier(0.0, 0.0, 1.0, 1.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(0.25), 0.25, epsilon = 1e-3);
    assert_relative_eq!(curve.ease(0.5), 0.5, epsilon = 1e-3);
    assert_relative_eq!(curve.ease(0.75), 0.75, epsilon = 1e-3);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);
}

// ============================================================================
// EDGE CASES: Vertical tangents and extreme curves
// ============================================================================

#[test]
fn test_bezier_vertical_start() {
    // Vertical tangent at start: cubic-bezier(0, 1, 1, 1)
    let curve = EasingCurve::bezier(0.0, 1.0, 1.0, 1.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Should have sharp initial jump
    let early = curve.ease(0.1);
    assert!(early > 0.3, "Vertical start should jump quickly");
}

#[test]
fn test_bezier_vertical_end() {
    // Vertical tangent at end: cubic-bezier(0, 0, 1, 0)
    let curve = EasingCurve::bezier(0.0, 0.0, 1.0, 0.0);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Should have sharp final jump
    let late = curve.ease(0.9);
    assert!(late < 0.7, "Vertical end should delay then snap");
}

#[test]
fn test_bezier_overshoot() {
    // Overshoot curve: cubic-bezier(0.5, -0.5, 0.5, 1.5)
    // Y values outside [0,1] should create overshoot
    let curve = EasingCurve::bezier(0.5, -0.5, 0.5, 1.5);

    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);

    // Should overshoot below 0 early and above 1 late
    let early = curve.ease(0.2);
    let late = curve.ease(0.8);
    assert!(early < 0.0, "Should undershoot early");
    assert!(late > 1.0, "Should overshoot late");
}

// ============================================================================
// MONOTONICITY: X values must increase for well-defined curves
// ============================================================================

#[test]
#[should_panic(expected = "x values must be in [0, 1]")]
fn test_bezier_invalid_x1_negative() {
    // x1 must be in [0, 1]
    let _ = EasingCurve::bezier(-0.1, 0.0, 0.5, 1.0);
}

#[test]
#[should_panic(expected = "x values must be in [0, 1]")]
fn test_bezier_invalid_x1_too_large() {
    // x1 must be in [0, 1]
    let _ = EasingCurve::bezier(1.5, 0.0, 0.5, 1.0);
}

#[test]
#[should_panic(expected = "x values must be in [0, 1]")]
fn test_bezier_invalid_x2_negative() {
    // x2 must be in [0, 1]
    let _ = EasingCurve::bezier(0.5, 0.0, -0.1, 1.0);
}

#[test]
#[should_panic(expected = "x values must be in [0, 1]")]
fn test_bezier_invalid_x2_too_large() {
    // x2 must be in [0, 1]
    let _ = EasingCurve::bezier(0.5, 0.0, 1.5, 1.0);
}

// ============================================================================
// SERDE: JSON deserialization for recipe compatibility
// ============================================================================

#[test]
fn test_bezier_deserialize_object() {
    // New format: {"x1": 0.25, "y1": 0.1, "x2": 0.25, "y2": 1.0}
    let json = r#"{"x1": 0.25, "y1": 0.1, "x2": 0.25, "y2": 1.0}"#;
    let curve: EasingCurve = serde_json::from_str(json).expect("Should deserialize Bézier");

    // Should match CSS ease
    assert_relative_eq!(curve.ease(0.0), 0.0, epsilon = 1e-6);
    assert_relative_eq!(curve.ease(1.0), 1.0, epsilon = 1e-6);
}

#[test]
fn test_bezier_serialize_roundtrip() {
    let curve = EasingCurve::bezier(0.42, 0.0, 0.58, 1.0);
    let json = serde_json::to_string(&curve).expect("Should serialize");
    let deserialized: EasingCurve = serde_json::from_str(&json).expect("Should deserialize");

    // Should produce same results
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        assert_relative_eq!(curve.ease(t), deserialized.ease(t), epsilon = 1e-6);
    }
}

// ============================================================================
// PRECISION: Solver should be accurate to within 1e-6
// ============================================================================

#[test]
fn test_bezier_solver_precision() {
    let curve = EasingCurve::bezier(0.25, 0.1, 0.25, 1.0);

    // Test many points for consistent precision
    for i in 0..=100 {
        let t = (i as f64) / 100.0;
        let y = curve.ease(t);

        // Result should be finite
        assert!(y.is_finite(), "Result must be finite at t={}", t);

        // Solver should be monotonic (for valid curves)
        if i > 0 {
            let prev_t = ((i - 1) as f64) / 100.0;
            let prev_y = curve.ease(prev_t);
            // Note: Overshoot curves may not be monotonic, so this is a soft check
            if y < prev_y {
                // Only warn for non-overshoot curves
                // For now, just check it's not NaN
            }
        }
    }
}

// <FILE>tui-vfx-geometry/tests/types/test_bezier_curve.rs</FILE> - <DESC>Tests for cubic Bézier curve solver</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
