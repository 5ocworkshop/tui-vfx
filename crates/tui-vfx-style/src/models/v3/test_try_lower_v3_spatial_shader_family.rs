// <FILE>tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Focused tests for grouped V3 spatial families lowering back into the legacy runtime surface</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>V3 shader-lane hardening — prove grouped spatial families lower back into executable legacy shaders and surface exact traveling-band blocker contracts.</WCTX>
// <CLOG>0.5.0: prove V3 head_tail colors now lower losslessly for border, reflect, trace_propagation, and trace_path legacy traveling-band variants.</CLOG>

use crate::models::v3::{VfxSpatialShaderFamily, try_lower_v3_spatial_shader_family};
use crate::models::{
    AffordanceWakeShader, AmbientOcclusionShader, BarberPoleShader, BevelShader, BorderSweepShader,
    ChromaticEdgeShader, ColorConfig, ConcealedLightShader, CursorShader, DiffusionShader,
    EdgeSheenShader, FocusFieldShader, FocusedRowGradientShader, GlistenBandShader,
    GlitchLinesShader, GlowShader, Gradient, HighlighterShader, LinearGradientShader,
    ModifierWindowShader, NeonFlickerShader, OrbitShader, PulseWaveShader, RadarShader,
    RadialSpiralShader, RainbowCycleShader, ReflectShader, RevealWipeShader, SpatialShaderType,
    StochasticSparkleShader, SubCellShakeShader, TracePathShader, TracePropagationShader,
    WayfindingNodeShader,
};

fn assert_legacy_roundtrip(legacy: SpatialShaderType) {
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);
    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

fn set_traveling_band_color(
    family: &mut VfxSpatialShaderFamily,
    color: crate::models::VfxTravelingBandColor,
) {
    match family {
        VfxSpatialShaderFamily::ComposedPrimitive(
            crate::models::VfxSpatialComposedPrimitive::TravelingBand(shader),
        ) => shader.color = color,
        _ => panic!("expected traveling-band family"),
    }
}

