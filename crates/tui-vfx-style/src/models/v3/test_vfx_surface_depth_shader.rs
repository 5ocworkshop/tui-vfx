// <FILE>tui-vfx-style/src/models/v3/test_vfx_surface_depth_shader.rs</FILE> - <DESC>Focused tests for the V3 surface-depth family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 surface-depth surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxSurfaceDepthShader into a dedicated sibling file.</CLOG>

use super::{
    VfxSurfaceDepthBehavior, VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection,
    VfxSurfaceDepthShader,
};
use crate::models::{
    AmbientOcclusionShader, BevelShader, ColorConfig, GlowShader, SpatialShaderType,
};

#[test]
fn converts_ambient_occlusion_into_v3_surface_depth_surface() {
    let legacy = AmbientOcclusionShader {
        intensity: 0.4,
        radius: 3,
        edges: crate::models::AOEdges::All,
        falloff: crate::models::FalloffType::Quadratic,
        shadow_color: ColorConfig::Black,
    };

    let converted = VfxSurfaceDepthShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxSurfaceDepthBehavior::AmbientOcclusion {
            intensity: 0.4,
            radius: 3,
            edges: VfxSurfaceDepthEdges::All,
            falloff: crate::models::FalloffType::Quadratic,
            shadow_color: ColorConfig::Black,
        }
    );
}

#[test]
fn converts_bevel_into_v3_surface_depth_surface() {
    let legacy = BevelShader {
        light_direction: crate::models::LightDirection::BottomRight,
        highlight_intensity: 0.2,
        shadow_intensity: 0.45,
        edge_width: 2,
    };

    let converted = VfxSurfaceDepthShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxSurfaceDepthBehavior::Bevel {
            light_direction: VfxSurfaceDepthLightDirection::BottomRight,
            highlight_intensity: 0.2,
            shadow_intensity: 0.45,
            edge_width: 2,
        }
    );
}

#[test]
fn converts_glow_into_v3_surface_depth_surface() {
    let legacy = GlowShader {
        color: ColorConfig::Cyan,
        radius: 4,
        falloff: crate::models::FalloffType::Quadratic,
        intensity: 0.75,
        pulse_speed: 1.2,
    };

    let converted = VfxSurfaceDepthShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxSurfaceDepthBehavior::Glow {
            color: ColorConfig::Cyan,
            radius: 4,
            falloff: crate::models::FalloffType::Quadratic,
            intensity: 0.75,
            pulse_speed: 1.2,
        }
    );
}

#[test]
fn returns_none_for_non_surface_depth_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxSurfaceDepthShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_surface_depth_shader.rs</FILE> - <DESC>Focused tests for the V3 surface-depth family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
