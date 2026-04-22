// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs</FILE> - <DESC>Composed-primitive layer for V3 spatial shaders</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — make the composed-primitive layer explicit so named/composed V2 shader families have a first-class grouped V3 home above the primitive layer.</WCTX>
// <CLOG>Define the VfxSpatialComposedPrimitive enum over grouped composed-family V3 shader surfaces.</CLOG>

//! Composed-primitive layer for V3 spatial shaders.

use crate::models::v3::{
    VfxCursorShader, VfxGuidanceCueShader, VfxMaterialLightShader, VfxProgressEmphasisShader,
    VfxStochasticTextureShader, VfxStripeMotionShader, VfxTravelingBandShader,
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
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_composed_primitive.rs</FILE> - <DESC>Composed-primitive layer for V3 spatial shaders</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
