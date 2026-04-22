// <FILE>tui-vfx-compositor/tests/types/test_shader_layer_spec.rs</FILE> - <DESC>Tests for ShaderLayerSpec V3 family lowering helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>First runtime-facing wiring slice for the central style-family seam — ensure serializable shader layer specs can expose the grouped V3 family form without changing the legacy execution path.</WCTX>
// <CLOG>Add coverage for ShaderLayerSpec::v3_shader_family across representative primitive and composed families.</CLOG>

use tui_vfx_compositor::pipeline::ShaderLayerSpec;
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, SpatialShaderType, StyleRegion, VfxSpatialComposedPrimitive,
    VfxSpatialPrimitive, VfxSpatialShaderFamily,
};

#[test]
fn shader_layer_spec_exposes_composed_v3_family() {
    let spec = ShaderLayerSpec {
        shader: SpatialShaderType::BorderSweep(BorderSweepShader {
            speed: 1.0,
            length: 3,
            color: ColorConfig::Red,
            position_binding: None,
        }),
        region: StyleRegion::All,
    };

    assert!(matches!(
        spec.v3_shader_family(),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::TravelingBand(_))
    ));
}

#[test]
fn shader_layer_spec_exposes_primitive_v3_family() {
    let spec = ShaderLayerSpec {
        shader: SpatialShaderType::Glow(tui_vfx_style::models::GlowShader::default()),
        region: StyleRegion::All,
    };

    assert!(matches!(
        spec.v3_shader_family(),
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(_))
    ));
}

// <FILE>tui-vfx-compositor/tests/types/test_shader_layer_spec.rs</FILE> - <DESC>Tests for ShaderLayerSpec V3 family lowering helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>

#[test]
fn shader_layer_spec_can_build_from_v3_shader_family() {
    let family = tui_vfx_style::models::VfxSpatialShaderFamily::from_legacy_spatial_shader(
        &tui_vfx_style::models::SpatialShaderType::Glow(tui_vfx_style::models::GlowShader::default()),
    );

    let spec = ShaderLayerSpec::try_from_v3_shader_family(
        &family,
        tui_vfx_style::models::StyleRegion::All,
    )
    .expect("lowers");

    assert!(matches!(spec.shader, tui_vfx_style::models::SpatialShaderType::Glow(_)));
}
