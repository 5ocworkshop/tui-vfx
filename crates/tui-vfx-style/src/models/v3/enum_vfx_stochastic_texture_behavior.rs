// <FILE>tui-vfx-style/src/models/v3/enum_vfx_stochastic_texture_behavior.rs</FILE> - <DESC>V3 stochastic-texture family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for stochastic/flicker texture shaders while preserving the legacy NeonFlicker and StochasticSparkle variants for current playback.</WCTX>
// <CLOG>Define the V3 stochastic-texture family enums and payloads that lift the shared noise-driven texture behavior out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for stochastic-texture shaders.
//!
//! This family groups the noise-driven texture/flicker treatments currently
//! exposed as `NeonFlicker` and `StochasticSparkle`.

use crate::models::NoiseType;
use serde::{Deserialize, Serialize};

/// Segmenting policy for flicker-style stochastic textures.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxTextureSegmentMode {
    /// Each cell flickers independently.
    Cell,
    /// Each row flickers as a unit.
    #[default]
    Row,
    /// Each column flickers as a unit.
    Column,
}

/// Target channel(s) for sparkle-style textures.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxTextureTarget {
    /// Apply texture to the foreground only.
    Foreground,
    /// Apply texture to the background only.
    #[default]
    Background,
    /// Apply texture to both channels.
    Both,
}

/// Behavior surface for the V3 stochastic-texture family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxStochasticTextureBehavior {
    /// Independently flickering segments like a damaged neon sign.
    NeonFlicker {
        /// Base stability of the sign.
        #[config(default = 0.7)]
        stability: f32,
        /// Deterministic seed.
        #[config(default = 42)]
        seed: u64,
        /// Segment grouping policy.
        #[serde(default)]
        segment: VfxTextureSegmentMode,
        /// Dimming amount during flicker.
        #[config(default = 0.8)]
        dim_amount: f32,
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Chance of a white flash instead of dim.
        #[serde(default)]
        flash_chance: f32,
        /// Optional smooth recovery decay.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[config(opaque)]
        decay_rate: Option<f32>,
        /// Noise distribution type.
        #[serde(default)]
        noise_type: NoiseType,
    },
    /// Randomly brightened cells for grain, frost, or sparkle.
    StochasticSparkle {
        /// Fraction of cells that sparkle per frame.
        #[config(default = 0.05)]
        sparkle_density: f32,
        /// Brightness multiplier for sparkling cells.
        #[config(default = 1.2)]
        brightness_boost: f32,
        /// Speed of sparkle animation.
        #[config(default = 0.25)]
        speed: f32,
        /// Deterministic seed.
        #[config(default = 42)]
        seed: u64,
        /// Target channel(s).
        #[serde(default)]
        apply_to: VfxTextureTarget,
        /// Noise distribution type.
        #[serde(default)]
        noise_type: NoiseType,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_stochastic_texture_behavior.rs</FILE> - <DESC>V3 stochastic-texture family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
