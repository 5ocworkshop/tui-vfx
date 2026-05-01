// <FILE>tui-vfx-compositor-next/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Runtime-facing seam extension — ensure CompositionOptions can report grouped V3 shader families when layers were constructed through a lowering-aware path, without changing the legacy shader application path.</WCTX>
// <CLOG>0.3.0: add sampler-chain compatibility coverage for legacy and ordered CompositionOptions paths.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor_next::pipeline::{
    CompositionOptions, CompositionPlaybackTiming, ShaderWithRegion,
};
use tui_vfx_compositor_next::types::{Axis, SamplerSpec};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, GlowShader, StyleRegion, VfxSpatialComposedPrimitive,
    VfxSpatialPrimitive, VfxSpatialShaderFamily,
};

#[test]
fn composition_options_ignores_unknown_v3_shader_families() {
    let shader = GlowShader::default();
    let options = CompositionOptions::default().with_shader_layer(&shader, StyleRegion::All);
    assert!(options.v3_shader_families().is_empty());
}

#[test]
fn composition_options_reports_known_v3_shader_families() {
    let border = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        head: None,
        tail: None,
        position_binding: None,
    };
    let glow = GlowShader::default();
    let mut options = CompositionOptions::default();
    options.shader_layers.push(ShaderWithRegion {
        shader: &border,
        region: StyleRegion::All,
        v3_family: Some(VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand((&border).into()),
        )),
        shader_label: Some("BorderSweep".to_string()),
    });
    options.shader_layers.push(ShaderWithRegion {
        shader: &glow,
        region: StyleRegion::All,
        v3_family: Some(VfxSpatialShaderFamily::Primitive(
            VfxSpatialPrimitive::SurfaceDepth((&glow).into()),
        )),
        shader_label: Some("Glow".to_string()),
    });

    let families = options.v3_shader_families();
    assert_eq!(families.len(), 2);
    assert!(matches!(
        families[0],
        VfxSpatialShaderFamily::ComposedPrimitive(_)
    ));
    assert!(matches!(families[1], VfxSpatialShaderFamily::Primitive(_)));
}

#[test]
fn composition_options_can_add_grouped_v3_shader_family_directly() {
    let glow = GlowShader::default();
    let family =
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth((&glow).into()));

    let options = CompositionOptions::default()
        .try_with_v3_shader_family(&family, &glow, StyleRegion::All)
        .expect("lowers");

    let families = options.v3_shader_families();
    assert_eq!(families, vec![family]);
    assert_eq!(
        options.shader_layers[0].shader_label.as_deref(),
        Some("Glow")
    );
}

#[test]
fn shader_with_region_can_build_from_grouped_v3_family() {
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

    let layer = ShaderWithRegion::try_from_v3_shader_family(&family, &border, StyleRegion::All)
        .expect("lowers");

    assert_eq!(layer.v3_family, Some(family));
    assert_eq!(layer.shader_label.as_deref(), Some("BorderSweep"));
}

#[test]
fn composition_options_can_apply_shared_playback_timing() {
    let timing =
        CompositionPlaybackTiming::new(1.3, Some(1.4), Some(mixed_signals::traits::Phase::End));
    let options = CompositionOptions::default().with_playback_timing(timing);

    assert_eq!(options.t, 1.0);
    assert_eq!(options.loop_t, Some(1.0));
    assert_eq!(options.phase, Some(mixed_signals::traits::Phase::End));
}

#[test]
fn composition_playback_timing_from_options_falls_back_to_phase_progress() {
    let options = CompositionOptions::default().with_playback_timing(
        CompositionPlaybackTiming::new(0.4, None, Some(mixed_signals::traits::Phase::Start)),
    );
    let timing = CompositionPlaybackTiming::from_options(&options);

    assert_eq!(timing.effective_loop_t(), 0.4);
    assert_eq!(timing.shader_t(), 0.4);
}

#[test]
fn composition_options_effective_samplers_fall_back_to_legacy_field() {
    let gravity = SamplerSpec::Gravity {
        axis: Axis::X,
        acceleration: SignalOrFloat::Static(2.0),
        terminal_velocity: SignalOrFloat::Static(2.0),
    };
    let options = CompositionOptions {
        sampler_spec: Some(gravity.clone()),
        ..CompositionOptions::default()
    };

    assert_eq!(options.effective_samplers().as_ref(), &[gravity]);
    assert!(options.has_active_sampler());
}

#[test]
fn composition_options_with_samplers_preserves_declared_order() {
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
    let options =
        CompositionOptions::default().with_samplers(vec![gravity.clone(), pendulum.clone()]);

    assert_eq!(
        options.effective_samplers().as_ref(),
        &[gravity.clone(), pendulum]
    );
    assert_eq!(options.sampler_spec, Some(gravity));
}

// <FILE>tui-vfx-compositor-next/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
