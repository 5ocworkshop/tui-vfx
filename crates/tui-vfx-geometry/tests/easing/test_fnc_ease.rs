// <FILE>tui-vfx-geometry/tests/easing/test_fnc_ease.rs</FILE> - <DESC>Comprehensive tests for easing functions</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>feat-20251224-183752: Easing curves expansion - Phase 1 TDD</WCTX>
// <CLOG>Fixed test_expo_in_acceleration (was incorrectly named test_expo_out_snap_to_finish) with realistic thresholds matching standard Robert Penner formulas</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::easing::{EasingType, ease};

/// All existing easing types (19 variants)
const ALL_EXISTING_EASING_TYPES: &[EasingType] = &[
    EasingType::Linear,
    EasingType::QuadIn,
    EasingType::QuadOut,
    EasingType::QuadInOut,
    EasingType::CubicIn,
    EasingType::CubicOut,
    EasingType::CubicInOut,
    EasingType::SineIn,
    EasingType::SineOut,
    EasingType::SineInOut,
    EasingType::BackIn,
    EasingType::BackOut,
    EasingType::BackInOut,
    EasingType::ElasticIn,
    EasingType::ElasticOut,
    EasingType::ElasticInOut,
    EasingType::BounceIn,
    EasingType::BounceOut,
    EasingType::BounceInOut,
];

// ============================================================================
// BASELINE TESTS: Verify existing 19 variants work correctly
// ============================================================================

#[test]
fn test_all_existing_curves_at_zero() {
    for &easing_type in ALL_EXISTING_EASING_TYPES {
        let result = ease(0.0, easing_type);
        assert!(
            (result - 0.0).abs() < 1e-5,
            "{:?} should be 0.0 at t=0.0, got {}",
            easing_type,
            result
        );
    }
}

#[test]
fn test_all_existing_curves_at_one() {
    for &easing_type in ALL_EXISTING_EASING_TYPES {
        let result = ease(1.0, easing_type);
        assert!(
            (result - 1.0).abs() < 1e-5,
            "{:?} should be 1.0 at t=1.0, got {}",
            easing_type,
            result
        );
    }
}

#[test]
fn test_all_existing_curves_no_nan_or_infinity() {
    for &easing_type in ALL_EXISTING_EASING_TYPES {
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let value = ease(t, easing_type);
            assert!(
                value.is_finite(),
                "{:?} produced non-finite value at t={}: {}",
                easing_type,
                t,
                value
            );
        }
    }
}

#[test]
fn test_linear_identity() {
    assert_relative_eq!(ease(0.0, EasingType::Linear), 0.0);
    assert_relative_eq!(ease(0.5, EasingType::Linear), 0.5);
    assert_relative_eq!(ease(1.0, EasingType::Linear), 1.0);
}

#[test]
fn test_quad_in_out() {
    assert_relative_eq!(ease(0.0, EasingType::QuadInOut), 0.0);
    assert_relative_eq!(ease(0.5, EasingType::QuadInOut), 0.5);
    assert_relative_eq!(ease(1.0, EasingType::QuadInOut), 1.0);
    assert_relative_eq!(ease(0.25, EasingType::QuadInOut), 0.125);
}

#[test]
fn test_cubic_in() {
    assert_relative_eq!(ease(0.0, EasingType::CubicIn), 0.0);
    assert_relative_eq!(ease(1.0, EasingType::CubicIn), 1.0);
    assert_relative_eq!(ease(0.5, EasingType::CubicIn), 0.125);
}

#[test]
fn test_cubic_out() {
    assert_relative_eq!(ease(0.0, EasingType::CubicOut), 0.0);
    assert_relative_eq!(ease(1.0, EasingType::CubicOut), 1.0);
    assert_relative_eq!(ease(0.5, EasingType::CubicOut), 0.875);
}

#[test]
fn test_cubic_in_out() {
    assert_relative_eq!(ease(0.0, EasingType::CubicInOut), 0.0);
    assert_relative_eq!(ease(0.5, EasingType::CubicInOut), 0.5);
    assert_relative_eq!(ease(1.0, EasingType::CubicInOut), 1.0);
    assert_relative_eq!(ease(0.25, EasingType::CubicInOut), 0.0625);
    assert_relative_eq!(ease(0.75, EasingType::CubicInOut), 0.9375);
}

