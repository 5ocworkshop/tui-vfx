// <FILE>tui-vfx-style/tests/models/test_cls_spatial_shader_type.rs</FILE> - <DESC>Focused V3 payload-constructor coverage for SpatialShaderType</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Push representative V3-authored shader payload normalization closer to executable shader semantics instead of leaving it in recipes-only bridge code.</WCTX>
// <CLOG>0.1.0: add focused coverage for gradient_overlay, colored_overlay pattern lowering, and runtime-binding extraction through SpatialShaderType::try_from_v3_payload.</CLOG>

use tui_vfx_style::models::{
    AmbientOcclusionShader, ConcealedLightMode, DiffusionMode, FireMode, HighlighterDirection,
    LinearGradientShader, SpatialShaderType, TerminalFireShader, TerminalWaterShader,
    VfxMotionFieldBehavior, VfxSpatialPrimitive, VfxSpatialShaderFamily,
};

#[test]
fn spatial_shader_type_can_build_from_v3_gradient_overlay_payload() {
    let shader = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "gradient_overlay",
        "gradient": {
            "stops": [
                [0.0, {"type":"black"}],
                [1.0, {"type":"white"}]
            ],
            "space": "rgb"
        },
        "angle_deg": 45.0,
        "intensity": 1.0
    }))
    .unwrap();

    match shader {
        SpatialShaderType::LinearGradient(LinearGradientShader { angle_deg, .. }) => {
            assert_eq!(angle_deg, 45.0);
        }
        other => panic!("expected LinearGradient, got {other:?}"),
    }
}

#[test]
fn spatial_shader_type_can_build_from_v3_colored_overlay_pattern_payload() {
    let shader = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "colored_overlay",
        "pattern": {
            "kind": "edge_shadow",
            "edges": ["bottom", "right"],
            "radius": 3,
            "falloff": "quadratic"
        },
        "color": { "type": "rgb", "r": 0, "g": 0, "b": 0 },
        "intensity": 0.7
    }))
    .unwrap();

    match shader {
        SpatialShaderType::AmbientOcclusion(AmbientOcclusionShader { radius, .. }) => {
            assert_eq!(radius, 3);
        }
        other => panic!("expected AmbientOcclusion, got {other:?}"),
    }
}

#[test]
fn spatial_shader_type_can_build_from_v3_binding_and_signal_payloads() {
    let highlighter = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "highlighter",
        "color": { "type": "yellow" },
        "speed": { "binding": "rate", "default": 1.0 },
        "blend_strength": { "binding": "blend", "default": 0.7 },
        "direction": { "binding": "dir", "default": "reverse" }
    }))
    .unwrap();
    let concealed = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "concealed_light",
        "color": { "type": "rgb", "r": 196, "g": 228, "b": 255 },
        "source": "left",
        "spread": {
            "signal": {
                "kind": "sine",
                "clock_ref": "config.clock",
                "amplitude": 0.6,
                "offset": 5.0
            }
        },
        "edge_width": 1,
        "source_cutoff": 0.16,
        "intensity": 0.24
    }))
    .unwrap();
    let diffusion = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "colored_overlay",
        "pattern": {
            "kind": "radial_from_corner",
            "source": "top_left",
            "radius": 8,
            "softness": 0.85,
            "edge_firmness": 0.25
        },
        "color": { "type": "rgb", "r": 242, "g": 222, "b": 196 },
        "intensity": {
            "signal": {
                "kind": "sine",
                "clock_ref": "config.clock",
                "amplitude": 0.05,
                "offset": 0.24
            }
        }
    }))
    .unwrap();

    match highlighter {
        SpatialShaderType::Highlighter(shader) => {
            assert_eq!(shader.speed_binding.as_deref(), Some("rate"));
            assert_eq!(shader.blend_strength_binding.as_deref(), Some("blend"));
            assert_eq!(shader.direction_binding.as_deref(), Some("dir"));
            assert_eq!(shader.direction, HighlighterDirection::Reverse);
        }
        other => panic!("expected Highlighter, got {other:?}"),
    }

    match concealed {
        SpatialShaderType::ConcealedLight(shader) => {
            assert_eq!(shader.spread, 5);
            assert_eq!(shader.mode, ConcealedLightMode::Drift);
            assert_eq!(shader.pulse_speed, 1.0);
        }
        other => panic!("expected ConcealedLight, got {other:?}"),
    }

    match diffusion {
        SpatialShaderType::Diffusion(shader) => {
            assert_eq!(shader.mode, DiffusionMode::Breath);
            assert_eq!(shader.drift_speed, 1.0);
            assert_eq!(shader.drift_amount, 0.2);
        }
        other => panic!("expected Diffusion, got {other:?}"),
    }
}

#[test]
fn diffusion_key_parameters_format_signal_intensity_for_authors() {
    let shader = SpatialShaderType::try_from_v3_payload(serde_json::json!({
        "type": "diffusion",
        "source": "right",
        "color": { "type": "white" },
        "radius": 4,
        "intensity": { "type": "sample_norm_x" }
    }))
    .unwrap();

    let params = shader.key_parameters();
    let intensity = params
        .iter()
        .find_map(|(name, value)| (*name == "intensity").then_some(value.as_str()))
        .expect("diffusion intensity key parameter");
    assert_eq!(intensity, "signal(sample_norm_x)");
}

