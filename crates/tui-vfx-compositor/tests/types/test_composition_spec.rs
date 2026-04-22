// <FILE>tui-vfx-compositor/tests/types/test_composition_spec.rs</FILE> - <DESC>Tests for CompositionSpec V3 family lowering helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>First runtime-facing wiring slice for the central style-family seam — ensure composition specs can expose grouped V3 families without changing the legacy execution path.</WCTX>
// <CLOG>Add coverage for CompositionSpec::v3_shader_families across empty, primitive, and composed shader-layer sets.</CLOG>

use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
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

// <FILE>tui-vfx-compositor/tests/types/test_composition_spec.rs</FILE> - <DESC>Tests for CompositionSpec V3 family lowering helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
