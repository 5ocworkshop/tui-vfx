// <FILE>tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs</FILE> - <DESC>V3 edge-distortion family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 surface for primitive edge-distortion shaders so GlitchLines, ChromaticEdge, and SubCellShake become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxEdgeDistortionShader plus conversion helpers from the legacy edge-distortion variants and SpatialShaderType.</CLOG>

//! V3 family surface for edge-distortion shaders.
//!
//! This grouped type provides a forward-looking V3 home for the primitive edge
//! glitch and micro-distortion treatments that currently live as separate flat
//! variants.

use crate::models::v3::enum_vfx_edge_distortion_behavior::{
    VfxEdgeDistortionAxis, VfxEdgeDistortionBehavior,
};
use crate::models::{
    ChromaticEdgeShader, GlitchLinesShader, ShakeAxis, SpatialShaderType, SubCellShakeShader,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for edge-distortion shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxEdgeDistortionShader {
    /// Behavior/configuration surface for the chosen edge-distortion family member.
    pub behavior: VfxEdgeDistortionBehavior,
}

impl VfxEdgeDistortionShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// edge-distortion family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::GlitchLines(shader) => Some(Self::from(shader)),
            SpatialShaderType::ChromaticEdge(shader) => Some(Self::from(shader)),
            SpatialShaderType::SubCellShake(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&GlitchLinesShader> for VfxEdgeDistortionShader {
    fn from(shader: &GlitchLinesShader) -> Self {
        Self {
            behavior: VfxEdgeDistortionBehavior::GlitchLines {
                seed: shader.seed,
                intensity: shader.intensity,
                max_lines: shader.max_lines,
                speed: shader.speed,
                flash_chance: shader.flash_chance,
                pulse_color: shader.pulse_color,
                pulse_speed: shader.pulse_speed,
                italic_on_flash: shader.italic_on_flash,
                flash_hold: shader.flash_hold,
                noise_type: shader.noise_type,
            },
        }
    }
}

impl From<&ChromaticEdgeShader> for VfxEdgeDistortionShader {
    fn from(shader: &ChromaticEdgeShader) -> Self {
        Self {
            behavior: VfxEdgeDistortionBehavior::ChromaticEdge {
                intensity: shader.intensity,
                edge_width: shader.edge_width,
                horizontal: shader.horizontal,
            },
        }
    }
}

impl From<&SubCellShakeShader> for VfxEdgeDistortionShader {
    fn from(shader: &SubCellShakeShader) -> Self {
        Self {
            behavior: VfxEdgeDistortionBehavior::SubCellShake {
                amplitude: shader.amplitude,
                frequency: shader.frequency,
                axis: shader.axis.into(),
                chromatic: shader.chromatic,
                seed: shader.seed,
                edge_only: shader.edge_only,
                edge_width: shader.edge_width,
            },
        }
    }
}

impl From<ShakeAxis> for VfxEdgeDistortionAxis {
    fn from(value: ShakeAxis) -> Self {
        match value {
            ShakeAxis::Horizontal => Self::Horizontal,
            ShakeAxis::Vertical => Self::Vertical,
            ShakeAxis::Both => Self::Both,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_edge_distortion_shader.rs</FILE> - <DESC>V3 edge-distortion family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
