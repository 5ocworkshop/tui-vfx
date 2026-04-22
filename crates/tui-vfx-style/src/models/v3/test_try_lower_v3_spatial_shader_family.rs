// <FILE>tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Focused tests for grouped V3 spatial families lowering back into the legacy runtime surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 runtime follow-on — prove grouped V3 spatial shader families can lower back into the current executable legacy shader surface for representative primitive and composed families.</WCTX>
// <CLOG>0.1.0: add grouped-V3-to-legacy lowering coverage plus one explicit unsupported-color-policy case.</CLOG>

use crate::models::{
    BorderSweepShader, GlowShader, HighlighterShader, SpatialShaderType,
};
use crate::models::v3::{VfxSpatialShaderFamily, try_lower_v3_spatial_shader_family};

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
fn rejects_unrepresentable_traveling_band_color_policy_for_border_variant() {
    let mut family = VfxSpatialShaderFamily::from_legacy_spatial_shader(&SpatialShaderType::BorderSweep(BorderSweepShader::default()));
    if let VfxSpatialShaderFamily::ComposedPrimitive(crate::models::VfxSpatialComposedPrimitive::TravelingBand(shader)) = &mut family {
        shader.color = crate::models::VfxTravelingBandColor::HeadTail {
            head: crate::models::ColorConfig::White,
            tail: crate::models::ColorConfig::Black,
        };
    }

    assert!(try_lower_v3_spatial_shader_family(&family).is_err());
}

// <FILE>tui-vfx-style/src/models/v3/test_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Focused tests for grouped V3 spatial families lowering back into the legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
