// <FILE>tui-vfx-style/tests/models/test_cls_spatial_shader_type.rs</FILE> - <DESC>Focused V3 payload-constructor coverage for SpatialShaderType</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Push representative V3-authored shader payload normalization closer to executable shader semantics instead of leaving it in recipes-only bridge code.</WCTX>
// <CLOG>0.1.0: add focused coverage for gradient_overlay, colored_overlay pattern lowering, and runtime-binding extraction through SpatialShaderType::try_from_v3_payload.</CLOG>

use tui_vfx_style::models::{
    AmbientOcclusionShader, ConcealedLightMode, DiffusionMode, HighlighterDirection,
    LinearGradientShader, SpatialShaderType, VfxSpatialShaderFamily,
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
