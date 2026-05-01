// <FILE>tui-vfx-compositor-next/tests/types/test_filter_spec.rs</FILE> - <DESC>Tests for FilterSpec</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Add greyscale filter for modal backdrop ghost effects</WCTX>
// <CLOG>Added test for FilterSpec::Greyscale serde roundtrip</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor_next::types::{
    AnimatedGlyphRampAffect, AnimatedGlyphRampApplyTo, ApplyTo, FilterSpec,
};
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::{ColorConfig, ColorSpace, Gradient};

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
        sides: vec![],
        dither_amount: 0.0,
        temporal_dither_hz: 0.0,
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

#[test]
fn test_filter_spec_edge_grow_serde_roundtrip() {
    let spec = FilterSpec::EdgeGrow {
        rest_eighths: 2,
        peak_eighths: 14,
        edge: tui_vfx_compositor_next::types::HoverBarPosition::Bottom,
        fill_color: ColorConfig::Rgb {
            r: 255,
            g: 128,
            b: 0,
        },
        bg_color: ColorConfig::Rgb {
            r: 20,
            g: 20,
            b: 25,
        },
        progress: tui_vfx_compositor_next::types::BindableValue::static_f32(0.5),
        margin_width: 3,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_can_build_from_v3_rigid_shake_binding_payload() {
    let spec = FilterSpec::try_from_v3_payload(serde_json::json!({
        "type": "rigid_shake",
        "shake_period": 0.29,
        "num_shakes": { "binding": "error_severity", "default": 4 },
        "pause_duration": 0.52,
        "max_eighths": 12,
        "base_eighths": 3,
        "damping": [1.0, 0.7, 0.45, 0.25],
        "damping_scale": { "binding": "severity_decay", "default": 1.0 }
    }))
    .unwrap();

    match spec {
        FilterSpec::RigidShake {
            num_shakes,
            num_shakes_binding,
            damping_scale_binding,
            ..
        } => {
            assert_eq!(num_shakes, 4);
            assert_eq!(num_shakes_binding.as_deref(), Some("error_severity"));
            assert_eq!(damping_scale_binding.as_deref(), Some("severity_decay"));
        }
        other => panic!("expected RigidShake, got {other:?}"),
    }
}

#[test]
fn test_filter_spec_animated_glyph_ramp_serde_roundtrip() {
    let spec = FilterSpec::AnimatedGlyphRamp {
        glyphs: "AB".into(),
        cycles_per_second: 2.0,
        ease: EasingCurve::Type(EasingType::SineInOut),
        apply_to: AnimatedGlyphRampApplyTo::Both,
        affect: AnimatedGlyphRampAffect::All,
        phase_offset_x_ms: 10.0,
        phase_offset_y_ms: 5.0,
        colors: Some(vec![ColorConfig::Red, ColorConfig::Blue]),
        color_gradient: None,
    };
    spec.validate().unwrap();
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: FilterSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_filter_spec_animated_glyph_ramp_gradient_payload_validates() {
    let spec = FilterSpec::try_from_v3_payload(serde_json::json!({
        "type": "animated_glyph_ramp",
        "glyphs": "ABC",
        "cycles_per_second": 1.25,
        "ease": "SineInOut",
        "color_gradient": {
            "stops": [
                [0.0, { "type": "rgb", "r": 0, "g": 0, "b": 0 }],
                [1.0, { "type": "rgb", "r": 255, "g": 255, "b": 255 }]
            ],
            "space": "rgb"
        }
    }))
    .unwrap();
    spec.validate().unwrap();
    match spec {
        FilterSpec::AnimatedGlyphRamp {
            ease,
            color_gradient: Some(Gradient { space, .. }),
            ..
        } => {
            assert_eq!(ease, EasingCurve::Type(EasingType::SineInOut));
            assert_eq!(space, ColorSpace::Rgb);
        }
        other => panic!("expected AnimatedGlyphRamp, got {other:?}"),
    }
}

#[test]
fn test_filter_spec_animated_glyph_ramp_rejects_invalid_colour_modes() {
    let neither = FilterSpec::AnimatedGlyphRamp {
        glyphs: "AB".into(),
        cycles_per_second: 1.0,
        ease: EasingCurve::default(),
        apply_to: AnimatedGlyphRampApplyTo::default(),
        affect: AnimatedGlyphRampAffect::default(),
        phase_offset_x_ms: 0.0,
        phase_offset_y_ms: 0.0,
        colors: None,
        color_gradient: None,
    };
    assert!(neither.validate().is_err());

    let mismatch = FilterSpec::AnimatedGlyphRamp {
        glyphs: "AB".into(),
        cycles_per_second: 1.0,
        ease: EasingCurve::default(),
        apply_to: AnimatedGlyphRampApplyTo::default(),
        affect: AnimatedGlyphRampAffect::default(),
        phase_offset_x_ms: 0.0,
        phase_offset_y_ms: 0.0,
        colors: Some(vec![ColorConfig::Red]),
        color_gradient: None,
    };
    assert!(mismatch.validate().is_err());
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

// <FILE>tui-vfx-compositor-next/tests/types/test_filter_spec.rs</FILE> - <DESC>Tests for FilterSpec</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
