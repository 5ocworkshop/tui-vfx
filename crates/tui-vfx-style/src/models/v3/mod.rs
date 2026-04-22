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

pub mod cls_vfx_cursor_shader;
pub mod cls_vfx_edge_distortion_shader;
pub mod cls_vfx_gradient_reveal_shader;
pub mod cls_vfx_guidance_cue_shader;
pub mod cls_vfx_material_light_shader;
pub mod cls_vfx_motion_field_shader;
pub mod cls_vfx_progress_emphasis_shader;
pub mod cls_vfx_stochastic_texture_shader;
pub mod cls_vfx_stripe_motion_shader;
pub mod cls_vfx_surface_depth_shader;
pub mod cls_vfx_traveling_band_shader;
pub mod enum_vfx_cursor_behavior;
pub mod enum_vfx_edge_distortion_behavior;
pub mod enum_vfx_gradient_reveal_behavior;
pub mod enum_vfx_guidance_cue_behavior;
pub mod enum_vfx_material_light_behavior;
pub mod enum_vfx_motion_field_behavior;
pub mod enum_vfx_progress_emphasis_behavior;
pub mod enum_vfx_stochastic_texture_behavior;
pub mod enum_vfx_stripe_motion_behavior;
pub mod enum_vfx_surface_depth_behavior;
pub mod enum_vfx_traveling_band_behavior;

pub use cls_vfx_cursor_shader::VfxCursorShader;
pub use cls_vfx_edge_distortion_shader::VfxEdgeDistortionShader;
pub use cls_vfx_gradient_reveal_shader::VfxGradientRevealShader;
pub use cls_vfx_guidance_cue_shader::VfxGuidanceCueShader;
pub use cls_vfx_material_light_shader::VfxMaterialLightShader;
pub use cls_vfx_motion_field_shader::VfxMotionFieldShader;
pub use cls_vfx_progress_emphasis_shader::VfxProgressEmphasisShader;
pub use cls_vfx_stochastic_texture_shader::VfxStochasticTextureShader;
pub use cls_vfx_stripe_motion_shader::VfxStripeMotionShader;
pub use cls_vfx_surface_depth_shader::VfxSurfaceDepthShader;
pub use cls_vfx_traveling_band_shader::VfxTravelingBandShader;
pub use enum_vfx_cursor_behavior::{VfxCursorMode, VfxCursorPrimary, VfxCursorTrail};
pub use enum_vfx_edge_distortion_behavior::{VfxEdgeDistortionAxis, VfxEdgeDistortionBehavior};
pub use enum_vfx_gradient_reveal_behavior::{VfxGradientRevealBehavior, VfxRevealDirection};
pub use enum_vfx_guidance_cue_behavior::{
    VfxAffordanceWakeZone, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior, VfxWayfindingNode,
};
pub use enum_vfx_material_light_behavior::{
    VfxConcealedLightMode, VfxConcealedLightSource, VfxDiffusionMode, VfxDiffusionSource,
    VfxMaterialLightApplyTo, VfxMaterialLightBehavior,
};
pub use enum_vfx_motion_field_behavior::{VfxMotionFieldBehavior, VfxMotionFieldDirection};
pub use enum_vfx_progress_emphasis_behavior::{
    VfxProgressEmphasisApplyTo, VfxProgressEmphasisDirection, VfxProgressEmphasisMode,
    VfxProgressEmphasisRowMask, VfxProgressEmphasisTextContrast,
};
pub use enum_vfx_stochastic_texture_behavior::{
    VfxStochasticTextureBehavior, VfxTextureSegmentMode, VfxTextureTarget,
};
pub use enum_vfx_stripe_motion_behavior::VfxStripeMotionBehavior;
pub use enum_vfx_surface_depth_behavior::{
    VfxSurfaceDepthBehavior, VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection,
};
pub use enum_vfx_traveling_band_behavior::{
    VfxTracePathTailMode, VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection,
};

#[cfg(test)]
mod test_vfx_cursor_shader;
#[cfg(test)]
mod test_vfx_edge_distortion_shader;
#[cfg(test)]
mod test_vfx_gradient_reveal_shader;
#[cfg(test)]
mod test_vfx_guidance_cue_shader;
#[cfg(test)]
mod test_vfx_material_light_shader;
#[cfg(test)]
mod test_vfx_motion_field_shader;
#[cfg(test)]
mod test_vfx_progress_emphasis_shader;
#[cfg(test)]
mod test_vfx_stochastic_texture_shader;
#[cfg(test)]
mod test_vfx_stripe_motion_shader;
#[cfg(test)]
mod test_vfx_surface_depth_shader;
#[cfg(test)]
mod test_vfx_traveling_band_shader;

// <FILE>tui-vfx-style/src/models/v3/mod.rs</FILE> - <DESC>Parallel V3 model surfaces for tui-vfx-style</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
