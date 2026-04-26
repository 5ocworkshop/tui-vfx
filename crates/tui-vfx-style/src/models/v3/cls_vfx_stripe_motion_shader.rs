// <FILE>tui-vfx-style/src/models/v3/cls_vfx_stripe_motion_shader.rs</FILE> - <DESC>V3 stripe-motion family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create an explicit V3 home for BarberPole so the future-decision bucket has no remaining ungrouped style holdouts.</WCTX>
// <CLOG>Introduce VfxStripeMotionShader plus conversion helpers from BarberPole and SpatialShaderType.</CLOG>

//! V3 family surface for stripe-motion shaders.
//!
//! This grouped type provides a forward-looking V3 home for the animated stripe
//! treatment currently exposed as the `BarberPole` flat variant.

use crate::models::v3::enum_vfx_stripe_motion_behavior::VfxStripeMotionBehavior;
use crate::models::{BarberPoleShader, SpatialShaderType};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for stripe-motion shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxStripeMotionShader {
    /// Behavior/configuration surface for the chosen stripe-motion family member.
    pub behavior: VfxStripeMotionBehavior,
}

impl VfxStripeMotionShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// stripe-motion family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::BarberPole(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&BarberPoleShader> for VfxStripeMotionShader {
    fn from(shader: &BarberPoleShader) -> Self {
        Self {
            behavior: VfxStripeMotionBehavior::BarberPole {
                speed: shader.speed,
                stripe_width: shader.stripe_width,
                gap_width: shader.gap_width,
                color: shader.color,
            },
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_stripe_motion_shader.rs</FILE> - <DESC>V3 stripe-motion family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
