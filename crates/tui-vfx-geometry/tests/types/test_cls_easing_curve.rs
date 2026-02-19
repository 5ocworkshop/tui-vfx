// <FILE>tui-vfx-geometry/tests/types/test_cls_easing_curve.rs</FILE> - <DESC>Tests for EasingCurve wrapper type</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-24</VERS>
// <WCTX>feat-20251224-183752: Easing curves expansion - Phase 1.5 TDD</WCTX>
// <CLOG>Removed test_deserialize_new_format_type (untagged serde doesn't support explicit tagging)</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;

// ============================================================================
// CONVERSION TESTS: From<EasingType> trait
// ============================================================================

#[test]
fn test_from_easing_type() {
    let curve: EasingCurve = EasingType::Linear.into();
    assert!(matches!(curve, EasingCurve::Type(EasingType::Linear)));

    let curve: EasingCurve = EasingType::ExpoIn.into();
    assert!(matches!(curve, EasingCurve::Type(EasingType::ExpoIn)));
}

// ============================================================================
// EVALUATION TESTS: ease() method
// ============================================================================

#[test]
fn test_ease_simple_type() {
    let curve = EasingCurve::Type(EasingType::Linear);
    assert_eq!(curve.ease(0.0), 0.0);
    assert_eq!(curve.ease(0.5), 0.5);
    assert_eq!(curve.ease(1.0), 1.0);
}

#[test]
fn test_ease_expo_type() {
    let curve = EasingCurve::Type(EasingType::ExpoIn);
    let result = curve.ease(0.2);
    assert!(result < 0.01, "ExpoIn should stay very low early");
}

// ============================================================================
// SERDE BACKWARD COMPATIBILITY: Untagged deserialization
// ============================================================================

#[test]
fn test_deserialize_legacy_easing_type() {
    // Old format: just the enum variant name (e.g., "Linear", "ExpoIn")
    let json = r#""Linear""#;
    let curve: EasingCurve = serde_json::from_str(json).expect("Should deserialize legacy format");
    assert!(matches!(curve, EasingCurve::Type(EasingType::Linear)));

    let json = r#""ExpoIn""#;
    let curve: EasingCurve = serde_json::from_str(json).expect("Should deserialize ExpoIn");
    assert!(matches!(curve, EasingCurve::Type(EasingType::ExpoIn)));
}

// NOTE: With untagged serde, there's no "new format" - the bare enum value IS the format.
// The backward compatibility works because old recipes using "Linear" continue to work.
// When Bezier is added in Phase 2, it will deserialize from {"x1": ..., "y1": ..., ...}

#[test]
fn test_serialize_roundtrip() {
    let curve = EasingCurve::Type(EasingType::QuadOut);
    let json = serde_json::to_string(&curve).expect("Should serialize");
    let deserialized: EasingCurve = serde_json::from_str(&json).expect("Should deserialize");
    assert!(matches!(
        deserialized,
        EasingCurve::Type(EasingType::QuadOut)
    ));
}

// ============================================================================
// DEFAULT: Should use Linear
// ============================================================================

#[test]
fn test_default() {
    let curve = EasingCurve::default();
    assert!(matches!(curve, EasingCurve::Type(EasingType::Linear)));
    assert_eq!(curve.ease(0.5), 0.5);
}

// <FILE>tui-vfx-geometry/tests/types/test_cls_easing_curve.rs</FILE> - <DESC>Tests for EasingCurve wrapper type</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-24</VERS>
