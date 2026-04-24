// <FILE>tui-vfx-compositor/tests/types/test_composition_spec.rs</FILE> - <DESC>Tests for CompositionSpec V3 family lowering helpers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>First runtime-facing wiring slice for the central style-family seam — ensure composition specs can expose grouped V3 families without changing the legacy execution path.</WCTX>
// <CLOG>0.3.0: add sampler-chain compatibility coverage for legacy and ordered CompositionSpec paths.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::pipeline::{CompositionPlaybackTiming, CompositionSpec, ShaderLayerSpec};
use tui_vfx_compositor::types::{Axis, SamplerSpec};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, GlowShader, SpatialShaderType, StyleRegion,
    VfxSpatialComposedPrimitive, VfxSpatialPrimitive, VfxSpatialShaderFamily,
};

#[test]
fn composition_spec_reports_no_v3_shader_families_when_empty() {
    let spec = CompositionSpec::default();
    assert!(spec.v3_shader_families().is_empty());
}

#[test]
fn composition_spec_exposes_mixed_v3_shader_families() {
    let spec = CompositionSpec {
        shader_layers: vec![
            ShaderLayerSpec {
                shader: SpatialShaderType::BorderSweep(BorderSweepShader {
                    speed: 1.0,
                    length: 3,
                    color: ColorConfig::Red,
                    head: None,
                    tail: None,
                    position_binding: None,
                }),
                region: StyleRegion::All,
            },
            ShaderLayerSpec {
                shader: SpatialShaderType::Glow(GlowShader::default()),
                region: StyleRegion::All,
            },
        ],
        ..CompositionSpec::default()
    };

    let families = spec.v3_shader_families();
    assert_eq!(families.len(), 2);
    assert!(matches!(
        families[0],
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::TravelingBand(_))
    ));
    assert!(matches!(
        families[1],
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(_))
    ));
}

#[test]
fn composition_spec_can_push_grouped_v3_shader_family() {
    let family = VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(
        (&GlowShader::default()).into(),
    ));
    let mut spec = CompositionSpec::default();
    spec.try_push_v3_shader_family(&family, StyleRegion::All)
        .expect("lowers");

    assert_eq!(spec.v3_shader_families(), vec![family]);
}

#[test]
fn composition_spec_can_append_grouped_v3_shader_family_via_builder() {
    let border = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        head: None,
        tail: None,
        position_binding: None,
    };
    let family = VfxSpatialShaderFamily::ComposedPrimitive(
        VfxSpatialComposedPrimitive::TravelingBand((&border).into()),
    );

    let spec = CompositionSpec::default()
        .try_with_v3_shader_family(&family, StyleRegion::All)
        .expect("lowers");

    assert_eq!(spec.v3_shader_families(), vec![family]);
}

#[test]
fn composition_spec_can_apply_shared_playback_timing() {
    let timing =
        CompositionPlaybackTiming::new(1.5, Some(1.2), Some(mixed_signals::traits::Phase::Active));
    let mut spec = CompositionSpec::default();
    spec.apply_playback_timing(timing);

    assert_eq!(spec.t, 1.0);
    assert_eq!(spec.loop_t, Some(1.0));
    assert_eq!(spec.phase, Some(mixed_signals::traits::Phase::Active));
}

#[test]
fn composition_playback_timing_from_spec_exposes_effective_loop_and_shader_progress() {
    let spec = CompositionSpec::default().with_playback_timing(CompositionPlaybackTiming::new(
        0.25,
        Some(0.75),
        Some(mixed_signals::traits::Phase::Active),
    ));
    let timing = CompositionPlaybackTiming::from_spec(&spec);

    assert_eq!(timing.effective_loop_t(), 0.75);
    assert_eq!(timing.shader_t(), 0.75);
}

#[test]
fn composition_spec_effective_samplers_fall_back_to_legacy_field() {
    let gravity = SamplerSpec::Gravity {
        axis: Axis::X,
        acceleration: SignalOrFloat::Static(2.0),
        terminal_velocity: SignalOrFloat::Static(2.0),
    };
    let spec = CompositionSpec {
        sampler_spec: Some(gravity.clone()),
        ..CompositionSpec::default()
    };

    assert_eq!(spec.effective_samplers(), vec![gravity]);
    assert!(spec.has_active_sampler());
}

#[test]
fn composition_spec_push_sampler_preserves_order_and_compatibility_mirror() {
    let gravity = SamplerSpec::Gravity {
        axis: Axis::X,
        acceleration: SignalOrFloat::Static(2.0),
        terminal_velocity: SignalOrFloat::Static(2.0),
    };
    let pendulum = SamplerSpec::Pendulum {
        axis: Axis::Y,
        amplitude: SignalOrFloat::Static(1.0),
        speed: SignalOrFloat::Static(0.0),
        phase_spread: SignalOrFloat::Static(std::f32::consts::FRAC_PI_2),
    };
    let mut spec = CompositionSpec::default();
    spec.push_sampler(gravity.clone());
    spec.push_sampler(pendulum.clone());

    assert_eq!(spec.effective_samplers(), vec![gravity.clone(), pendulum]);
    assert_eq!(spec.sampler_spec, Some(gravity));
}

// <FILE>tui-vfx-compositor/tests/types/test_composition_spec.rs</FILE> - <DESC>Tests for CompositionSpec V3 family lowering helpers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
