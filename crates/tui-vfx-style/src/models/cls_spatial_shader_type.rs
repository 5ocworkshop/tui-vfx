// <FILE>tui-vfx-style/src/models/cls_spatial_shader_type.rs</FILE> - <DESC>Enum of all spatial shaders with documentation methods</DESC>
// <VERS>VERSION: 2.5.0</VERS>
// <WCTX>Keep representative V3-authored shader payload normalization close to executable shader semantics so direct-V3 callers can stop carrying primitive-form alias rewrites and binding-object extraction themselves.</WCTX>
// <CLOG>2.5.0: add RadialSpiral as an executable spatial shader backed by mixed-signals radial field math.
// 2.4.0: add SpatialShaderType::try_from_v3_payload for engine-owned V3 shader payload normalization (primitive aliases, representative binding-object fields, and compatibility fallbacks).</CLOG>

//! # Spatial Shader Types
//!
//! Spatial shaders compute per-cell style modifications based on position,
//! time, and animation state. They add visual texture and dynamic effects
//! to widget content.
//!
//! ## Migration note
//!
//! `SpatialShaderType` is the **current flat implementation surface** for
//! spatial shaders. Under the V3 plan (Decision 2), this catalog is expected to
//! evolve toward:
//!
//! - deeper primitive / substrate families
//! - earned named factories or presets
//! - clearer separation between primitive capability and policy composition
//!
//! So this enum should be read as the live migration surface, not as proof that
//! the final V3 conceptual model is "one flat list forever."
//!
//! The practical cutover seam already exists in the grouped V3 family surface:
//!
//! - `LinearGradient`, `RevealWipe` → `gradient_reveal` primitive family
//! - `Glow`, `AmbientOcclusion`, `Bevel` → `surface_depth` primitive family
//! - `PulseWave`, `Radar`, `Orbit` → `motion_field` primitive family
//! - `GlitchLines`, `ChromaticEdge`, `SubCellShake` → `edge_distortion`
//!   primitive family
//! - `BorderSweep`, `Reflect`, `GlistenBand`, `TracePropagation`, `TracePath`
//!   → `traveling_band` composed family
//! - `Highlighter` → `progress_emphasis` composed family
//! - `Diffusion`, `ConcealedLight`, `EdgeSheen` → `material_light` composed
//!   family
//! - `FocusedRowGradient`, `FocusField`, `AffordanceWake`, `WayfindingNode`
//!   → `guidance_cue` composed family
//! - `NeonFlicker`, `StochasticSparkle` → `stochastic_texture` composed family
//! - `BarberPole` → `stripe_motion` composed family
//! - `Cursor` → `cursor` composed family
//!
//! Callers that need the grouped V3 view should prefer
//! [`SpatialShaderType::v3_spatial_shader_family`] or
//! [`SpatialShaderType::v3_family_label`] instead of re-classifying variants
//! ad hoc.
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
//! | [`RadialSpiral`](SpatialShaderType::RadialSpiral) | Procedural radial spiral field |
//! | [`TracePropagation`](SpatialShaderType::TracePropagation) | Orthogonal routed signal pulse |
//! | [`TracePath`](SpatialShaderType::TracePath) | Authored routed signal path |
//! | [`EdgeSheen`](SpatialShaderType::EdgeSheen) | Calm perimeter sheen for shells |
//! | [`ConcealedLight`](SpatialShaderType::ConcealedLight) | Hidden-source architectural light wash |
//! | [`Diffusion`](SpatialShaderType::Diffusion) | Soft material-light diffusion |
//! | [`FocusField`](SpatialShaderType::FocusField) | Point or pane-following focus field |
//! | [`AffordanceWake`](SpatialShaderType::AffordanceWake) | Dormant secondary affordances resolving on demand |
//! | [`WayfindingNode`](SpatialShaderType::WayfindingNode) | Calm node emphasis for steps, breadcrumbs, and junctions |
//! | [`Cursor`](SpatialShaderType::Cursor) | Primary-cell alpha + wake trail tint/ghost |
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
//! Shaders are typically applied via `CompositionOptions::shader_layers`
//! (in `tui-vfx-compositor`) or wrapped in a
//! [`crate::models::StyleEffect::Spatial`] for temporal animation.