#[test]
fn spatial_shader_type_exposes_grouped_v3_family_seam() {
    let primitive = SpatialShaderType::Glow(Default::default());
    let composed = SpatialShaderType::BorderSweep(Default::default());

    assert_eq!(primitive.v3_family_label(), "surface_depth");
    assert_eq!(composed.v3_family_label(), "traveling_band");
    assert!(matches!(
        primitive.v3_spatial_shader_family(),
        VfxSpatialShaderFamily::Primitive(_)
    ));
    assert!(matches!(
        composed.v3_spatial_shader_family(),
        VfxSpatialShaderFamily::ComposedPrimitive(_)
    ));
}

#[test]
fn terminal_water_deserializes_and_reports_metadata() {
    let shader: SpatialShaderType = serde_json::from_value(serde_json::json!({
        "type": "terminal_water",
        "mode": { "mode": "ocean" },
        "layers": 2,
        "amplitude": 0.4,
        "wavelength": 10.0,
        "speed": 1.2,
        "direction_deg": 35.0,
        "steepness": 0.5,
        "normal_strength": 1.2,
        "diffuse": 0.7,
        "specular": 0.4,
        "shininess": 18.0,
        "fresnel": 0.3,
        "foam": 0.6,
        "deep_color": { "type": "rgb", "r": 5, "g": 32, "b": 64 },
        "shallow_color": { "type": "rgb", "r": 40, "g": 170, "b": 210 },
        "foam_color": { "type": "white" },
        "glint_strength": 0.2,
        "glint_angle_deg": -18.0,
        "glint_width": 8.0,
        "glint_speed": 1.0,
        "apply_to": "both"
    }))
    .unwrap();

    assert_eq!(shader.name(), "TerminalWater");
    assert!(shader.terse_description().contains("water"));
    let keys: Vec<_> = shader
        .key_parameters()
        .into_iter()
        .map(|(key, _)| key)
        .collect();
    assert!(keys.contains(&"mode"));
    assert!(keys.contains(&"glint"));
}

#[test]
fn terminal_water_maps_to_motion_field_family() {
    let shader = SpatialShaderType::TerminalWater(TerminalWaterShader::default());

    match shader.v3_spatial_shader_family() {
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(field)) => {
            assert!(matches!(
                field.behavior,
                VfxMotionFieldBehavior::TerminalWater { .. }
            ));
        }
        other => panic!("expected terminal water motion field, got {other:?}"),
    }
}

#[test]
fn terminal_fire_deserializes_and_reports_metadata() {
    let shader: SpatialShaderType = serde_json::from_value(serde_json::json!({
        "type": "terminal_fire",
        "mode": { "mode": "candle" },
        "apply_to": "both",
        "aspect": 1.0,
        "base_width": 0.18,
        "min_width": 0.035,
        "wind": 0.02,
        "rise_speed": 1.4,
        "turbulence": 0.55,
        "intensity": 0.85,
        "density": 1.0,
        "cooling": 0.78,
        "flicker_strength": 0.18,
        "blue_core_strength": 0.75,
        "white_core_strength": 0.45,
        "smoke_strength": 0.08,
        "sparks": { "seed": 7, "count": 0, "intensity": 0.35, "rise_speed": 1.2, "drift": 0.25 },
        "palette": {
            "blue_core": { "type": "rgb", "r": 0, "g": 215, "b": 255 },
            "white_core": { "type": "white" },
            "yellow": { "type": "rgb", "r": 255, "g": 215, "b": 0 },
            "orange": { "type": "rgb", "r": 255, "g": 95, "b": 0 },
            "red": { "type": "rgb", "r": 175, "g": 0, "b": 0 },
            "smoke": { "type": "rgb", "r": 88, "g": 88, "b": 88 }
        }
    }))
    .expect("valid terminal_fire candle recipe");

    assert_eq!(shader.name(), "TerminalFire");
    assert!(shader.terse_description().to_lowercase().contains("flame"));
    let keys: Vec<_> = shader
        .key_parameters()
        .into_iter()
        .map(|(key, _)| key)
        .collect();
    assert!(keys.contains(&"mode"));
    assert!(keys.contains(&"sparks"));
    assert!(keys.contains(&"blue_core"));
}

#[test]
fn terminal_fire_maps_to_motion_field_family() {
    let shader = SpatialShaderType::TerminalFire(TerminalFireShader::default());

    match shader.v3_spatial_shader_family() {
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(field)) => {
            assert!(matches!(
                field.behavior,
                VfxMotionFieldBehavior::TerminalFire { .. }
            ));
        }
        other => panic!("expected terminal fire motion field, got {other:?}"),
    }
}

#[test]
fn terminal_fire_v3_round_trip_preserves_mode() {
    // Lowering legacy → V3 → legacy must preserve the FireMode tuning.
    let original = SpatialShaderType::TerminalFire(TerminalFireShader {
        mode: FireMode::Campfire,
        wind: -0.18,
        smoke_strength: 0.75,
        ..TerminalFireShader::default()
    });

    let v3 = original.v3_spatial_shader_family();
    let lowered: SpatialShaderType = match v3 {
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(field)) => {
            (&field).into()
        }
        other => panic!("expected motion-field primitive, got {other:?}"),
    };

    match lowered {
        SpatialShaderType::TerminalFire(shader) => {
            assert_eq!(shader.mode, FireMode::Campfire);
            assert!((shader.wind - (-0.18)).abs() < 1e-6);
            assert!((shader.smoke_strength - 0.75).abs() < 1e-6);
        }
        other => panic!("round-trip lost variant: {other:?}"),
    }
}
