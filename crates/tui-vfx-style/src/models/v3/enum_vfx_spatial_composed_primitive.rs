// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs</FILE> - <DESC>Composed-primitive layer for V3 spatial shaders</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — make the composed-primitive layer explicit and expose a stable family label for runtime wiring and inspection once the grouped V3 family surfaces exist.</WCTX>
// <CLOG>0.2.0: add family_label() for inspection/runtime seams while keeping the grouped composed surface stable.
// 0.1.0: define the VfxSpatialComposedPrimitive enum over grouped composed-family V3 shader surfaces.</CLOG>

//! Composed-primitive layer for V3 spatial shaders.
//!
//! This enum is the composed half of the central V3 style-family seam. It holds
//! the earned named factories, special-case composed families, and other V2-era
//! authored surfaces that the plan decided should not be treated as true
//! primitives.

use crate::models::v3::{
    VfxCursorShader, VfxGuidanceCueShader, VfxMaterialLightShader, VfxModifierWindowShader,
    VfxProgressEmphasisShader, VfxRainbowCycleShader, VfxStochasticTextureShader,
    VfxStripeMotionShader, VfxTravelingBandShader,
};
use serde::{Deserialize, Serialize};

/// Composed-primitive V3 representation for spatial shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "composed", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxSpatialComposedPrimitive {
    /// Traveling-band / sweep composed family.
    TravelingBand(VfxTravelingBandShader),
    /// Progress / emphasis composed family.
    ProgressEmphasis(VfxProgressEmphasisShader),
    /// Material-light composed family.
    MaterialLight(VfxMaterialLightShader),
    /// Guidance-cue composed family.
    GuidanceCue(VfxGuidanceCueShader),
    /// Stochastic-texture composed family.
    StochasticTexture(VfxStochasticTextureShader),
    /// Cursor composed family.
    Cursor(VfxCursorShader),
    /// Stripe-motion composed family.
    StripeMotion(VfxStripeMotionShader),
    /// Time-windowed text modifier composed family.
    ModifierWindow(VfxModifierWindowShader),
    /// Rainbow foreground hue rotation composed family.
    RainbowCycle(VfxRainbowCycleShader),
}

impl VfxSpatialComposedPrimitive {
    /// Stable composed-family label for inspection/debug surfaces.
    pub fn family_label(&self) -> &'static str {
        match self {
            Self::TravelingBand(_) => "traveling_band",
            Self::ProgressEmphasis(_) => "progress_emphasis",
            Self::MaterialLight(_) => "material_light",
            Self::GuidanceCue(_) => "guidance_cue",
            Self::StochasticTexture(_) => "stochastic_texture",
            Self::Cursor(_) => "cursor",
            Self::StripeMotion(_) => "stripe_motion",
            Self::ModifierWindow(_) => "modifier_window",
            Self::RainbowCycle(_) => "rainbow_cycle",
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs</FILE> - <DESC>Composed-primitive layer for V3 spatial shaders</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
