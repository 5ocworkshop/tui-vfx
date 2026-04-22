// <FILE>tui-vfx-style/src/models/v3/test_vfx_material_light_shader.rs</FILE> - <DESC>Focused tests for the V3 material-light family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 material-light surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxMaterialLightShader into a dedicated sibling file.</CLOG>

use super::{
    VfxConcealedLightMode, VfxConcealedLightSource, VfxDiffusionMode, VfxDiffusionSource,
    VfxMaterialLightApplyTo, VfxMaterialLightBehavior, VfxMaterialLightShader,
};
use crate::models::{
    ColorConfig, ConcealedLightMode, ConcealedLightShader, DiffusionMode, DiffusionShader,
    EdgeSheenApplyTo, EdgeSheenShader, SpatialShaderType,
};

#[test]
fn converts_diffusion_into_v3_material_light_surface() {
    let legacy = DiffusionShader {
        source: crate::models::DiffusionSource::BottomRight,
        color: ColorConfig::White,
        radius: 9,
        softness: 0.4,
        edge_firmness: 0.3,
        falloff: crate::models::FalloffType::Quadratic,
        intensity: 0.22,
        apply_to: crate::models::DiffusionApplyTo::Both,
        mode: DiffusionMode::Breath,
        drift_speed: 1.2,
        drift_amount: 0.08,
    };

    let converted = VfxMaterialLightShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMaterialLightBehavior::Diffusion {
            source: VfxDiffusionSource::BottomRight,
            color: ColorConfig::White,
            radius: 9,
            softness: 0.4,
            edge_firmness: 0.3,
            falloff: crate::models::FalloffType::Quadratic,
            intensity: 0.22,
            apply_to: VfxMaterialLightApplyTo::Both,
            mode: VfxDiffusionMode::Breath,
            drift_speed: 1.2,
            drift_amount: 0.08,
        }
    );
}

#[test]
fn converts_concealed_light_into_v3_material_light_surface() {
    let legacy = ConcealedLightShader {
        color: ColorConfig::Cyan,
        source: crate::models::ConcealedLightSource::Left,
        spread: 5,
        edge_width: 2,
        falloff: crate::models::FalloffType::Quadratic,
        intensity: 0.2,
        apply_to: crate::models::ConcealedLightApplyTo::Foreground,
        mode: ConcealedLightMode::Pulse,
        pulse_speed: 0.9,
        source_cutoff: 0.12,
    };

    let converted = VfxMaterialLightShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMaterialLightBehavior::ConcealedLight {
            source: VfxConcealedLightSource::Left,
            color: ColorConfig::Cyan,
            spread: 5,
            edge_width: 2,
            falloff: crate::models::FalloffType::Quadratic,
            intensity: 0.2,
            apply_to: VfxMaterialLightApplyTo::Foreground,
            mode: VfxConcealedLightMode::Pulse,
            pulse_speed: 0.9,
            source_cutoff: 0.12,
        }
    );
}

#[test]
fn returns_none_for_non_material_light_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxMaterialLightShader::from_legacy_spatial_shader(&shader).is_none());
}

#[test]
fn converts_edge_sheen_into_v3_material_light_surface() {
    let legacy = EdgeSheenShader {
        color: ColorConfig::Yellow,
        speed: 1.1,
        band_width: 12,
        edge_width: 3,
        intensity: 0.66,
        corner_boost: 0.4,
        apply_to: EdgeSheenApplyTo::Both,
    };

    let converted = VfxMaterialLightShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMaterialLightBehavior::EdgeSheen {
            color: ColorConfig::Yellow,
            speed: 1.1,
            band_width: 12,
            edge_width: 3,
            intensity: 0.66,
            corner_boost: 0.4,
            apply_to: VfxMaterialLightApplyTo::Both,
        }
    );
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_material_light_shader.rs</FILE> - <DESC>Focused tests for the V3 material-light family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
