// <FILE>tui-vfx-style/src/models/v3/enum_vfx_stripe_motion_behavior.rs</FILE> - <DESC>V3 stripe-motion family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a minimal grouped V3 home for BarberPole while preserving the legacy flat variant for current playback.</WCTX>
// <CLOG>Define the V3 stripe-motion behavior surface for the remaining animated stripe holdout.</CLOG>

//! V3 behavior surface for stripe-motion shaders.
//!
//! This family currently contains the legacy `BarberPole` effect, which still
//! benefits from an explicit grouped V3 home even though it remains a single
//! member family for now.

use crate::models::ColorConfig;
use serde::{Deserialize, Serialize};

/// Behavior surface for the V3 stripe-motion family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxStripeMotionBehavior {
    /// Diagonal moving stripes like a barber-pole loading treatment.
    BarberPole {
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Stripe width in cells.
        #[config(default = 2)]
        stripe_width: u16,
        /// Gap width in cells.
        #[config(default = 2)]
        gap_width: u16,
        /// Stripe color.
        color: ColorConfig,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_stripe_motion_behavior.rs</FILE> - <DESC>V3 stripe-motion family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
