// <FILE>tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs</FILE> - <DESC>V3 gradient-reveal family shader surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — RevealDirection and VfxRevealDirection both became type aliases for tui_vfx_geometry::WipeDirection, so the From<RevealDirection> for VfxRevealDirection conversion is now identity (both sides are the same type) and is no longer needed. The legacy-shader conversion can copy the direction directly with no enum-mapping.</WCTX>
// <CLOG>0.2.0: drop the now-redundant From<RevealDirection> for VfxRevealDirection impl and the redundant `.into()` call (both sides are the same WipeDirection type after the audit-recommended unification).
// 0.1.0: initial V3 grouped surface for LinearGradient + RevealWipe</CLOG>

//! V3 family surface for gradient-reveal shaders.
//!
//! This grouped type provides a forward-looking V3 home for the remaining
//! primitive directional fill/reveal treatments that currently live as separate
//! flat variants.

use crate::models::v3::enum_vfx_gradient_reveal_behavior::VfxGradientRevealBehavior;
use crate::models::{Gradient, LinearGradientShader, RevealWipeShader, SpatialShaderType};
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
        // shader.direction is RevealDirection = WipeDirection, and
        // VfxRevealDirection is also WipeDirection after the unification.
        // Direct copy — no enum mapping required.
        Self {
            behavior: VfxGradientRevealBehavior::RevealWipe {
                direction: shader.direction,
            },
        }
    }
}

#[allow(dead_code)]
fn _keep_gradient_type_visible(_gradient: &Gradient) {}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs</FILE> - <DESC>V3 gradient-reveal family shader surface</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