use crate::models::{
    LinearGradientShader, VfxSpatialShaderFamily, cls_affordance_wake_shader::AffordanceWakeShader,
    cls_ambient_occlusion_shader::AmbientOcclusionShader, cls_barber_pole_shader::BarberPoleShader,
    cls_bevel_shader::BevelShader, cls_border_sweep_shader::BorderSweepShader,
    cls_chromatic_edge_shader::ChromaticEdgeShader,
    cls_concealed_light_shader::ConcealedLightShader, cls_cursor_shader::CursorShader,
    cls_diffusion_shader::DiffusionShader, cls_edge_sheen_shader::EdgeSheenShader,
    cls_focus_field_shader::FocusFieldShader,
    cls_focused_row_gradient_shader::FocusedRowGradientShader,
    cls_glisten_band_shader::GlistenBandShader, cls_glitch_lines_shader::GlitchLinesShader,
    cls_glow_shader::GlowShader, cls_highlighter_shader::HighlighterShader,
    cls_neon_flicker_shader::NeonFlickerShader, cls_orbit_shader::OrbitShader,
    cls_pulse_wave_shader::PulseWaveShader, cls_radar_shader::RadarShader,
    cls_radial_spiral_shader::RadialSpiralShader, cls_reflect_shader::ReflectShader,
    cls_reveal_wipe_shader::RevealWipeShader,
    cls_stochastic_sparkle_shader::StochasticSparkleShader,
    cls_sub_cell_shake_shader::SubCellShakeShader, cls_trace_path_shader::TracePathShader,
    cls_trace_propagation_shader::TracePropagationShader,
    cls_wayfinding_node_shader::WayfindingNodeShader,
};
use mixed_signals::types::SignalOrFloat;

