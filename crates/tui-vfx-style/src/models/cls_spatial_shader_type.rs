// <FILE>tui-vfx-style/src/models/cls_spatial_shader_type.rs</FILE> - <DESC>Enum of all spatial shaders with documentation methods</DESC>
// <VERS>VERSION: 2.1.1</VERS>
// <WCTX>Rustfmt normalization for spatial shader docs</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

//! # Spatial Shader Types
//!
//! Spatial shaders compute per-cell style modifications based on position,
//! time, and animation state. They add visual texture and dynamic effects
//! to widget content.
//!
//! ## Shader Categories
//!
//! ### Gradients & Fills
//! | Shader | Description |
//! |--------|-------------|
//! | [`LinearGradient`](SpatialShaderType::LinearGradient) | Static color gradient at any angle |
//! | [`Highlighter`](SpatialShaderType::Highlighter) | Marker-style text reveal |
//!
//! ### Animated Effects
//! | Shader | Description |
//! |--------|-------------|
//! | [`BarberPole`](SpatialShaderType::BarberPole) | Animated diagonal stripes |
//! | [`Radar`](SpatialShaderType::Radar) | Rotating radar sweep |
//! | [`Orbit`](SpatialShaderType::Orbit) | Dots orbiting the center |
//! | [`BorderSweep`](SpatialShaderType::BorderSweep) | Highlight tracing border |
//! | [`Reflect`](SpatialShaderType::Reflect) | Moving reflective glint |
//! | [`GlistenBand`](SpatialShaderType::GlistenBand) | Moving light band sweep |
//! | [`PulseWave`](SpatialShaderType::PulseWave) | Rippling color wave |
//!
//! ### Glitch & Flicker
//! | Shader | Description |
//! |--------|-------------|
//! | [`GlitchLines`](SpatialShaderType::GlitchLines) | Random horizontal glitch |
//! | [`NeonFlicker`](SpatialShaderType::NeonFlicker) | Flickering neon tube |
//! | [`SubCellShake`](SpatialShaderType::SubCellShake) | Micro-jitter oscillation |
//! | [`ChromaticEdge`](SpatialShaderType::ChromaticEdge) | RGB edge separation |
//!
//! ### Depth & 3D
//! | Shader | Description |
//! |--------|-------------|
//! | [`AmbientOcclusion`](SpatialShaderType::AmbientOcclusion) | Contact shadow at edges |
//! | [`Bevel`](SpatialShaderType::Bevel) | 3D embossed edge effect |
//! | [`Glow`](SpatialShaderType::Glow) | Multi-cell bloom/halo |
//!
//! ### Premium Textures
//! | Shader | Description |
//! |--------|-------------|
//! | [`StochasticSparkle`](SpatialShaderType::StochasticSparkle) | Film grain / frosted glass shimmer |
//!
//! ## Usage
//!
//! Shaders are typically applied via [`CompositionOptions::shader_layers`] or
//! wrapped in a [`StyleEffect::Spatial`] for temporal animation.

