// <FILE>tui-vfx-style/src/models/v3/mod.rs</FILE> - <DESC>Parallel V3 model surfaces for tui-vfx-style</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — start creating real V3-side family model files as each family is analyzed, instead of only tagging legacy flat variants for later.</WCTX>
// <CLOG>Expose the first V3 family module (traveling-band / sweep) alongside the legacy model surface.</CLOG>

//! Parallel V3 model surfaces for `tui-vfx-style`.
//!
//! These modules are the forward-looking family/grouped surfaces used during the
//! V3 migration. The legacy flat `SpatialShaderType` surface stays intact for
//! current playback and downstream compatibility until the broader cutover is
//! complete.

pub mod cls_vfx_guidance_cue_shader;
pub mod cls_vfx_material_light_shader;
pub mod cls_vfx_progress_emphasis_shader;
pub mod cls_vfx_traveling_band_shader;
pub mod enum_vfx_guidance_cue_behavior;
pub mod enum_vfx_material_light_behavior;
pub mod enum_vfx_progress_emphasis_behavior;
pub mod enum_vfx_traveling_band_behavior;

pub use cls_vfx_guidance_cue_shader::VfxGuidanceCueShader;
pub use cls_vfx_material_light_shader::VfxMaterialLightShader;
pub use cls_vfx_progress_emphasis_shader::VfxProgressEmphasisShader;
pub use cls_vfx_traveling_band_shader::VfxTravelingBandShader;
pub use enum_vfx_guidance_cue_behavior::{
    VfxAffordanceWakeZone, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior, VfxWayfindingNode,
};
pub use enum_vfx_material_light_behavior::{
    VfxConcealedLightMode, VfxConcealedLightSource, VfxDiffusionMode, VfxDiffusionSource,
    VfxMaterialLightApplyTo, VfxMaterialLightBehavior,
};
pub use enum_vfx_progress_emphasis_behavior::{
    VfxProgressEmphasisApplyTo, VfxProgressEmphasisDirection, VfxProgressEmphasisMode,
    VfxProgressEmphasisRowMask, VfxProgressEmphasisTextContrast,
};
pub use enum_vfx_traveling_band_behavior::{
    VfxTracePathTailMode, VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection,
};

#[cfg(test)]
mod test_vfx_guidance_cue_shader;
#[cfg(test)]
mod test_vfx_material_light_shader;
#[cfg(test)]
mod test_vfx_progress_emphasis_shader;
#[cfg(test)]
mod test_vfx_traveling_band_shader;

// <FILE>tui-vfx-style/src/models/v3/mod.rs</FILE> - <DESC>Parallel V3 model surfaces for tui-vfx-style</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
