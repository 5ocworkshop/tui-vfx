// <FILE>tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs</FILE> - <DESC>V3 gradient-reveal family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 surface for the remaining primitive gradient/reveal shaders so LinearGradient and RevealWipe become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxGradientRevealShader plus conversion helpers from the legacy gradient/reveal variants and SpatialShaderType.</CLOG>

//! V3 family surface for gradient-reveal shaders.
//!
//! This grouped type provides a forward-looking V3 home for the remaining
//! primitive directional fill/reveal treatments that currently live as separate
//! flat variants.

use crate::models::v3::enum_vfx_gradient_reveal_behavior::{
    VfxGradientRevealBehavior, VfxRevealDirection,
};
use crate::models::{
    Gradient, LinearGradientShader, RevealDirection, RevealWipeShader, SpatialShaderType,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for gradient-reveal shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxGradientRevealShader {
    /// Behavior/configuration surface for the chosen gradient-reveal family member.
    pub behavior: VfxGradientRevealBehavior,
}

impl VfxGradientRevealShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// gradient-reveal family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::LinearGradient(shader) => Some(Self::from(shader)),
            SpatialShaderType::RevealWipe(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&LinearGradientShader> for VfxGradientRevealShader {
    fn from(shader: &LinearGradientShader) -> Self {
        Self {
            behavior: VfxGradientRevealBehavior::LinearGradient {
                gradient: shader.gradient.clone(),
                angle_deg: shader.angle_deg,
            },
        }
    }
}

impl From<&RevealWipeShader> for VfxGradientRevealShader {
    fn from(shader: &RevealWipeShader) -> Self {
        Self {
            behavior: VfxGradientRevealBehavior::RevealWipe {
                direction: shader.direction.into(),
            },
        }
    }
}

impl From<RevealDirection> for VfxRevealDirection {
    fn from(value: RevealDirection) -> Self {
        match value {
            RevealDirection::LeftToRight => Self::LeftToRight,
            RevealDirection::RightToLeft => Self::RightToLeft,
            RevealDirection::TopToBottom => Self::TopToBottom,
            RevealDirection::BottomToTop => Self::BottomToTop,
        }
    }
}

#[allow(dead_code)]
fn _keep_gradient_type_visible(_gradient: &Gradient) {}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs</FILE> - <DESC>V3 gradient-reveal family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