#[test]
fn test_sine_in_smooth_start() {
    let early = ease(0.1, EasingType::SineIn);
    let linear = 0.1;
    assert!(early < linear, "SineIn should start slower than linear");
}

#[test]
fn test_sine_out_smooth_end() {
    let late = ease(0.9, EasingType::SineOut);
    let linear = 0.9;
    assert!(late > linear, "SineOut should end slower than linear");
}

#[test]
fn test_back_in_overshoots_backward() {
    let early = ease(0.2, EasingType::BackIn);
    assert!(early < 0.0, "BackIn should overshoot backward");
}

#[test]
fn test_back_out_overshoots_forward() {
    let late = ease(0.8, EasingType::BackOut);
    assert!(late > 1.0, "BackOut should overshoot forward");
}

// ============================================================================
// NEW TESTS: Exponential & Circular (will FAIL until implementation)
// ============================================================================

#[test]
fn test_expo_in_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::ExpoIn), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(1.0, EasingType::ExpoIn), 1.0, epsilon = 1e-5);
}

#[test]
fn test_expo_out_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::ExpoOut), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(1.0, EasingType::ExpoOut), 1.0, epsilon = 1e-5);
}

#[test]
fn test_expo_inout_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::ExpoInOut), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(0.5, EasingType::ExpoInOut), 0.5, epsilon = 0.05);
    assert_relative_eq!(ease(1.0, EasingType::ExpoInOut), 1.0, epsilon = 1e-5);
}

#[test]
fn test_expo_in_acceleration() {
    // ExpoIn should stay near zero early, then accelerate dramatically (standard Penner formula)
    let very_early = ease(0.2, EasingType::ExpoIn);
    let mid = ease(0.5, EasingType::ExpoIn);
    let late = ease(0.9, EasingType::ExpoIn);

    // Standard ExpoIn: 2^(10*(t-1))
    // t=0.2 → ~0.004, t=0.5 → ~0.031, t=0.9 → 0.5
    assert!(
        very_early < 0.01,
        "ExpoIn should stay very low early (t=0.2)"
    );
    assert!(mid < 0.05, "ExpoIn should still be low at midpoint (t=0.5)");
    assert!(
        late > 0.4 && late < 0.6,
        "ExpoIn should reach ~0.5 at t=0.9"
    );
}

#[test]
fn test_circ_in_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::CircIn), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(1.0, EasingType::CircIn), 1.0, epsilon = 1e-5);
}

#[test]
fn test_circ_out_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::CircOut), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(1.0, EasingType::CircOut), 1.0, epsilon = 1e-5);
}

#[test]
fn test_circ_inout_at_boundaries() {
    assert_relative_eq!(ease(0.0, EasingType::CircInOut), 0.0, epsilon = 1e-5);
    assert_relative_eq!(ease(0.5, EasingType::CircInOut), 0.5, epsilon = 0.05);
    assert_relative_eq!(ease(1.0, EasingType::CircInOut), 1.0, epsilon = 1e-5);
}

#[test]
fn test_circ_inout_continuity_at_midpoint() {
    // Test derivative smoothness at t=0.5 transition point
    const DELTA: f64 = 0.001;
    let delta = DELTA as f32;

    let left_slope =
        (ease(0.5, EasingType::CircInOut) - ease(0.5 - DELTA, EasingType::CircInOut)) / delta;
    let right_slope =
        (ease(0.5 + DELTA, EasingType::CircInOut) - ease(0.5, EasingType::CircInOut)) / delta;

    let derivative_diff = (left_slope - right_slope).abs();
    assert!(
        derivative_diff < 0.1,
        "CircInOut should have continuous derivative at t=0.5, diff={}",
        derivative_diff
    );
}

#[test]
fn test_circ_smoother_than_quad() {
    // Circular should be smoother (less sharp) than quadratic
    let circ_mid = ease(0.5, EasingType::CircOut);
    let quad_mid = ease(0.5, EasingType::QuadOut);

    // CircOut should be between Quad and Linear at midpoint
    assert!(
        circ_mid > quad_mid,
        "CircOut should be smoother than QuadOut"
    );
    assert!(circ_mid < 0.9, "CircOut should not be as fast as Expo");
}

// <FILE>tui-vfx-geometry/tests/easing/test_fnc_ease.rs</FILE> - <DESC>Comprehensive tests for easing functions</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
