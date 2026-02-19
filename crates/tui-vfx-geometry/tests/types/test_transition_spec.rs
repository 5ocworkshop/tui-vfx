// <FILE>tui-vfx-geometry/tests/types/test_transition_spec.rs</FILE> - <DESC>Tests for TransitionSpec</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>mixed-signals migration: processing adoption</WCTX>
// <CLOG>Added tests for quantize_steps support</CLOG>

use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::{EasingCurve, PathType, SnappingStrategy, TransitionSpec};

#[test]
fn test_default_values() {
    let spec = TransitionSpec::default();
    assert_eq!(spec.duration_ms, 500);
    assert!(matches!(spec.path, PathType::Linear));
    assert!(matches!(spec.ease, EasingCurve::Type(EasingType::Linear)));
    assert!(matches!(spec.snap, SnappingStrategy::Round));
    assert!(spec.quantize_steps.is_none());
}

#[test]
fn test_serialization() {
    let spec = TransitionSpec {
        duration_ms: 300,
        path: PathType::Arc { bulge: 0.5 },
        ease: EasingCurve::Type(EasingType::QuadInOut),
        snap: SnappingStrategy::Floor,
        quantize_steps: None,
    };
    let json = serde_json::to_string(&spec).expect("Failed to serialize");
    let deserialized: TransitionSpec = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(spec, deserialized);
}

#[test]
fn test_quantize_none() {
    let spec = TransitionSpec::default();
    // Without quantize_steps, should pass through unchanged
    assert_eq!(spec.quantize(0.33), 0.33);
    assert_eq!(spec.quantize(0.5), 0.5);
    assert_eq!(spec.quantize(0.75), 0.75);
}

#[test]
fn test_quantize_8_steps() {
    let spec = TransitionSpec {
        quantize_steps: Some(8),
        ..Default::default()
    };
    // With 8 steps: 0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875
    assert_eq!(spec.quantize(0.0), 0.0);
    assert_eq!(spec.quantize(0.1), 0.0); // floors to 0
    assert_eq!(spec.quantize(0.15), 0.125); // floors to 0.125
    assert_eq!(spec.quantize(0.5), 0.5);
    assert_eq!(spec.quantize(0.99), 0.875); // floors to 0.875
    assert_eq!(spec.quantize(1.0), 1.0); // clamped
}

#[test]
fn test_quantize_4_steps() {
    let spec = TransitionSpec {
        quantize_steps: Some(4),
        ..Default::default()
    };
    // With 4 steps: 0, 0.25, 0.5, 0.75
    assert_eq!(spec.quantize(0.1), 0.0);
    assert_eq!(spec.quantize(0.3), 0.25);
    assert_eq!(spec.quantize(0.6), 0.5);
    assert_eq!(spec.quantize(0.8), 0.75);
}

#[test]
fn test_quantize_serialization() {
    let spec = TransitionSpec {
        duration_ms: 500,
        path: PathType::Linear,
        ease: EasingCurve::Type(EasingType::Linear),
        snap: SnappingStrategy::Round,
        quantize_steps: Some(8),
    };
    let json = serde_json::to_string(&spec).expect("Failed to serialize");
    assert!(json.contains("\"quantize_steps\":8"));
    let deserialized: TransitionSpec = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(spec.quantize_steps, deserialized.quantize_steps);
}

// <FILE>tui-vfx-geometry/tests/types/test_transition_spec.rs</FILE> - <DESC>Tests for TransitionSpec</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
