// <FILE>tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs</FILE> - <DESC>Focused tests for the V3 stochastic-texture family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 stochastic-texture surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxStochasticTextureShader into a dedicated sibling file.</CLOG>

use super::{
    VfxStochasticTextureBehavior, VfxStochasticTextureShader, VfxTextureSegmentMode,
    VfxTextureTarget,
};
use crate::models::{NoiseType, SpatialShaderType, StochasticSparkleShader};

#[test]
fn converts_neon_flicker_into_v3_stochastic_texture_surface() {
    let legacy = crate::models::NeonFlickerShader {
        stability: 0.6,
        seed: 7,
        segment: crate::models::SegmentMode::Column,
        dim_amount: 0.9,
        speed: 1.4,
        flash_chance: 0.1,
        decay_rate: Some(2.0),
        noise_type: NoiseType::Gaussian,
    };

    let converted = VfxStochasticTextureShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxStochasticTextureBehavior::NeonFlicker {
            stability: 0.6,
            seed: 7,
            segment: VfxTextureSegmentMode::Column,
            dim_amount: 0.9,
            speed: 1.4,
            flash_chance: 0.1,
            decay_rate: Some(2.0),
            noise_type: NoiseType::Gaussian,
        }
    );
}

#[test]
fn converts_stochastic_sparkle_into_v3_stochastic_texture_surface() {
    let legacy = StochasticSparkleShader {
        sparkle_density: 0.07,
        brightness_boost: 1.35,
        speed: 0.5,
        seed: 13,
        apply_to: crate::models::SparkleTarget::Both,
        noise_type: NoiseType::Uniform,
    };

    let converted = VfxStochasticTextureShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxStochasticTextureBehavior::StochasticSparkle {
            sparkle_density: 0.07,
            brightness_boost: 1.35,
            speed: 0.5,
            seed: 13,
            apply_to: VfxTextureTarget::Both,
            noise_type: NoiseType::Uniform,
        }
    );
}

#[test]
fn returns_none_for_non_stochastic_texture_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxStochasticTextureShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_stochastic_texture_shader.rs</FILE> - <DESC>Focused tests for the V3 stochastic-texture family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
