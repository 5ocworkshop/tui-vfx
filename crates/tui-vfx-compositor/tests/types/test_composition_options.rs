// <FILE>tui-vfx-compositor/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime-facing seam extension — ensure CompositionOptions can report grouped V3 shader families when layers were constructed through a lowering-aware path, without changing the legacy shader application path.</WCTX>
// <CLOG>Add coverage for CompositionOptions::v3_shader_families across unknown and known shader-layer construction paths.</CLOG>

use tui_vfx_compositor::pipeline::{CompositionOptions, ShaderWithRegion};
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

// <FILE>tui-vfx-compositor/tests/types/test_composition_options.rs</FILE> - <DESC>Tests for CompositionOptions V3 family exposure helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
