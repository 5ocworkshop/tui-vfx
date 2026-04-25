// <FILE>tui-vfx-style/src/models/v3/cls_vfx_gradient_reveal_shader.rs</FILE> - <DESC>V3 gradient-reveal family shader surface</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Audit recommendation 2.1 — propagate the new LinearGradientShader fields (apply_to, intensity) through the V3 grouped surface so the round-trip from grouped V3 → legacy SpatialShaderType → grouped V3 stays lossless.</WCTX>
// <CLOG>0.3.0: From<&LinearGradientShader> for VfxGradientRevealShader now copies apply_to and intensity into the LinearGradient behavior. Round-trip parity with the new LinearGradientShader 1.0.0.
// 0.2.0: drop the now-redundant From<RevealDirection> for VfxRevealDirection impl.
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
                apply_to: shader.apply_to,
                intensity: shader.intensity,
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