#[test]
fn roundtrips_surface_depth_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::Glow(GlowShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_progress_emphasis_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::Highlighter(HighlighterShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_traveling_band_border_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::BorderSweep(BorderSweepShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_modifier_window_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::ModifierWindow(ModifierWindowShader {
        start: 0.2,
        end: 0.8,
        italic: true,
    });
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_material_light_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::Diffusion(DiffusionShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_guidance_cue_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_motion_field_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::PulseWave(PulseWaveShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_edge_distortion_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::GlitchLines(GlitchLinesShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_gradient_reveal_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::LinearGradient(LinearGradientShader::new(Gradient::default()));
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_stochastic_texture_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::NeonFlicker(NeonFlickerShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_cursor_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::Cursor(CursorShader::default());
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn roundtrips_stripe_motion_family_back_to_legacy_shader() {
    let legacy = SpatialShaderType::BarberPole(BarberPoleShader {
        speed: 1.0,
        stripe_width: 2,
        gap_width: 2,
        angle_deg: 0.0,
        color: crate::models::ColorConfig::Red,
        background_color: None,
        apply_to: Default::default(),
    });
    let family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);

    assert_eq!(try_lower_v3_spatial_shader_family(&family).unwrap(), legacy);
}

#[test]
fn lowers_head_tail_color_policy_for_legacy_traveling_band_variants() {
    let cases = [
        (
            SpatialShaderType::BorderSweep(BorderSweepShader::default()),
            SpatialShaderType::BorderSweep(BorderSweepShader {
                color: ColorConfig::White,
                head: Some(ColorConfig::White),
                tail: Some(ColorConfig::Black),
                ..BorderSweepShader::default()
            }),
        ),
        (
            SpatialShaderType::Reflect(ReflectShader {
                speed: 2.0,
                color: ColorConfig::White,
                ..ReflectShader::default()
            }),
            SpatialShaderType::Reflect(ReflectShader {
                speed: 2.0,
                color: ColorConfig::White,
                head: Some(ColorConfig::White),
                tail: Some(ColorConfig::Black),
                ..ReflectShader::default()
            }),
        ),
        (
            SpatialShaderType::TracePropagation(TracePropagationShader::default()),
            SpatialShaderType::TracePropagation(TracePropagationShader {
                color: ColorConfig::White,
                head: Some(ColorConfig::White),
                tail: Some(ColorConfig::Black),
                ..TracePropagationShader::default()
            }),
        ),
        (
            SpatialShaderType::TracePath(TracePathShader::default()),
            SpatialShaderType::TracePath(TracePathShader {
                color: ColorConfig::White,
                head: Some(ColorConfig::White),
                tail: Some(ColorConfig::Black),
                ..TracePathShader::default()
            }),
        ),
    ];

    for (legacy, expected) in cases {
        let mut family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&legacy);
        set_traveling_band_color(
            &mut family,
            crate::models::VfxTravelingBandColor::HeadTail {
                head: crate::models::ColorConfig::White,
                tail: crate::models::ColorConfig::Black,
            },
        );

        assert_eq!(try_lower_v3_spatial_shader_family(&family), Ok(expected));
    }
}

#[test]
fn roundtrips_all_individual_spatial_shader_variants_back_to_legacy_shader() {
    assert_legacy_roundtrip(SpatialShaderType::AmbientOcclusion(
        AmbientOcclusionShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::Bevel(BevelShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::Glow(GlowShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::Radar(RadarShader {
        speed: 1.0,
        tail_length: 1.0,
        color: ColorConfig::Green,
    }));
    assert_legacy_roundtrip(SpatialShaderType::Orbit(OrbitShader {
        speed: 1.0,
        dot_count: 3,
        color: ColorConfig::Cyan,
    }));
    assert_legacy_roundtrip(SpatialShaderType::PulseWave(PulseWaveShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::RadialSpiral(
        RadialSpiralShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::GlitchLines(GlitchLinesShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::ChromaticEdge(
        ChromaticEdgeShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::SubCellShake(
        SubCellShakeShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::LinearGradient(
        LinearGradientShader::new(Gradient::default()),
    ));
    assert_legacy_roundtrip(SpatialShaderType::RevealWipe(RevealWipeShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::BorderSweep(BorderSweepShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::Reflect(ReflectShader {
        speed: 2.0,
        color: ColorConfig::White,
        ..ReflectShader::default()
    }));
    assert_legacy_roundtrip(SpatialShaderType::GlistenBand(GlistenBandShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::TracePropagation(
        TracePropagationShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::TracePath(TracePathShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::Highlighter(HighlighterShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::Diffusion(DiffusionShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::ConcealedLight(
        ConcealedLightShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::EdgeSheen(EdgeSheenShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::FocusedRowGradient(
        FocusedRowGradientShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::FocusField(FocusFieldShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::AffordanceWake(
        AffordanceWakeShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::WayfindingNode(
        WayfindingNodeShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::NeonFlicker(NeonFlickerShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::RainbowCycle(RainbowCycleShader {
        rotation_speed: 1.5,
    }));
    assert_legacy_roundtrip(SpatialShaderType::StochasticSparkle(
        StochasticSparkleShader::default(),
    ));
    assert_legacy_roundtrip(SpatialShaderType::Cursor(CursorShader::default()));
    assert_legacy_roundtrip(SpatialShaderType::BarberPole(BarberPoleShader {
        speed: 1.0,
        stripe_width: 2,
        gap_width: 2,
        angle_deg: 0.0,
        color: ColorConfig::Red,
        background_color: None,
        apply_to: Default::default(),
    }));
}

#[test]
fn lowers_non_default_reflect_geometry_for_executable_lowering() {
    let mut family = VfxSpatialShaderFamily::from_legacy_spatial_shader(
        &SpatialShaderType::Reflect(ReflectShader {
            speed: 2.0,
            color: ColorConfig::White,
            ..ReflectShader::default()
        }),
    );
    if let VfxSpatialShaderFamily::ComposedPrimitive(
        crate::models::VfxSpatialComposedPrimitive::TravelingBand(shader),
    ) = &mut family
    {
        shader.behavior = crate::models::VfxTravelingBandBehavior::Reflect {
            gap: 40.0,
            width: 3.5,
        };
    }

    assert_eq!(
        try_lower_v3_spatial_shader_family(&family),
        Ok(SpatialShaderType::Reflect(ReflectShader {
            speed: 2.0,
            color: ColorConfig::White,
            head: None,
            tail: None,
            gap: 40.0,
            width: 3.5,
        }))
    );
}

// <FILE>tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Focused tests for grouped V3 spatial families lowering back into the legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
