// <FILE>tui-vfx-style/src/models/mod.rs</FILE> - <DESC>Style models module</DESC>
// <VERS>VERSION: 2.7.0</VERS>
// <WCTX>Glyph rendering framework Phase 5: register cls_water_field_signal sibling that exposes TerminalWaterShader's per-cell field as a mixed_signals::Signal.</WCTX>
// <CLOG>2.7.0: register cls_water_field_signal module + re-export WaterFieldSignal so downstream consumers (ScalarFieldGlyphFilter, water glyph recipes) can plug water into the glyph encoder pipeline.</CLOG>

//! Style/shader model surface for `tui-vfx-style`.
//!
//! ## Migration note
//!
//! This module currently re-exports a flat style/shader model surface centered
//! around `SpatialShaderType`. Under the V3 plan, that flat surface is expected
//! to be reclassified into deeper primitive families plus earned named
//! factories/presets. The current export layout is therefore the live migration
//! surface, not the final conceptual claim.
//!
//! See:
//! - `docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md` (Decision 2)
//! - `docs/design/tui-vfx-v3-capability-catalog.md`
//! - `docs/design/tui-vfx-v3-style-model-restructure-inventory.md`
//!
//! Real V3-side family files now live under [`crate::models::v3`]. Those
//! parallel modules should grow as each family is actively migrated, while the
//! legacy flat surface remains available for current playback and cutover
//! compatibility.

pub mod cls_affordance_wake_shader;
pub mod cls_ambient_occlusion_shader;
pub mod cls_barber_pole_shader;
pub mod cls_bevel_shader;
pub mod cls_bindable_string;
pub mod cls_bindable_u16;
pub mod cls_blend_mode;
pub mod cls_border_sweep_shader;
pub mod cls_chromatic_edge_shader;
pub mod cls_color_config;
pub mod cls_color_fade_shader;
pub mod cls_color_ramp;
pub mod cls_color_space;
pub mod cls_concealed_light_shader;
pub mod cls_cursor_shader;
pub mod cls_diffusion_shader;
pub mod cls_edge_sheen_shader;
pub mod cls_fade_effect;
pub mod cls_fade_spec;
pub mod cls_falloff_type;
pub mod cls_fire_field_signal;
pub mod cls_focus_field_shader;
pub mod cls_focused_row_gradient_shader;
pub mod cls_glisten_band_shader;
pub mod cls_glitch_lines_shader;
pub mod cls_glow_shader;
pub mod cls_gradient;
pub mod cls_gradient_lut;
pub mod cls_highlighter_shader;
pub mod cls_linear_gradient_shader;
pub mod cls_modifier_window_shader;
pub mod cls_neon_flicker_shader;
pub mod cls_noise_type;
pub mod cls_orbit_shader;
pub mod cls_pulse_wave_shader;
pub mod cls_radar_shader;
pub mod cls_radial_spiral_shader;
pub mod cls_rainbow_cycle_shader;
pub mod cls_reflect_shader;
pub mod cls_reveal_wipe_shader;
pub mod cls_signal_color;
pub mod cls_spatial_shader_type;
pub mod cls_stochastic_sparkle_shader;
pub mod cls_style_config;
pub mod cls_style_effect;
pub mod cls_style_layer;
pub mod cls_style_region;
pub mod cls_style_transition;
pub mod cls_sub_cell_shake_shader;
pub mod cls_terminal_fire_shader;
pub mod cls_terminal_water_shader;
pub mod cls_trace_common;
pub mod cls_trace_path_shader;
pub mod cls_trace_propagation_shader;
pub mod cls_water_field_signal;
pub mod cls_wayfinding_node_shader;
pub mod fnc_apply_style_effects_to_scene;
pub mod fnc_style_region_bounding_rect;
pub mod fnc_style_region_deserialize;
pub mod fnc_style_region_resolved;
pub mod fnc_style_region_schema;
pub mod fnc_style_region_should_style;
pub mod v3;