use crate::models::{
    LinearGradientShader, cls_ambient_occlusion_shader::AmbientOcclusionShader,
    cls_barber_pole_shader::BarberPoleShader, cls_bevel_shader::BevelShader,
    cls_border_sweep_shader::BorderSweepShader, cls_chromatic_edge_shader::ChromaticEdgeShader,
    cls_focused_row_gradient_shader::FocusedRowGradientShader,
    cls_glisten_band_shader::GlistenBandShader, cls_glitch_lines_shader::GlitchLinesShader,
    cls_glow_shader::GlowShader, cls_highlighter_shader::HighlighterShader,
    cls_neon_flicker_shader::NeonFlickerShader, cls_orbit_shader::OrbitShader,
    cls_pulse_wave_shader::PulseWaveShader, cls_radar_shader::RadarShader,
    cls_reflect_shader::ReflectShader, cls_reveal_wipe_shader::RevealWipeShader,
    cls_stochastic_sparkle_shader::StochasticSparkleShader,
    cls_sub_cell_shake_shader::SubCellShakeShader,
};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;
/// Spatial shader types for per-cell style computation.
///
/// Each shader computes style modifications based on cell position,
/// animation time, and shader-specific parameters. They're the primary
/// mechanism for adding visual texture to widget content.
///
/// # Categories
///
/// - **Gradients**: LinearGradient, Highlighter
/// - **Animated**: BarberPole, Radar, Orbit, BorderSweep, Reflect, GlistenBand, PulseWave
/// - **Glitch**: GlitchLines, NeonFlicker, SubCellShake, ChromaticEdge
/// - **Depth**: AmbientOcclusion, Bevel, Glow
/// - **Premium**: StochasticSparkle
///
/// # Built-in Documentation
///
/// Use [`terse_description()`](Self::terse_description) and
/// [`key_parameters()`](Self::key_parameters) for runtime documentation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SpatialShaderType {
    /// Static color gradient across widget at configurable angle.
    LinearGradient(LinearGradientShader),

    /// Animated diagonal stripes like a barber pole (loading indicator).
    BarberPole(BarberPoleShader),

    /// Rotating radar-style sweep effect (scanning/search).
    Radar(RadarShader),

    /// Dots orbiting around the widget center (loading/attention).
    Orbit(OrbitShader),

    /// Highlight that traces the border edge (focus indication).
    BorderSweep(BorderSweepShader),

    /// Marker-style highlight revealing text (emphasis).
    Highlighter(HighlighterShader),

    /// Moving reflective glint band (premium shine).
    Reflect(ReflectShader),

    /// Moving band of light sweeping across (loading shimmer).
    GlistenBand(GlistenBandShader),

    /// Random horizontal glitch interference lines (error/warning).
    GlitchLines(GlitchLinesShader),

    /// Flickering neon sign effect with segments (retro).
    NeonFlicker(NeonFlickerShader),

    /// Rippling color wave emanating from position (attention).
    PulseWave(PulseWaveShader),

    /// Vertical gradient centered on selected row (list navigation).
    FocusedRowGradient(FocusedRowGradientShader),

    /// Progressive reveal from one direction (transition).
    RevealWipe(RevealWipeShader),

    /// Film grain / frosted glass shimmer texture (premium).
    StochasticSparkle(StochasticSparkleShader),

    /// Contact shadow effect at widget edges (depth).
    AmbientOcclusion(AmbientOcclusionShader),

    /// 3D embossed edge effect with light direction (depth).
    Bevel(BevelShader),

    /// Multi-cell bloom/halo around widget edges (focus).
    Glow(GlowShader),

    /// Micro-jitter through rapid color oscillation (error).
    SubCellShake(SubCellShakeShader),

    /// Chromatic aberration separating RGB at edges (glitch).
    ChromaticEdge(ChromaticEdgeShader),
}
impl StyleShader for SpatialShaderType {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        match self {
            SpatialShaderType::LinearGradient(s) => s.style_at(ctx, base),
            SpatialShaderType::BarberPole(s) => s.style_at(ctx, base),
            SpatialShaderType::Radar(s) => s.style_at(ctx, base),
            SpatialShaderType::Orbit(s) => s.style_at(ctx, base),
            SpatialShaderType::BorderSweep(s) => s.style_at(ctx, base),
            SpatialShaderType::Highlighter(s) => s.style_at(ctx, base),
            SpatialShaderType::Reflect(s) => s.style_at(ctx, base),
            SpatialShaderType::GlistenBand(s) => s.style_at(ctx, base),
            SpatialShaderType::GlitchLines(s) => s.style_at(ctx, base),
            SpatialShaderType::NeonFlicker(s) => s.style_at(ctx, base),
            SpatialShaderType::PulseWave(s) => s.style_at(ctx, base),
            SpatialShaderType::FocusedRowGradient(s) => s.style_at(ctx, base),
            SpatialShaderType::RevealWipe(s) => s.style_at(ctx, base),
            SpatialShaderType::StochasticSparkle(s) => s.style_at(ctx, base),
            SpatialShaderType::AmbientOcclusion(s) => s.style_at(ctx, base),
            SpatialShaderType::Bevel(s) => s.style_at(ctx, base),
            SpatialShaderType::Glow(s) => s.style_at(ctx, base),
            SpatialShaderType::SubCellShake(s) => s.style_at(ctx, base),
            SpatialShaderType::ChromaticEdge(s) => s.style_at(ctx, base),
        }
    }

    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl SpatialShaderType {
    /// Returns the shader type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            SpatialShaderType::LinearGradient(_) => "LinearGradient",
            SpatialShaderType::BarberPole(_) => "BarberPole",
            SpatialShaderType::Radar(_) => "Radar",
            SpatialShaderType::Orbit(_) => "Orbit",
            SpatialShaderType::BorderSweep(_) => "BorderSweep",
            SpatialShaderType::Highlighter(_) => "Highlighter",
            SpatialShaderType::Reflect(_) => "Reflect",
            SpatialShaderType::GlistenBand(_) => "GlistenBand",
            SpatialShaderType::GlitchLines(_) => "GlitchLines",
            SpatialShaderType::NeonFlicker(_) => "NeonFlicker",
            SpatialShaderType::PulseWave(_) => "PulseWave",
            SpatialShaderType::FocusedRowGradient(_) => "FocusedRowGradient",
            SpatialShaderType::RevealWipe(_) => "RevealWipe",
            SpatialShaderType::StochasticSparkle(_) => "StochasticSparkle",
            SpatialShaderType::AmbientOcclusion(_) => "AmbientOcclusion",
            SpatialShaderType::Bevel(_) => "Bevel",
            SpatialShaderType::Glow(_) => "Glow",
            SpatialShaderType::SubCellShake(_) => "SubCellShake",
            SpatialShaderType::ChromaticEdge(_) => "ChromaticEdge",
        }
    }

    /// Returns a brief human-readable description of what this shader does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            SpatialShaderType::LinearGradient(_) => "Static color gradient across widget",
            SpatialShaderType::BarberPole(_) => "Animated diagonal stripes like a barber pole",
            SpatialShaderType::Radar(_) => "Rotating radar-style sweep effect",
            SpatialShaderType::Orbit(_) => "Dots orbiting around the widget center",
            SpatialShaderType::BorderSweep(_) => "Highlight that traces the border edge",
            SpatialShaderType::Highlighter(_) => "Marker-style highlight revealing text",
            SpatialShaderType::Reflect(_) => "Moving reflective glint band",
            SpatialShaderType::GlistenBand(_) => {
                "Moving band of light that sweeps across the widget"
            }
            SpatialShaderType::GlitchLines(_) => "Random horizontal glitch interference lines",
            SpatialShaderType::NeonFlicker(_) => {
                "Flickering neon sign effect with independent segments"
            }
            SpatialShaderType::PulseWave(_) => "Rippling color wave emanating from position",
            SpatialShaderType::FocusedRowGradient(_) => {
                "Vertical gradient centered on a selected row"
            }
            SpatialShaderType::RevealWipe(_) => {
                "Progressive reveal from one direction, hiding unrevealed text"
            }
            SpatialShaderType::StochasticSparkle(_) => {
                "Film grain / frosted glass effect with random cell brightening"
            }
            SpatialShaderType::AmbientOcclusion(_) => {
                "Contact shadow effect darkening cells near widget edges"
            }
            SpatialShaderType::Bevel(_) => {
                "3D embossed edge effect with configurable light direction"
            }
            SpatialShaderType::Glow(_) => "Multi-cell bloom/halo effect around widget edges",
            SpatialShaderType::SubCellShake(_) => {
                "Micro-jitter visual effect through rapid color oscillation"
            }
            SpatialShaderType::ChromaticEdge(_) => {
                "Chromatic aberration effect separating RGB edges"
            }
        }
    }

    /// Returns key parameters of this shader for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            SpatialShaderType::LinearGradient(s) => vec![("angle_deg", format!("{}", s.angle_deg))],
            SpatialShaderType::BarberPole(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("stripe_width", format!("{}", s.stripe_width)),
                ("gap_width", format!("{}", s.gap_width)),
            ],
            SpatialShaderType::Radar(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("tail_length", format!("{:.2} rad", s.tail_length)),
            ],
            SpatialShaderType::Orbit(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("dot_count", format!("{}", s.dot_count)),
            ],
            SpatialShaderType::BorderSweep(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("length", format!("{} cells", s.length)),
            ],
            SpatialShaderType::Highlighter(_s) => vec![],
            SpatialShaderType::Reflect(s) => vec![("speed", format!("{}", s.speed))],
            SpatialShaderType::GlistenBand(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("band_width", format!("{} cells", s.band_width)),
                ("direction", format!("{:?}", s.direction)),
                ("angle_deg", format!("{}deg", s.angle_deg)),
            ],
            SpatialShaderType::GlitchLines(s) => vec![
                ("intensity", format!("{}", s.intensity)),
                ("max_lines", format!("{}", s.max_lines)),
                ("speed", format!("{}", s.speed)),
            ],
            SpatialShaderType::NeonFlicker(s) => vec![
                ("stability", format!("{}", s.stability)),
                ("segment", format!("{:?}", s.segment)),
                ("dim_amount", format!("{}", s.dim_amount)),
            ],
            SpatialShaderType::PulseWave(s) => vec![
                ("frequency", format!("{}", s.frequency)),
                ("speed", format!("{}", s.speed)),
                ("direction", format!("{:?}", s.direction)),
                ("wavelength", format!("{} cells", s.wavelength)),
            ],
            SpatialShaderType::FocusedRowGradient(s) => vec![
                ("selected_row_ratio", format!("{}", s.selected_row_ratio)),
                ("falloff_distance", format!("{} rows", s.falloff_distance)),
                ("apply_to", format!("{:?}", s.apply_to)),
            ],
            SpatialShaderType::RevealWipe(s) => vec![("direction", format!("{:?}", s.direction))],
            SpatialShaderType::StochasticSparkle(s) => vec![
                (
                    "sparkle_density",
                    format!("{:.0}%", s.sparkle_density * 100.0),
                ),
                (
                    "brightness_boost",
                    format!("{:.0}%", (s.brightness_boost - 1.0) * 100.0),
                ),
                ("speed", format!("{}", s.speed)),
                ("apply_to", format!("{:?}", s.apply_to)),
            ],
            SpatialShaderType::AmbientOcclusion(s) => vec![
                ("intensity", format!("{}", s.intensity)),
                ("radius", format!("{} cells", s.radius)),
                ("edges", format!("{:?}", s.edges)),
                ("falloff", format!("{:?}", s.falloff)),
            ],
            SpatialShaderType::Bevel(s) => vec![
                ("light_direction", format!("{:?}", s.light_direction)),
                ("highlight_intensity", format!("{}", s.highlight_intensity)),
                ("shadow_intensity", format!("{}", s.shadow_intensity)),
                ("edge_width", format!("{} cells", s.edge_width)),
            ],
            SpatialShaderType::Glow(s) => vec![
                ("radius", format!("{} cells", s.radius)),
                ("intensity", format!("{}", s.intensity)),
                ("falloff", format!("{:?}", s.falloff)),
                ("pulse_speed", format!("{} Hz", s.pulse_speed)),
            ],
            SpatialShaderType::SubCellShake(s) => vec![
                ("amplitude", format!("{}", s.amplitude)),
                ("frequency", format!("{} Hz", s.frequency)),
                ("axis", format!("{:?}", s.axis)),
                ("chromatic", format!("{}", s.chromatic)),
            ],
            SpatialShaderType::ChromaticEdge(s) => vec![
                ("intensity", format!("{}", s.intensity)),
                ("edge_width", format!("{} cells", s.edge_width)),
                ("horizontal", format!("{}", s.horizontal)),
            ],
        }
    }
}

// <FILE>tui-vfx-style/src/models/cls_spatial_shader_type.rs</FILE> - <DESC>Enum of all spatial shaders with documentation methods</DESC>
// <VERS>END OF VERSION: 2.1.1</VERS>
