// <FILE>tui-vfx-compositor/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Runtime-facing seam extension — ensure CompositionOptions can report grouped V3 shader families when layers were constructed through a lowering-aware path, without changing the legacy shader application path.</WCTX>
// <CLOG>0.2.0: add grouped-V3 runtime-construction coverage through ShaderWithRegion::try_from_v3_shader_family and CompositionOptions::try_with_v3_shader_family.
// Add coverage for CompositionOptions::v3_shader_families across unknown and known shader-layer construction paths.</CLOG>

use tui_vfx_compositor::pipeline::{CompositionOptions, CompositionPlaybackTiming, ShaderWithRegion};
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
    let family = VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth((&glow).into()));

    let options = CompositionOptions::default()
        .try_with_v3_shader_family(&family, &glow, StyleRegion::All)
        .expect("lowers");

    let families = options.v3_shader_families();
    assert_eq!(families, vec![family]);
    assert_eq!(options.shader_layers[0].shader_label.as_deref(), Some("Glow"));
}

#[test]
fn shader_with_region_can_build_from_grouped_v3_family() {
    let border = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
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
    let timing = CompositionPlaybackTiming::new(
        1.3,
        Some(1.4),
        Some(mixed_signals::traits::Phase::End),
    );
    let options = CompositionOptions::default().with_playback_timing(timing);

    assert_eq!(options.t, 1.0);
    assert_eq!(options.loop_t, Some(1.0));
    assert_eq!(options.phase, Some(mixed_signals::traits::Phase::End));
}

#[test]
fn composition_playback_timing_from_options_falls_back_to_phase_progress() {
    let options = CompositionOptions::default()
        .with_playback_timing(CompositionPlaybackTiming::new(
            0.4,
            None,
            Some(mixed_signals::traits::Phase::Start),
        ));
    let timing = CompositionPlaybackTiming::from_options(&options);

    assert_eq!(timing.effective_loop_t(), 0.4);
    assert_eq!(timing.shader_t(), 0.4);
}

// <FILE>tui-vfx-compositor/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