#[cfg(test)]
pub(crate) mod test_support;
pub use cls_affordance_wake_shader::{
    AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone,
};
pub use cls_ambient_occlusion_shader::{AOEdges, AmbientOcclusionShader};
pub use cls_barber_pole_shader::{BarberPoleApplyTo, BarberPoleShader};
pub use cls_bevel_shader::{BevelShader, LightDirection};
pub use cls_bindable_string::BindableString;
pub use cls_bindable_u16::BindableU16;
pub use cls_blend_mode::BlendMode;
pub use cls_border_sweep_shader::BorderSweepShader;
pub use cls_chromatic_edge_shader::ChromaticEdgeShader;
pub use cls_color_config::ColorConfig;
pub use cls_color_fade_shader::ColorFadeShader;
pub use cls_color_ramp::{ColorRamp, ColorStop};
pub use cls_color_space::ColorSpace;
pub use cls_concealed_light_shader::{
    ConcealedLightApplyTo, ConcealedLightMode, ConcealedLightShader, ConcealedLightSource,
};
pub use cls_cursor_shader::{
    CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
};
pub use cls_diffusion_shader::{DiffusionApplyTo, DiffusionMode, DiffusionShader, DiffusionSource};
pub use cls_edge_sheen_shader::{EdgeSheenApplyTo, EdgeSheenShader};
pub use cls_fade_effect::{FadeDirection, FadeEffect, FadeToBlack, FadeToColor, fade_effect};
pub use cls_fade_spec::{FadeApplyTo, FadeChain, FadeSegment, FadeSpec, FadeTarget};
pub use cls_falloff_type::FalloffType;
pub use cls_fire_field_signal::FireFieldSignal;
pub use cls_focus_field_shader::{FocusFieldApplyTo, FocusFieldShader, FocusFieldShape};
pub use cls_focused_row_gradient_shader::{ApplyToColor, FocusedRowGradientShader};
pub use cls_glisten_band_shader::{GlistenApplyTo, GlistenBandShader, GlistenDirection};
pub use cls_glitch_lines_shader::GlitchLinesShader;
pub use cls_glow_shader::GlowShader;
pub use cls_gradient::Gradient;
pub use cls_gradient_lut::GradientLUT;
pub use cls_highlighter_shader::{
    HighlighterApplyTo, HighlighterDirection, HighlighterMode, HighlighterRowMask,
    HighlighterShader, TextContrast,
};
pub use cls_linear_gradient_shader::{LinearGradientApplyTo, LinearGradientShader};
pub use cls_modifier_window_shader::ModifierWindowShader;
pub use cls_neon_flicker_shader::{NeonFlickerShader, SegmentMode};
pub use cls_noise_type::NoiseType;
pub use cls_orbit_shader::OrbitShader;
pub use cls_pulse_wave_shader::{PulseWaveShader, WaveDirection};
pub use cls_radar_shader::RadarShader;
pub use cls_radial_spiral_shader::RadialSpiralShader;
pub use cls_rainbow_cycle_shader::RainbowCycleShader;
pub use cls_reflect_shader::ReflectShader;
pub use cls_reveal_wipe_shader::{RevealDirection, RevealWipeShader};
pub use cls_signal_color::SignalColor;
pub use cls_spatial_shader_type::SpatialShaderType;
pub use cls_stochastic_sparkle_shader::{SparkleTarget, StochasticSparkleShader};
pub use cls_style_config::{ModifierConfig, StyleConfig};
pub use cls_style_effect::StyleEffect;
pub use cls_style_layer::StyleLayer;
pub use cls_style_region::{CellCoord, ModuloAxis, StyleRegion};
pub use cls_style_transition::StyleTransition;
pub use cls_sub_cell_shake_shader::{ShakeAxis, SubCellShakeShader};
pub use cls_terminal_fire_shader::{
    FireApplyTo, FireMode, FirePalette, FireSparkConfig, TerminalFireShader,
};
pub use cls_terminal_water_shader::{
    TerminalWaterShader, WaterApplyTo, WaterRippleEmitter, WaterWakeSource, WaterWaveMode,
};
pub use cls_trace_common::{TraceApplyTo, TraceOrigin, TracePoint, TracePolyline};
pub use cls_trace_path_shader::TracePathShader;
pub use cls_trace_propagation_shader::TracePropagationShader;
pub use cls_water_field_signal::WaterFieldSignal;
pub use cls_wayfinding_node_shader::{WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader};
pub use fnc_apply_style_effects_to_scene::apply_style_effects_to_scene;
pub use tui_vfx_geometry::easing::EasingType;
pub use v3::{
    TryLowerV3SpatialShaderError, TryLowerV3StyleEffectError, VfxAffordanceWakeZone,
    VfxConcealedLightMode, VfxConcealedLightSource, VfxCursorMode, VfxCursorPrimary,
    VfxCursorShader, VfxCursorTrail, VfxDiffusionMode, VfxDiffusionSource, VfxEdgeDistortionAxis,
    VfxEdgeDistortionBehavior, VfxEdgeDistortionShader, VfxFocusFieldShape,
    VfxGradientRevealBehavior, VfxGradientRevealShader, VfxGuidanceCueApplyTo,
    VfxGuidanceCueBehavior, VfxGuidanceCueShader, VfxMaterialLightApplyTo,
    VfxMaterialLightBehavior, VfxMaterialLightShader, VfxMotionFieldBehavior,
    VfxMotionFieldDirection, VfxMotionFieldShader, VfxProgressEmphasisApplyTo,
    VfxProgressEmphasisDirection, VfxProgressEmphasisMode, VfxProgressEmphasisRowMask,
    VfxProgressEmphasisShader, VfxProgressEmphasisTextContrast, VfxRevealDirection,
    VfxSpatialComposedPrimitive, VfxSpatialPrimitive, VfxSpatialShaderFamily,
    VfxStochasticTextureBehavior, VfxStochasticTextureShader, VfxStripeMotionBehavior,
    VfxStripeMotionShader, VfxStyleEffectFamily, VfxStyleEffectValue, VfxSurfaceDepthBehavior,
    VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection, VfxSurfaceDepthShader,
    VfxTextureSegmentMode, VfxTextureTarget, VfxTracePathTailMode, VfxTravelingBandApplyTo,
    VfxTravelingBandBehavior, VfxTravelingBandColor, VfxTravelingBandDirection,
    VfxTravelingBandShader, VfxWayfindingNode, lower_legacy_spatial_shader,
    try_lower_v3_spatial_shader_family,
};

// <FILE>tui-vfx-style/src/models/mod.rs</FILE> - <DESC>Style models module</DESC>
// <VERS>END OF VERSION: 2.6.0</VERS>
