// <FILE>tui-vfx-style/src/models/v3/enum_vfx_surface_depth_behavior.rs</FILE> - <DESC>V3 surface-depth family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for primitive surface/depth shaders while preserving the legacy AmbientOcclusion, Bevel, and Glow variants for current playback.</WCTX>
// <CLOG>Define the V3 surface-depth family enums and payloads that lift the shared depth/surface treatment behavior out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for surface-depth family shaders.
//!
//! This family groups the primitive/substrate-aligned depth/surface treatments
//! currently exposed as `AmbientOcclusion`, `Bevel`, and `Glow`.

use crate::models::{ColorConfig, FalloffType};
use serde::{Deserialize, Serialize};

/// Edge-selection policy for ambient occlusion style treatments.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxSurfaceDepthEdges {
    /// Bottom and right edges.
    #[default]
    BottomRight,
    /// Top and left edges.
    TopLeft,
    /// All four edges.
    All,
    /// Inward contact shadow from all edges.
    Inner,
}

/// Light-direction policy for bevel-style treatments.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxSurfaceDepthLightDirection {
    /// Light from the top-left.
    #[default]
    TopLeft,
    /// Light from the top-right.
    TopRight,
    /// Light from the bottom-left.
    BottomLeft,
    /// Light from the bottom-right.
    BottomRight,
    /// Light from directly above.
    Top,
    /// Light from directly below.
    Bottom,
    /// Light from the left.
    Left,
    /// Light from the right.
    Right,
}

/// Behavior surface for the V3 surface-depth family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxSurfaceDepthBehavior {
    /// Contact shadow near widget edges.
    AmbientOcclusion {
        /// Maximum darkening intensity.
        #[config(default = 0.3)]
        intensity: f32,
        /// Radius in cells from the edge.
        #[config(default = 2)]
        radius: u8,
        /// Active edges.
        #[serde(default)]
        edges: VfxSurfaceDepthEdges,
        /// Falloff curve.
        #[serde(default)]
        falloff: FalloffType,
        /// Shadow tint.
        #[serde(default = "default_shadow_color")]
        shadow_color: ColorConfig,
    },
    /// 3D embossed/raised edge treatment.
    Bevel {
        /// Simulated light direction.
        #[serde(default)]
        light_direction: VfxSurfaceDepthLightDirection,
        /// Highlight intensity.
        #[config(default = 0.3)]
        highlight_intensity: f32,
        /// Shadow intensity.
        #[config(default = 0.3)]
        shadow_intensity: f32,
        /// Width of the bevel edge in cells.
        #[config(default = 1)]
        edge_width: u8,
    },
    /// Bloom/halo treatment near the widget boundary.
    Glow {
        /// Glow tint.
        color: ColorConfig,
        /// Radius in cells.
        #[config(default = 2)]
        radius: u8,
        /// Falloff curve.
        #[serde(default)]
        falloff: FalloffType,
        /// Overall intensity multiplier.
        #[config(default = 0.6)]
        intensity: f32,
        /// Optional pulse speed.
        #[serde(default)]
        pulse_speed: f32,
    },
}

fn default_shadow_color() -> ColorConfig {
    ColorConfig::Black
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_surface_depth_behavior.rs</FILE> - <DESC>V3 surface-depth family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
