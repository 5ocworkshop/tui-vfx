// <FILE>tui-vfx-style/src/models/v3/enum_vfx_material_light_behavior.rs</FILE> - <DESC>V3 material-light family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — introduce a grouped V3 material-light family surface for Diffusion, ConcealedLight, and EdgeSheen while preserving the legacy flat variants for current playback.</WCTX>
// <CLOG>Define the V3 material-light family enums by lifting the shared apply-to and behavior policy surface out of the legacy flat shader variants.</CLOG>

//! V3 behavior surface for material-light family shaders.
//!
//! This family covers the calm shell/material-light effects currently exposed as
//! `Diffusion`, `ConcealedLight`, and `EdgeSheen` in the legacy flat catalog.

use crate::models::{ColorConfig, FalloffType};
use serde::{Deserialize, Serialize};

/// Shared channel-target surface for V3 material-light shaders.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxMaterialLightApplyTo {
    /// Apply only to the foreground channel.
    Foreground,
    /// Apply only to the background channel.
    #[default]
    Background,
    /// Apply to both foreground and background.
    Both,
}

/// Diffusion source geometry.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxDiffusionSource {
    /// Diffuse outward from the center.
    #[default]
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Diffusion motion/animation policy.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxDiffusionMode {
    /// Static material diffusion.
    #[default]
    Static,
    WarmDrift,
    CoolDrift,
    Breath,
}

/// Concealed-light source edge.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxConcealedLightSource {
    /// Hidden source along the top edge.
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Concealed-light motion/animation policy.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxConcealedLightMode {
    /// Static concealed lighting.
    #[default]
    Static,
    Pulse,
    Drift,
}

/// Behavior surface for the V3 material-light family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxMaterialLightBehavior {
    /// Soft source-to-surface diffusion for paper, textile, and frosted materials.
    Diffusion {
        /// Source geometry for the diffusion.
        #[serde(default)]
        source: VfxDiffusionSource,
        /// Diffusion tint.
        color: ColorConfig,
        /// Radius in cells.
        #[config(default = 6)]
        radius: u8,
        /// Edge softness.
        #[config(default = 0.55)]
        softness: f32,
        /// Extra edge discipline to keep diffusion from turning into a glow.
        #[config(default = 0.2)]
        edge_firmness: f32,
        /// Falloff curve.
        #[serde(default)]
        falloff: FalloffType,
        /// Blend strength.
        #[config(default = 0.2)]
        intensity: f32,
        /// Target channel(s).
        #[serde(default)]
        apply_to: VfxMaterialLightApplyTo,
        /// Motion policy.
        #[serde(default)]
        mode: VfxDiffusionMode,
        /// Drift/breath speed.
        #[serde(default)]
        drift_speed: f32,
        /// Drift amplitude.
        #[config(default = 0.06)]
        drift_amount: f32,
    },
    /// Hidden-source architectural light for thresholds, seams, and shell depth.
    ConcealedLight {
        /// Concealed source edge.
        #[serde(default)]
        source: VfxConcealedLightSource,
        /// Light tint.
        color: ColorConfig,
        /// Spread in cells.
        #[config(default = 4)]
        spread: u8,
        /// Width of the dark lip/source edge.
        #[config(default = 1)]
        edge_width: u8,
        /// Falloff curve.
        #[serde(default)]
        falloff: FalloffType,
        /// Blend strength.
        #[config(default = 0.18)]
        intensity: f32,
        /// Target channel(s).
        #[serde(default)]
        apply_to: VfxMaterialLightApplyTo,
        /// Motion policy.
        #[serde(default)]
        mode: VfxConcealedLightMode,
        /// Pulse/drift speed.
        #[serde(default)]
        pulse_speed: f32,
        /// Fraction of the source edge held back as a dark lip.
        #[config(default = 0.18)]
        source_cutoff: f32,
    },
    /// Calm perimeter sheen for shells and finished surfaces.
    EdgeSheen {
        /// Sheen tint.
        color: ColorConfig,
        /// Sweep speed multiplier.
        #[config(default = 0.8)]
        speed: f32,
        /// Band width along the perimeter in cells.
        #[config(default = 10)]
        band_width: u16,
        /// Effect thickness measured inward from the edge.
        #[config(default = 2)]
        edge_width: u8,
        /// Blend strength.
        #[config(default = 0.55)]
        intensity: f32,
        /// Extra highlight near corners.
        #[config(default = 0.2)]
        corner_boost: f32,
        /// Target channel(s).
        #[serde(default)]
        apply_to: VfxMaterialLightApplyTo,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_material_light_behavior.rs</FILE> - <DESC>V3 material-light family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