use crate::traits::{
    ShaderContext, ShaderRuntimeBindingRequest, ShaderRuntimeBindingResolution, StyleShader,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
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
/// - **Animated**: BarberPole, Radar, Orbit, BorderSweep, Reflect, GlistenBand, PulseWave,
///   TracePropagation, TracePath, EdgeSheen, ConcealedLight, Diffusion, FocusField,
///   AffordanceWake, WayfindingNode
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

    /// Procedural radial spiral density field (portal/loading/background texture).
    RadialSpiral(RadialSpiralShader),

    /// Orthogonal signal pulse moving through routed trace lanes.
    TracePropagation(TracePropagationShader),

    /// Authored routed signal path following explicit waypoints.
    TracePath(TracePathShader),

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

    /// Calm premium sheen that glides along the widget perimeter.
    EdgeSheen(EdgeSheenShader),

    /// Hidden-source architectural light for thresholds, shells, and seams.
    ConcealedLight(ConcealedLightShader),

    /// Soft material-light diffusion for paper, textile, and frosted surfaces.
    Diffusion(DiffusionShader),

    /// Point/ellipse or pane/rect-following focus field.
    FocusField(FocusFieldShader),

    /// Dormant secondary affordances that resolve on demand.
    AffordanceWake(AffordanceWakeShader),

    /// Calm node/junction emphasis for breadcrumbs, steps, and route hints.
    WayfindingNode(WayfindingNodeShader),

    /// Micro-jitter through rapid color oscillation (error).
    SubCellShake(SubCellShakeShader),

    /// Chromatic aberration separating RGB at edges (glitch).
    ChromaticEdge(ChromaticEdgeShader),

    /// Primary-cell alpha modulation + wake trail tint/ghost for a cursor
    /// primitive. Built per-frame from a `CursorPaintOps` snapshot via
    /// `tui_vfx_content::cursor::fnc_build_cursor_shader`.
    Cursor(CursorShader),
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
            SpatialShaderType::RadialSpiral(s) => s.style_at(ctx, base),
            SpatialShaderType::TracePropagation(s) => s.style_at(ctx, base),
            SpatialShaderType::TracePath(s) => s.style_at(ctx, base),
            SpatialShaderType::FocusedRowGradient(s) => s.style_at(ctx, base),
            SpatialShaderType::RevealWipe(s) => s.style_at(ctx, base),
            SpatialShaderType::StochasticSparkle(s) => s.style_at(ctx, base),
            SpatialShaderType::AmbientOcclusion(s) => s.style_at(ctx, base),
            SpatialShaderType::Bevel(s) => s.style_at(ctx, base),
            SpatialShaderType::Glow(s) => s.style_at(ctx, base),
            SpatialShaderType::EdgeSheen(s) => s.style_at(ctx, base),
            SpatialShaderType::ConcealedLight(s) => s.style_at(ctx, base),
            SpatialShaderType::Diffusion(s) => s.style_at(ctx, base),
            SpatialShaderType::FocusField(s) => s.style_at(ctx, base),
            SpatialShaderType::AffordanceWake(s) => s.style_at(ctx, base),
            SpatialShaderType::WayfindingNode(s) => s.style_at(ctx, base),
            SpatialShaderType::SubCellShake(s) => s.style_at(ctx, base),
            SpatialShaderType::ChromaticEdge(s) => s.style_at(ctx, base),
            SpatialShaderType::Cursor(s) => s.style_at(ctx, base),
        }
    }

    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl SpatialShaderType {
    /// Build a runnable spatial shader from a V3-authored payload shape.
    ///
    /// This constructor accepts the current authoring/migration payload forms
    /// that still need compatibility normalization before they can execute on
    /// the flat runtime shader surface. Keeping those translations here moves
    /// V3 payload ownership closer to the executable shader semantics instead
    /// of leaving every alias in higher-level bridge code.
    pub fn try_from_v3_payload(mut payload: Value) -> serde_json::Result<Self> {
        if let Value::Object(ref mut object) = payload {
            let payload_type = object
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();

            if payload_type == "fractional_stripe_overlay" {
                object.clear();
                object.insert("type".into(), serde_json::json!("barber_pole"));
                object.insert("color".into(), serde_json::json!({"type":"white"}));
                object.insert("speed".into(), serde_json::json!(1.0));
                object.insert("stripe_width".into(), serde_json::json!(1));
                object.insert("gap_width".into(), serde_json::json!(1));
            }
            if payload_type == "gradient_overlay" {
                object.insert("type".into(), Value::String("linear_gradient".into()));
                object.remove("intensity");
                object.remove("apply_to");
            }
            if payload_type == "colored_overlay" {
                normalize_colored_overlay_payload(object);
            }

            match payload_type.as_str() {
                "highlighter" => {
                    extract_binding_object(object, "speed", "speed_binding");
                    extract_binding_object(object, "blend_strength", "blend_strength_binding");
                    extract_binding_object(object, "direction", "direction_binding");
                }
                "glisten_band" => {
                    extract_binding_object(object, "speed", "speed_binding");
                    extract_binding_object(object, "blend_strength", "blend_strength_binding");
                    extract_binding_object(object, "direction", "direction_binding");
                }
                "pulse_wave" => {
                    extract_binding_object(object, "frequency", "frequency_binding");
                }
                "concealed_light" => {
                    if let Some(spread) = object.get("spread").cloned() {
                        if spread.get("signal").is_some() {
                            if let Some(default) = signal_default_f32(&spread) {
                                object.insert(
                                    "spread".into(),
                                    serde_json::json!(default.round().clamp(1.0, 255.0) as u8),
                                );
                            }
                            object.insert("mode".into(), serde_json::json!("drift"));
                            object.insert("pulse_speed".into(), serde_json::json!(1.0));
                        }
                    }
                }
                "border_sweep" => {
                    extract_binding_object(object, "position", "position_binding");
                }
                "focus_field" => {
                    extract_binding_object(object, "center_x", "center_x_binding");
                    extract_binding_object(object, "center_y", "center_y_binding");
                    extract_binding_object(object, "rect_x", "rect_x_binding");
                    extract_binding_object(object, "rect_y", "rect_y_binding");
                    extract_binding_object(object, "rect_width", "rect_width_binding");
                    extract_binding_object(object, "rect_height", "rect_height_binding");
                }
                "affordance_wake" => {
                    extract_binding_object(object, "progress", "progress_binding");
                }
                "wayfinding_node" => {
                    extract_binding_object(object, "current_index", "current_index_binding");
                }
                "focused_row_gradient" => {
                    extract_binding_object(object, "selected_row", "selected_row_binding");
                    extract_binding_object(
                        object,
                        "selected_row_ratio",
                        "selected_row_ratio_binding",
                    );
                }
                _ => {}
            }

            if matches!(
                payload_type.as_str(),
                "bevel"
                    | "barber_pole"
                    | "border_sweep"
                    | "pulse_wave"
                    | "linear_gradient"
                    | "glow"
                    | "ambient_occlusion"
            ) {
                object.remove("apply_to");
            }
        }

        serde_json::from_value(payload)
    }

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
            SpatialShaderType::RadialSpiral(_) => "RadialSpiral",
            SpatialShaderType::TracePropagation(_) => "TracePropagation",
            SpatialShaderType::TracePath(_) => "TracePath",
            SpatialShaderType::FocusedRowGradient(_) => "FocusedRowGradient",
            SpatialShaderType::RevealWipe(_) => "RevealWipe",
            SpatialShaderType::StochasticSparkle(_) => "StochasticSparkle",
            SpatialShaderType::AmbientOcclusion(_) => "AmbientOcclusion",
            SpatialShaderType::Bevel(_) => "Bevel",
            SpatialShaderType::Glow(_) => "Glow",
            SpatialShaderType::EdgeSheen(_) => "EdgeSheen",
            SpatialShaderType::ConcealedLight(_) => "ConcealedLight",
            SpatialShaderType::Diffusion(_) => "Diffusion",
            SpatialShaderType::FocusField(_) => "FocusField",
            SpatialShaderType::AffordanceWake(_) => "AffordanceWake",
            SpatialShaderType::WayfindingNode(_) => "WayfindingNode",
            SpatialShaderType::SubCellShake(_) => "SubCellShake",
            SpatialShaderType::ChromaticEdge(_) => "ChromaticEdge",
            SpatialShaderType::Cursor(_) => "Cursor",
        }
    }

    /// Return the grouped V3 family form of this legacy flat shader.
    ///
    /// This is the preferred cutover seam for downstream callers that need the
    /// primitive-vs-composed V3 classification without re-matching every legacy
    /// enum variant locally.
    pub fn v3_spatial_shader_family(&self) -> VfxSpatialShaderFamily {
        VfxSpatialShaderFamily::from_legacy_spatial_shader(self)
    }

    /// Return the stable grouped V3 family label for this shader.
    pub fn v3_family_label(&self) -> &'static str {
        self.v3_spatial_shader_family().family_label()
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
            SpatialShaderType::RadialSpiral(_) => "Procedural radial spiral density field",
            SpatialShaderType::TracePropagation(_) => {
                "Orthogonal signal pulse moving through routed trace lanes"
            }
            SpatialShaderType::TracePath(_) => {
                "Authored routed signal path following explicit waypoints"
            }
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
            SpatialShaderType::EdgeSheen(_) => {
                "Calm premium sheen that glides along the widget perimeter"
            }
            SpatialShaderType::ConcealedLight(_) => {
                "Hidden-source architectural light wash for thresholds, seams, and shell hierarchy"
            }
            SpatialShaderType::Diffusion(_) => {
                "Soft material-light diffusion for paper, textile, and frosted surfaces"
            }
            SpatialShaderType::FocusField(_) => {
                "Point or pane-following focus field for subtle attention shaping"
            }
            SpatialShaderType::AffordanceWake(_) => {
                "Dormant secondary affordances resolving on demand through edge, corner, or rail emphasis"
            }
            SpatialShaderType::WayfindingNode(_) => {
                "Calm node emphasis for breadcrumbs, progress steps, and route hints"
            }
            SpatialShaderType::SubCellShake(_) => {
                "Micro-jitter visual effect through rapid color oscillation"
            }
            SpatialShaderType::ChromaticEdge(_) => {
                "Chromatic aberration effect separating RGB edges"
            }
            SpatialShaderType::Cursor(_) => {
                "Primary-cell alpha modulation + wake trail tint/ghost for a cursor primitive"
            }
        }
    }

    /// Returns the runtime binding requests declared by this shader.
    pub fn runtime_binding_requests(&self) -> Vec<ShaderRuntimeBindingRequest> {
        match self {
            SpatialShaderType::FocusedRowGradient(shader) => shader.runtime_binding_requests(),
            SpatialShaderType::FocusField(shader) => shader.runtime_binding_requests(),
            SpatialShaderType::AffordanceWake(shader) => shader.runtime_binding_requests(),
            SpatialShaderType::WayfindingNode(shader) => shader.runtime_binding_requests(),
            _ => Vec::new(),
        }
    }

    /// Resolves runtime bindings for this shader against the current shader context.
    pub fn runtime_binding_resolutions(
        &self,
        ctx: &ShaderContext,
    ) -> Vec<ShaderRuntimeBindingResolution> {
        match self {
            SpatialShaderType::FocusedRowGradient(shader) => {
                shader.runtime_binding_resolutions(ctx)
            }
            SpatialShaderType::FocusField(shader) => shader.runtime_binding_resolutions(ctx),
            SpatialShaderType::AffordanceWake(shader) => shader.runtime_binding_resolutions(ctx),
            SpatialShaderType::WayfindingNode(shader) => shader.runtime_binding_resolutions(ctx),
            _ => Vec::new(),
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
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::Radar(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("tail_length", format!("{:.2} rad", s.tail_length)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::Orbit(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("dot_count", format!("{}", s.dot_count)),
            ],
            SpatialShaderType::BorderSweep(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("length", format!("{} cells", s.length)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::Highlighter(s) => vec![
                ("color", format!("{:?}", s.color)),
                ("apply_to", format!("{:?}", s.apply_to)),
                ("mode", format!("{:?}", s.mode)),
                ("direction", format!("{:?}", s.direction)),
                ("speed", format!("{}", s.speed)),
                ("blend_strength", format!("{}", s.blend_strength)),
                ("soft_edge", format!("{}", s.soft_edge)),
                ("band_width", format!("{} cells", s.band_width)),
            ],
            SpatialShaderType::Reflect(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::GlistenBand(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("band_width", format!("{} cells", s.band_width)),
                ("direction", format!("{:?}", s.direction)),
                ("angle_deg", format!("{}deg", s.angle_deg)),
                ("head", format!("{:?}", s.head)),
                ("tail", format!("{:?}", s.tail)),
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
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::RadialSpiral(s) => vec![
                ("arms", format!("{}", s.arms)),
                ("radial_frequency", format!("{}", s.radial_frequency)),
                ("radial_power", format!("{}", s.radial_power)),
                ("speed", format!("{}", s.speed)),
                ("blend_strength", format!("{}", s.blend_strength)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::TracePropagation(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("grid_spacing", format!("{} cells", s.grid_spacing)),
                ("tail_length", format!("{:.1} cells", s.tail_length)),
                ("origin", format!("{:?}", s.origin)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::TracePath(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("tail_length", format!("{:.1} cells", s.tail_length)),
                ("vertical_weight", format!("{}", s.vertical_weight)),
                ("thickness", format!("{} cells", s.thickness)),
                ("junction_boost", format!("{}", s.junction_boost)),
                ("junction_glow", format!("{}", s.junction_glow)),
                ("tail_mode", format!("{:?}", s.tail_mode)),
                ("paths", format!("{}", s.paths.len())),
                ("color", format!("{:?}", s.color)),
                ("apply_to", format!("{:?}", s.apply_to)),
            ],
            SpatialShaderType::FocusedRowGradient(s) => {
                let mut params = vec![
                    ("selected_row_ratio", format!("{}", s.selected_row_ratio)),
                    ("falloff_distance", format!("{} rows", s.falloff_distance)),
                    ("apply_to", format!("{:?}", s.apply_to)),
                ];
                if let Some(selected_row) = s.selected_row {
                    params.push(("selected_row", format!("{selected_row}")));
                }
                if let Some(binding) = s.selected_row_binding.as_ref() {
                    params.push(("selected_row_binding", binding.clone()));
                }
                if let Some(binding) = s.selected_row_ratio_binding.as_ref() {
                    params.push(("selected_row_ratio_binding", binding.clone()));
                }
                params
            }
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
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::EdgeSheen(s) => vec![
                ("speed", format!("{}", s.speed)),
                ("band_width", format!("{} cells", s.band_width)),
                ("edge_width", format!("{} cells", s.edge_width)),
                ("corner_boost", format!("{}", s.corner_boost)),
                ("color", format!("{:?}", s.color)),
            ],
            SpatialShaderType::ConcealedLight(s) => vec![
                ("source", format!("{:?}", s.source)),
                ("spread", format!("{} cells", s.spread)),
                ("edge_width", format!("{} cells", s.edge_width)),
                ("intensity", format!("{:.2}", s.intensity)),
            ],
            SpatialShaderType::Diffusion(s) => vec![
                ("source", format!("{:?}", s.source)),
                ("radius", format!("{} cells", s.radius)),
                ("softness", format!("{:.2}", s.softness)),
                ("intensity", format_signal_or_float(&s.intensity)),
            ],
            SpatialShaderType::FocusField(s) => vec![
                ("shape", format!("{:?}", s.shape)),
                ("intensity", format!("{:.2}", s.intensity)),
                ("feather", format!("{} cells", s.feather)),
                ("pulse_speed", format!("{:.2}", s.pulse_speed)),
            ],
            SpatialShaderType::AffordanceWake(s) => vec![
                ("zone", format!("{:?}", s.zone)),
                ("radius", format!("{} cells", s.radius)),
                ("peak_intensity", format!("{:.2}", s.peak_intensity)),
                ("progress", format!("{:.2}", s.progress)),
            ],
            SpatialShaderType::WayfindingNode(s) => vec![
                ("nodes", format!("{}", s.nodes.len())),
                ("radius", format!("{} cells", s.radius)),
                ("intensity", format!("{:.2}", s.intensity)),
                (
                    "current_index",
                    s.current_index
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "none".to_string()),
                ),
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
            SpatialShaderType::Cursor(s) => vec![
                ("mode", format!("{:?}", s.mode)),
                ("tint", format!("{:?}", s.tint)),
                (
                    "primary",
                    match s.primary.as_ref() {
                        Some(p) => format!("{:?} @ alpha {:.2}", p.position, p.alpha),
                        None => "none".to_string(),
                    },
                ),
                ("trail_len", format!("{}", s.trail.len())),
            ],
        }
    }
}

fn format_signal_or_float(value: &SignalOrFloat) -> String {
    match value {
        SignalOrFloat::Static(value) => format!("{value:.2}"),
        SignalOrFloat::Signal { spec, .. } => serde_json::to_value(spec)
            .ok()
            .and_then(|value| {
                value
                    .get("type")
                    .and_then(Value::as_str)
                    .map(|kind| format!("signal({kind})"))
            })
            .unwrap_or_else(|| "signal".to_string()),
    }
}

fn extract_binding_object(
    object: &mut serde_json::Map<String, Value>,
    field: &str,
    binding_field: &str,
) {
    let Some(binding_value) = object.get(field).cloned() else {
        return;
    };
    let Value::Object(binding_obj) = binding_value else {
        return;
    };
    let binding_name = binding_obj.get("binding").and_then(Value::as_str);
    let default_value = binding_obj.get("default").cloned();
    if let Some(binding_name) = binding_name {
        object.insert(
            binding_field.to_string(),
            Value::String(binding_name.to_string()),
        );
    }
    if let Some(default_value) = default_value {
        object.insert(field.to_string(), default_value);
    } else {
        object.remove(field);
    }
}

fn signal_default_f32(value: &Value) -> Option<f32> {
    value
        .get("signal")?
        .get("offset")?
        .as_f64()
        .map(|v| v as f32)
}

fn signal_implies_looping(value: &Value) -> bool {
    value
        .get("signal")
        .and_then(|s| s.get("kind"))
        .and_then(Value::as_str)
        == Some("sine")
}

fn edges_to_ao(edges: &[Value]) -> &'static str {
    let strs: Vec<&str> = edges.iter().filter_map(Value::as_str).collect();
    let has = |x| strs.contains(&x);
    if has("bottom") && has("right") && strs.len() == 2 {
        "bottom_right"
    } else if has("top") && has("left") && strs.len() == 2 {
        "top_left"
    } else if has("top") && has("bottom") && has("left") && has("right") {
        "all"
    } else {
        "all"
    }
}

fn source_to_diffusion(source: &str) -> &'static str {
    match source {
        "center" => "center",
        "top" => "top",
        "bottom" => "bottom",
        "left" => "left",
        "right" => "right",
        "top_left" => "top_left",
        "top_right" => "top_right",
        "bottom_left" => "bottom_left",
        "bottom_right" => "bottom_right",
        _ => "center",
    }
}

fn normalize_colored_overlay_payload(obj: &mut serde_json::Map<String, Value>) {
    let Some(pattern) = obj.get("pattern").and_then(Value::as_object) else {
        return;
    };
    let kind = pattern.get("kind").and_then(Value::as_str).unwrap_or("");
    let color = obj
        .get("color")
        .cloned()
        .unwrap_or(serde_json::json!({"type":"white"}));
    let intensity = obj
        .get("intensity")
        .cloned()
        .unwrap_or(serde_json::json!(1.0));
    let apply_to = obj.get("apply_to").cloned();
    match kind {
        "perimeter_halo" => {
            let radius = pattern
                .get("radius")
                .cloned()
                .unwrap_or(serde_json::json!(2));
            let falloff = pattern
                .get("falloff")
                .cloned()
                .unwrap_or(serde_json::json!("quadratic"));
            let mut next = serde_json::Map::new();
            next.insert("type".into(), serde_json::json!("glow"));
            next.insert("color".into(), color);
            next.insert("radius".into(), radius);
            next.insert("falloff".into(), falloff);
            next.insert(
                "intensity".into(),
                signal_default_f32(&intensity)
                    .map(|value| serde_json::json!(value))
                    .unwrap_or(intensity),
            );
            if signal_implies_looping(&obj.get("intensity").cloned().unwrap_or_default()) {
                next.insert("pulse_speed".into(), serde_json::json!(1.0));
            }
            *obj = next;
        }
        "edge_shadow" => {
            let radius = pattern
                .get("radius")
                .cloned()
                .unwrap_or(serde_json::json!(2));
            let falloff = pattern
                .get("falloff")
                .cloned()
                .unwrap_or(serde_json::json!("quadratic"));
            let edges = pattern
                .get("edges")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let mut next = serde_json::Map::new();
            next.insert("type".into(), serde_json::json!("ambient_occlusion"));
            next.insert("radius".into(), radius);
            next.insert("falloff".into(), falloff);
            next.insert(
                "intensity".into(),
                signal_default_f32(&intensity)
                    .map(|value| serde_json::json!(value))
                    .unwrap_or(intensity),
            );
            next.insert("shadow_color".into(), color);
            next.insert("edges".into(), serde_json::json!(edges_to_ao(&edges)));
            *obj = next;
        }
        "radial_from_corner" => {
            let radius = pattern
                .get("radius")
                .cloned()
                .unwrap_or(serde_json::json!(6));
            let softness = pattern
                .get("softness")
                .cloned()
                .unwrap_or(serde_json::json!(0.55));
            let edge_firmness = pattern
                .get("edge_firmness")
                .cloned()
                .unwrap_or(serde_json::json!(0.2));
            let source = pattern
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or("center");
            let mut next = serde_json::Map::new();
            next.insert("type".into(), serde_json::json!("diffusion"));
            next.insert(
                "source".into(),
                serde_json::json!(source_to_diffusion(source)),
            );
            next.insert("color".into(), color);
            next.insert("radius".into(), radius);
            next.insert("softness".into(), softness);
            next.insert("edge_firmness".into(), edge_firmness);
            next.insert(
                "intensity".into(),
                signal_default_f32(&intensity)
                    .map(|value| serde_json::json!(value))
                    .unwrap_or(intensity),
            );
            if let Some(apply_to) = apply_to {
                next.insert("apply_to".into(), apply_to);
            }
            if signal_implies_looping(&obj.get("intensity").cloned().unwrap_or_default()) {
                next.insert("mode".into(), serde_json::json!("breath"));
                next.insert("drift_speed".into(), serde_json::json!(1.0));
                next.insert("drift_amount".into(), serde_json::json!(0.2));
            }
            *obj = next;
        }
        _ => {}
    }
}

// <FILE>tui-vfx-style/src/models/cls_spatial_shader_type.rs</FILE> - <DESC>Enum of all spatial shaders with documentation methods</DESC>
// <VERS>END OF VERSION: 2.3.0</VERS>
