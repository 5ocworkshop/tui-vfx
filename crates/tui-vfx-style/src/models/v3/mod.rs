// <FILE>tui-vfx-style/src/models/v3/mod.rs</FILE> - <DESC>Parallel V3 model surfaces for tui-vfx-style</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — start creating real V3-side family model files as each family is analyzed, instead of only tagging legacy flat variants for later.</WCTX>
// <CLOG>Expose the first V3 family module (traveling-band / sweep) alongside the legacy model surface.</CLOG>

//! Grouped V3 model surfaces for `tui-vfx-style`.
//!
//! These modules define the family-oriented V3 surface used during the style
//! cutover. The legacy flat `SpatialShaderType` catalog stays intact for current
//! playback and downstream compatibility until the broader migration finishes.

pub mod cls_vfx_color_fade_shader;
pub mod cls_vfx_color_shift_shader;
pub mod cls_vfx_cursor_shader;
pub mod cls_vfx_edge_distortion_shader;
pub mod cls_vfx_gradient_reveal_shader;
pub mod cls_vfx_guidance_cue_shader;
pub mod cls_vfx_material_light_shader;
pub mod cls_vfx_modifier_window_shader;
pub mod cls_vfx_motion_field_shader;
pub mod cls_vfx_progress_emphasis_shader;
pub mod cls_vfx_rainbow_cycle_shader;
pub mod cls_vfx_stochastic_texture_shader;
pub mod cls_vfx_stripe_motion_shader;
pub mod cls_vfx_surface_depth_shader;
pub mod cls_vfx_traveling_band_shader;
pub mod enum_try_lower_v3_spatial_shader_error;
pub mod enum_try_lower_v3_style_effect_error;
pub mod enum_vfx_cursor_behavior;
pub mod enum_vfx_edge_distortion_behavior;
pub mod enum_vfx_gradient_reveal_behavior;
pub mod enum_vfx_guidance_cue_behavior;
pub mod enum_vfx_material_light_behavior;
pub mod enum_vfx_motion_field_behavior;
pub mod enum_vfx_progress_emphasis_behavior;
pub mod enum_vfx_spatial_composed_primitive;
pub mod enum_vfx_spatial_primitive;
pub mod enum_vfx_spatial_shader_family;
pub mod enum_vfx_stochastic_texture_behavior;
pub mod enum_vfx_stripe_motion_behavior;
pub mod enum_vfx_style_effect_family;
pub mod enum_vfx_style_effect_value;
pub mod enum_vfx_surface_depth_behavior;
pub mod enum_vfx_traveling_band_behavior;
pub mod fnc_lower_legacy_spatial_shader;
pub mod fnc_try_lower_v3_spatial_shader_family;

pub use cls_vfx_color_fade_shader::VfxColorFadeShader;
pub use cls_vfx_color_shift_shader::VfxColorShiftShader;
pub use cls_vfx_cursor_shader::VfxCursorShader;
pub use cls_vfx_edge_distortion_shader::VfxEdgeDistortionShader;
pub use cls_vfx_gradient_reveal_shader::VfxGradientRevealShader;
pub use cls_vfx_guidance_cue_shader::VfxGuidanceCueShader;
pub use cls_vfx_material_light_shader::VfxMaterialLightShader;
pub use cls_vfx_modifier_window_shader::VfxModifierWindowShader;
pub use cls_vfx_motion_field_shader::VfxMotionFieldShader;
pub use cls_vfx_progress_emphasis_shader::VfxProgressEmphasisShader;
pub use cls_vfx_rainbow_cycle_shader::VfxRainbowCycleShader;
pub use cls_vfx_stochastic_texture_shader::VfxStochasticTextureShader;
pub use cls_vfx_stripe_motion_shader::VfxStripeMotionShader;
pub use cls_vfx_surface_depth_shader::VfxSurfaceDepthShader;
pub use cls_vfx_traveling_band_shader::VfxTravelingBandShader;
pub use enum_try_lower_v3_spatial_shader_error::TryLowerV3SpatialShaderError;
pub use enum_try_lower_v3_style_effect_error::TryLowerV3StyleEffectError;
pub use enum_vfx_cursor_behavior::{VfxCursorMode, VfxCursorPrimary, VfxCursorTrail};
pub use enum_vfx_edge_distortion_behavior::{VfxEdgeDistortionAxis, VfxEdgeDistortionBehavior};
pub use enum_vfx_gradient_reveal_behavior::{VfxGradientRevealBehavior, VfxRevealDirection};
pub use enum_vfx_guidance_cue_behavior::{
    VfxAffordanceWakeZone, VfxFocusFieldShape, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior,
    VfxWayfindingNode,
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
pub use enum_vfx_spatial_composed_primitive::VfxSpatialComposedPrimitive;
pub use enum_vfx_spatial_primitive::VfxSpatialPrimitive;
pub use enum_vfx_spatial_shader_family::VfxSpatialShaderFamily;
pub use enum_vfx_stochastic_texture_behavior::{
    VfxStochasticTextureBehavior, VfxTextureSegmentMode, VfxTextureTarget,
};
pub use enum_vfx_stripe_motion_behavior::VfxStripeMotionBehavior;
pub use enum_vfx_style_effect_family::VfxStyleEffectFamily;
pub use enum_vfx_style_effect_value::VfxStyleEffectValue;
pub use enum_vfx_surface_depth_behavior::{
    VfxSurfaceDepthBehavior, VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection,
};
pub use enum_vfx_traveling_band_behavior::{
    VfxTracePathTailMode, VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection,
};
pub use fnc_lower_legacy_spatial_shader::lower_legacy_spatial_shader;
pub use fnc_try_lower_v3_spatial_shader_family::try_lower_v3_spatial_shader_family;

#[cfg(test)]
mod test_try_lower_v3_spatial_shader_family;
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
mod test_vfx_spatial_shader_family;
#[cfg(test)]
mod test_vfx_stochastic_texture_shader;
#[cfg(test)]
mod test_vfx_stripe_motion_shader;
#[cfg(test)]
mod test_vfx_style_effect_value;
#[cfg(test)]
mod test_vfx_surface_depth_shader;
#[cfg(test)]
mod test_vfx_traveling_band_shader;

// <FILE>tui-vfx-style/src/models/v3/mod.rs</FILE> - <DESC>Parallel V3 model surfaces for tui-vfx-style</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
