// <FILE>tui-vfx-style/src/models/v3/cls_vfx_surface_depth_shader.rs</FILE> - <DESC>V3 surface-depth family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 surface for primitive surface/depth shaders so AmbientOcclusion, Bevel, and Glow become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxSurfaceDepthShader plus conversion helpers from the legacy surface/depth variants and SpatialShaderType.</CLOG>

//! V3 family surface for surface-depth shaders.
//!
//! This grouped type provides a forward-looking V3 home for the primitive
//! depth/surface treatments that currently live as separate flat variants.

use crate::models::v3::enum_vfx_surface_depth_behavior::{
    VfxSurfaceDepthBehavior, VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection,
};
use crate::models::{
    AOEdges, AmbientOcclusionShader, BevelShader, GlowShader, LightDirection, SpatialShaderType,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for surface-depth shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxSurfaceDepthShader {
    /// Behavior/configuration surface for the chosen surface-depth family member.
    pub behavior: VfxSurfaceDepthBehavior,
}

impl VfxSurfaceDepthShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// surface-depth family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::AmbientOcclusion(shader) => Some(Self::from(shader)),
            SpatialShaderType::Bevel(shader) => Some(Self::from(shader)),
            SpatialShaderType::Glow(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&AmbientOcclusionShader> for VfxSurfaceDepthShader {
    fn from(shader: &AmbientOcclusionShader) -> Self {
        Self {
            behavior: VfxSurfaceDepthBehavior::AmbientOcclusion {
                intensity: shader.intensity,
                radius: shader.radius,
                edges: shader.edges.into(),
                falloff: shader.falloff,
                shadow_color: shader.shadow_color.clone(),
            },
        }
    }
}

impl From<&BevelShader> for VfxSurfaceDepthShader {
    fn from(shader: &BevelShader) -> Self {
        Self {
            behavior: VfxSurfaceDepthBehavior::Bevel {
                light_direction: shader.light_direction.into(),
                highlight_intensity: shader.highlight_intensity,
                shadow_intensity: shader.shadow_intensity,
                edge_width: shader.edge_width,
            },
        }
    }
}

impl From<&GlowShader> for VfxSurfaceDepthShader {
    fn from(shader: &GlowShader) -> Self {
        Self {
            behavior: VfxSurfaceDepthBehavior::Glow {
                color: shader.color.clone(),
                radius: shader.radius,
                falloff: shader.falloff,
                intensity: shader.intensity,
                pulse_speed: shader.pulse_speed,
            },
        }
    }
}

impl From<AOEdges> for VfxSurfaceDepthEdges {
    fn from(value: AOEdges) -> Self {
        match value {
            AOEdges::BottomRight => Self::BottomRight,
            AOEdges::TopLeft => Self::TopLeft,
            AOEdges::All => Self::All,
            AOEdges::Inner => Self::Inner,
        }
    }
}

impl From<LightDirection> for VfxSurfaceDepthLightDirection {
    fn from(value: LightDirection) -> Self {
        match value {
            LightDirection::TopLeft => Self::TopLeft,
            LightDirection::TopRight => Self::TopRight,
            LightDirection::BottomLeft => Self::BottomLeft,
            LightDirection::BottomRight => Self::BottomRight,
            LightDirection::Top => Self::Top,
            LightDirection::Bottom => Self::Bottom,
            LightDirection::Left => Self::Left,
            LightDirection::Right => Self::Right,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_surface_depth_shader.rs</FILE> - <DESC>V3 surface-depth family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
