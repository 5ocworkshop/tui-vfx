// <FILE>tui-vfx-style/src/models/mod.rs</FILE> - <DESC>Style models module</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>feat/cursor-primitive T24: register CursorShader skeleton — new module cls_cursor_shader providing CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail as a flat per-frame snapshot type owned by tui-vfx-style so downstream consumers can hand it to SpatialShaderType without a style→content dep</WCTX>
// <CLOG>Register cls_cursor_shader module and public re-exports for CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail</CLOG>

pub mod cls_affordance_wake_shader;
pub mod cls_ambient_occlusion_shader;
pub mod cls_barber_pole_shader;
pub mod cls_bevel_shader;
pub mod cls_bindable_u16;
pub mod cls_blend_mode;
pub mod cls_border_sweep_shader;
pub mod cls_chromatic_edge_shader;
pub mod cls_color_config;
pub mod cls_color_ramp;
pub mod cls_color_space;
pub mod cls_concealed_light_shader;
pub mod cls_cursor_shader;
pub mod cls_diffusion_shader;
pub mod cls_edge_sheen_shader;
pub mod cls_fade_effect;
pub mod cls_fade_spec;
pub mod cls_falloff_type;
pub mod cls_focus_field_shader;
pub mod cls_focused_row_gradient_shader;
pub mod cls_glisten_band_shader;
pub mod cls_glitch_lines_shader;
pub mod cls_glow_shader;
pub mod cls_gradient;
pub mod cls_gradient_lut;
pub mod cls_highlighter_shader;
pub mod cls_linear_gradient_shader;
pub mod cls_neon_flicker_shader;
pub mod cls_noise_type;
pub mod cls_orbit_shader;
pub mod cls_pulse_wave_shader;
pub mod cls_radar_shader;
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
pub mod cls_trace_common;
pub mod cls_trace_path_shader;
pub mod cls_trace_propagation_shader;
pub mod cls_wayfinding_node_shader;

#[cfg(test)]
pub(crate) mod test_support;
pub use cls_affordance_wake_shader::{
    AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone,
};
pub use cls_ambient_occlusion_shader::{AOEdges, AmbientOcclusionShader};
pub use cls_barber_pole_shader::BarberPoleShader;
pub use cls_bevel_shader::{BevelShader, LightDirection};
pub use cls_bindable_u16::BindableU16;
pub use cls_blend_mode::BlendMode;
pub use cls_border_sweep_shader::BorderSweepShader;
pub use cls_chromatic_edge_shader::ChromaticEdgeShader;
pub use cls_color_config::ColorConfig;
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
pub use cls_linear_gradient_shader::LinearGradientShader;
pub use cls_neon_flicker_shader::{NeonFlickerShader, SegmentMode};
pub use cls_noise_type::NoiseType;
pub use cls_orbit_shader::OrbitShader;
pub use cls_pulse_wave_shader::{PulseWaveShader, WaveDirection};
pub use cls_radar_shader::RadarShader;
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
pub use cls_trace_common::{TraceApplyTo, TraceOrigin, TracePoint, TracePolyline};
pub use cls_trace_path_shader::TracePathShader;
pub use cls_trace_propagation_shader::TracePropagationShader;
pub use cls_wayfinding_node_shader::{WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader};
pub use tui_vfx_geometry::easing::EasingType;

// <FILE>tui-vfx-style/src/models/mod.rs</FILE> - <DESC>Style models module</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
