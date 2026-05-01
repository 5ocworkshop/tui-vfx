// <FILE>tui-vfx-style/src/models/v3/cls_vfx_stochastic_texture_shader.rs</FILE> - <DESC>V3 stochastic-texture family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 surface for stochastic texture shaders so NeonFlicker and StochasticSparkle become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxStochasticTextureShader plus conversion helpers from the legacy stochastic texture variants and SpatialShaderType.</CLOG>

//! V3 family surface for stochastic-texture shaders.
//!
//! This grouped type provides a forward-looking V3 home for the noise-driven
//! flicker/sparkle treatments that currently live as separate flat variants.

use crate::models::v3::enum_vfx_stochastic_texture_behavior::{
    VfxStochasticTextureBehavior, VfxTextureSegmentMode, VfxTextureTarget,
};
use crate::models::{
    NeonFlickerShader, SegmentMode, SparkleTarget, SpatialShaderType, StochasticSparkleShader,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for stochastic-texture shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxStochasticTextureShader {
    /// Behavior/configuration surface for the chosen stochastic-texture family member.
    pub behavior: VfxStochasticTextureBehavior,
}

impl VfxStochasticTextureShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// stochastic-texture family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::NeonFlicker(shader) => Some(Self::from(shader)),
            SpatialShaderType::StochasticSparkle(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&NeonFlickerShader> for VfxStochasticTextureShader {
    fn from(shader: &NeonFlickerShader) -> Self {
        Self {
            behavior: VfxStochasticTextureBehavior::NeonFlicker {
                stability: shader.stability,
                seed: shader.seed,
                segment: shader.segment.into(),
                dim_amount: shader.dim_amount,
                base_color: shader.base_color,
                italic_window: shader.italic_window,
                speed: shader.speed,
                flash_chance: shader.flash_chance,
                decay_rate: shader.decay_rate,
                noise_type: shader.noise_type,
            },
        }
    }
}

impl From<&StochasticSparkleShader> for VfxStochasticTextureShader {
    fn from(shader: &StochasticSparkleShader) -> Self {
        Self {
            behavior: VfxStochasticTextureBehavior::StochasticSparkle {
                sparkle_density: shader.sparkle_density,
                brightness_boost: shader.brightness_boost,
                speed: shader.speed,
                seed: shader.seed,
                apply_to: shader.apply_to.into(),
                noise_type: shader.noise_type,
            },
        }
    }
}

impl From<SegmentMode> for VfxTextureSegmentMode {
    fn from(value: SegmentMode) -> Self {
        match value {
            SegmentMode::Cell => Self::Cell,
            SegmentMode::Row => Self::Row,
            SegmentMode::Column => Self::Column,
        }
    }
}

impl From<SparkleTarget> for VfxTextureTarget {
    fn from(value: SparkleTarget) -> Self {
        match value {
            SparkleTarget::Foreground => Self::Foreground,
            SparkleTarget::Background => Self::Background,
            SparkleTarget::Both => Self::Both,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_stochastic_texture_shader.rs</FILE> - <DESC>V3 stochastic-texture family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
