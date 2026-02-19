// <FILE>tui-vfx-compositor/tests/types/test_filter_spec.rs</FILE> - <DESC>Tests for FilterSpec</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Add greyscale filter for modal backdrop ghost effects</WCTX>
// <CLOG>Added test for FilterSpec::Greyscale serde roundtrip</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::types::{ApplyTo, FilterSpec};
use tui_vfx_style::models::ColorConfig;

#[test]
fn test_filter_spec_default_is_none() {
    let spec = FilterSpec::default();
    assert_eq!(spec, FilterSpec::None);
}

#[test]
fn test_apply_to_serde_roundtrip() {
    for target in [ApplyTo::Foreground, ApplyTo::Background, ApplyTo::Both] {
        let json = serde_json::to_string(&target).unwrap();
        let parsed: ApplyTo = serde_json::from_str(&json).unwrap();
        assert_eq!(target, parsed);
    }
}

#[test]
fn test_filter_spec_dim_serde_roundtrip() {
    let spec = FilterSpec::Dim {
        factor: SignalOrFloat::Static(0.5),
        apply_to: ApplyTo::Both,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_invert_serde_roundtrip() {
    let spec = FilterSpec::Invert {
        apply_to: ApplyTo::Foreground,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_tint_serde_roundtrip() {
    let spec = FilterSpec::Tint {
        color: ColorConfig::Rgb { r: 255, g: 0, b: 0 },
        strength: SignalOrFloat::Static(0.3),
        apply_to: ApplyTo::Background,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_vignette_serde_roundtrip() {
    let spec = FilterSpec::Vignette {
        strength: SignalOrFloat::Static(0.6),
        radius: SignalOrFloat::Static(0.8),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_crt_serde_roundtrip() {
    let spec = FilterSpec::Crt {
        scanline_strength: SignalOrFloat::Static(0.4),
        glow: SignalOrFloat::Static(0.2),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_greyscale_serde_roundtrip() {
    let spec = FilterSpec::Greyscale {
        strength: SignalOrFloat::Static(0.8),
        apply_to: ApplyTo::Both,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_greyscale_fg_only() {
    let spec = FilterSpec::Greyscale {
        strength: SignalOrFloat::Static(1.0),
        apply_to: ApplyTo::Foreground,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// =============================================================================
// ApplyTo PascalCase alias tests
// =============================================================================

#[test]
fn test_apply_to_lowercase_deserialization() {
    // Standard lowercase format
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""fg""#).unwrap(),
        ApplyTo::Foreground
    );
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""bg""#).unwrap(),
        ApplyTo::Background
    );
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""both""#).unwrap(),
        ApplyTo::Both
    );
}

#[test]
fn test_apply_to_pascalcase_alias_deserialization() {
    // PascalCase aliases for consistency with other enums
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""Fg""#).unwrap(),
        ApplyTo::Foreground
    );
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""Bg""#).unwrap(),
        ApplyTo::Background
    );
    assert_eq!(
        serde_json::from_str::<ApplyTo>(r#""Both""#).unwrap(),
        ApplyTo::Both
    );
}

#[test]
fn test_apply_to_serializes_lowercase() {
    // Serialization should use snake_case (the canonical form after V2.2 migration)
    assert_eq!(
        serde_json::to_string(&ApplyTo::Foreground).unwrap(),
        r#""foreground""#
    );
    assert_eq!(
        serde_json::to_string(&ApplyTo::Background).unwrap(),
        r#""background""#
    );
    assert_eq!(serde_json::to_string(&ApplyTo::Both).unwrap(), r#""both""#);
}

// <FILE>tui-vfx-compositor/tests/types/test_filter_spec.rs</FILE> - <DESC>Tests for FilterSpec</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
