// <FILE>tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Lower grouped V3 spatial shader families back into the executable legacy runtime surface</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — VfxRevealDirection and RevealDirection both alias tui_vfx_geometry::WipeDirection, so the explicit From<VfxRevealDirection> for RevealDirection impl is now an orphan-rule violation (and unnecessary — the implicit identity From<T> for T covers any .into() call between them).</WCTX>
// <CLOG>0.4.0: drop the now-redundant From<VfxRevealDirection> for RevealDirection impl. Both types alias the same WipeDirection after the audit-recommended unification, so any .into() conversion route is already covered by the implicit identity From; the explicit impl violates Rust's orphan rule for two same-type aliases.
// 0.3.0: lower V3 traveling-band head_tail colors into backward-compatible legacy shader head/tail fields</CLOG>

//! Lower grouped V3 spatial shader families back into the executable legacy
//! runtime surface.

use crate::models::cls_trace_path_shader::TraceTailMode;
use crate::models::v3::{
    TryLowerV3SpatialShaderError, VfxAffordanceWakeZone, VfxConcealedLightMode,
    VfxConcealedLightSource, VfxCursorMode, VfxCursorPrimary, VfxCursorTrail, VfxDiffusionMode,
    VfxDiffusionSource, VfxEdgeDistortionAxis, VfxEdgeDistortionBehavior, VfxFocusFieldShape,
    VfxGradientRevealBehavior, VfxGuidanceCueApplyTo, VfxGuidanceCueBehavior,
    VfxMaterialLightApplyTo, VfxMaterialLightBehavior, VfxMotionFieldBehavior,
    VfxMotionFieldDirection, VfxProgressEmphasisApplyTo, VfxProgressEmphasisDirection,
    VfxProgressEmphasisMode, VfxProgressEmphasisRowMask, VfxProgressEmphasisTextContrast,
    VfxSpatialComposedPrimitive, VfxSpatialPrimitive, VfxSpatialShaderFamily,
    VfxStochasticTextureBehavior, VfxStripeMotionBehavior, VfxSurfaceDepthBehavior,
    VfxSurfaceDepthEdges, VfxSurfaceDepthLightDirection, VfxTextureSegmentMode, VfxTextureTarget,
    VfxTracePathTailMode, VfxTravelingBandApplyTo, VfxTravelingBandBehavior, VfxTravelingBandColor,
    VfxTravelingBandDirection, VfxWayfindingNode,
};
use crate::models::{
    AOEdges, AffordanceWakeApplyTo, AffordanceWakeShader, AffordanceWakeZone,
    AmbientOcclusionShader, ApplyToColor, BarberPoleShader, BevelShader, BorderSweepShader,
    ChromaticEdgeShader, ConcealedLightApplyTo, ConcealedLightMode, ConcealedLightShader,
    ConcealedLightSource, CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
    DiffusionApplyTo, DiffusionMode, DiffusionShader, DiffusionSource, EdgeSheenApplyTo,
    EdgeSheenShader, FocusFieldApplyTo, FocusFieldShader, FocusFieldShape,
    FocusedRowGradientShader, GlistenApplyTo, GlistenBandShader, GlistenDirection,
    GlitchLinesShader, GlowShader, HighlighterApplyTo, HighlighterDirection, HighlighterMode,
    HighlighterRowMask, HighlighterShader, LightDirection, LinearGradientShader, NeonFlickerShader,
    OrbitShader, PulseWaveShader, RadarShader, RadialSpiralShader, ReflectShader, RevealWipeShader,
    SegmentMode, ShakeAxis, SparkleTarget, SpatialShaderType, StochasticSparkleShader,
    SubCellShakeShader, TextContrast, TraceApplyTo, TracePathShader, TracePropagationShader,
    WayfindingNode, WayfindingNodeApplyTo, WayfindingNodeShader,
};

/// Lower a grouped V3 spatial shader family back into the executable legacy
/// [`SpatialShaderType`] surface.
pub fn try_lower_v3_spatial_shader_family(
    family: &VfxSpatialShaderFamily,
) -> Result<SpatialShaderType, TryLowerV3SpatialShaderError> {
    match family {
        VfxSpatialShaderFamily::Primitive(primitive) => try_lower_primitive(primitive),
        VfxSpatialShaderFamily::ComposedPrimitive(composed) => try_lower_composed(composed),
    }
}

impl VfxSpatialShaderFamily {
    /// Attempt to lower this grouped V3 family back into the executable legacy
    /// [`SpatialShaderType`] surface.
    pub fn try_to_legacy_spatial_shader(
        &self,
    ) -> Result<SpatialShaderType, TryLowerV3SpatialShaderError> {
        try_lower_v3_spatial_shader_family(self)
    }
}

fn try_lower_primitive(
    primitive: &VfxSpatialPrimitive,
) -> Result<SpatialShaderType, TryLowerV3SpatialShaderError> {
    Ok(match primitive {
        VfxSpatialPrimitive::SurfaceDepth(shader) => SpatialShaderType::from(shader),
        VfxSpatialPrimitive::MotionField(shader) => SpatialShaderType::from(shader),
        VfxSpatialPrimitive::EdgeDistortion(shader) => SpatialShaderType::from(shader),
        VfxSpatialPrimitive::GradientReveal(shader) => SpatialShaderType::from(shader),
    })
}

fn try_lower_composed(
    composed: &VfxSpatialComposedPrimitive,
) -> Result<SpatialShaderType, TryLowerV3SpatialShaderError> {
    match composed {
        VfxSpatialComposedPrimitive::TravelingBand(shader) => try_lower_traveling_band(shader),
        VfxSpatialComposedPrimitive::ProgressEmphasis(shader) => {
            Ok(SpatialShaderType::from(shader))
        }
        VfxSpatialComposedPrimitive::MaterialLight(shader) => Ok(SpatialShaderType::from(shader)),
        VfxSpatialComposedPrimitive::GuidanceCue(shader) => Ok(SpatialShaderType::from(shader)),
        VfxSpatialComposedPrimitive::StochasticTexture(shader) => {
            Ok(SpatialShaderType::from(shader))
        }
        VfxSpatialComposedPrimitive::Cursor(shader) => Ok(SpatialShaderType::from(shader)),
        VfxSpatialComposedPrimitive::StripeMotion(shader) => Ok(SpatialShaderType::from(shader)),
    }
}

fn try_lower_traveling_band(
    shader: &crate::models::v3::VfxTravelingBandShader,
) -> Result<SpatialShaderType, TryLowerV3SpatialShaderError> {
    match &shader.behavior {
        VfxTravelingBandBehavior::Border {
            length,
            position_binding,
        } => {
            let (color, head, tail) = legacy_traveling_band_colors(&shader.color);
            Ok(SpatialShaderType::BorderSweep(BorderSweepShader {
                speed: shader.speed,
                length: *length,
                color,
                head,
                tail,
                position_binding: position_binding.clone(),
            }))
        }
        VfxTravelingBandBehavior::Reflect { gap, width } => {
            let (color, head, tail) = legacy_traveling_band_colors(&shader.color);
            Ok(SpatialShaderType::Reflect(ReflectShader {
                speed: shader.speed,
                color,
                head,
                tail,
                gap: *gap,
                width: *width,
            }))
        }
        VfxTravelingBandBehavior::GlistenBand {
            band_width,
            angle_deg,
            direction,
            direction_binding,
            repeat_count,
            apply_to,
            blend_strength,
            blend_strength_binding,
            speed_binding,
        } => {
            let (head, tail) = match &shader.color {
                VfxTravelingBandColor::Solid { color } => (color.clone(), color.clone()),
                VfxTravelingBandColor::HeadTail { head, tail } => (head.clone(), tail.clone()),
            };
            Ok(SpatialShaderType::GlistenBand(GlistenBandShader {
                speed: shader.speed,
                speed_binding: speed_binding.clone(),
                band_width: *band_width,
                angle_deg: *angle_deg,
                head,
                tail,
                direction: (*direction).into(),
                direction_binding: direction_binding.clone(),
                repeat_count: *repeat_count,
                apply_to: (*apply_to).into(),
                blend_strength: *blend_strength,
                blend_strength_binding: blend_strength_binding.clone(),
            }))
        }
        VfxTravelingBandBehavior::TracePropagation {
            grid_spacing,
            line_width,
            tail_length,
            intensity,
            origin,
            apply_to,
        } => {
            let (color, head, tail) = legacy_traveling_band_colors(&shader.color);
            Ok(SpatialShaderType::TracePropagation(
                TracePropagationShader {
                    color,
                    head,
                    tail,
                    speed: shader.speed,
                    grid_spacing: *grid_spacing,
                    line_width: *line_width,
                    tail_length: *tail_length,
                    intensity: *intensity,
                    origin: *origin,
                    apply_to: (*apply_to).into(),
                },
            ))
        }
        VfxTravelingBandBehavior::TracePath {
            tail_length,
            vertical_weight,
            thickness,
            intensity,
            junction_boost,
            junction_glow,
            tail_mode,
            apply_to,
            paths,
        } => {
            let (color, head, tail) = legacy_traveling_band_colors(&shader.color);
            Ok(SpatialShaderType::TracePath(TracePathShader {
                color,
                head,
                tail,
                speed: shader.speed,
                tail_length: *tail_length,
                vertical_weight: *vertical_weight,
                thickness: *thickness,
                intensity: *intensity,
                junction_boost: *junction_boost,
                junction_glow: *junction_glow,
                tail_mode: (*tail_mode).into(),
                apply_to: (*apply_to).into(),
                paths: paths.clone(),
            }))
        }
    }
}

fn legacy_traveling_band_colors(
    color: &VfxTravelingBandColor,
) -> (
    crate::models::ColorConfig,
    Option<crate::models::ColorConfig>,
    Option<crate::models::ColorConfig>,
) {
    match color {
        VfxTravelingBandColor::Solid { color } => (color.clone(), None, None),
        VfxTravelingBandColor::HeadTail { head, tail } => {
            (head.clone(), Some(head.clone()), Some(tail.clone()))
        }
    }
}

impl From<&crate::models::v3::VfxProgressEmphasisShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxProgressEmphasisShader) -> Self {
        SpatialShaderType::Highlighter(HighlighterShader {
            color: shader.color.clone(),
            apply_to: shader.apply_to.into(),
            text_contrast: shader.text_contrast.clone().into(),
            mode: shader.mode.into(),
            band_width: shader.band_width,
            soft_edge: shader.soft_edge,
            blend_strength: shader.blend_strength,
            blend_strength_binding: shader.blend_strength_binding.clone(),
            speed: shader.speed,
            speed_binding: shader.speed_binding.clone(),
            direction: shader.direction.into(),
            direction_binding: shader.direction_binding.clone(),
            row_mask: shader.row_mask.into(),
        })
    }
}

impl From<&crate::models::v3::VfxMaterialLightShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxMaterialLightShader) -> Self {
        match &shader.behavior {
            VfxMaterialLightBehavior::Diffusion {
                source,
                color,
                radius,
                softness,
                edge_firmness,
                falloff,
                intensity,
                apply_to,
                mode,
                drift_speed,
                drift_amount,
            } => SpatialShaderType::Diffusion(DiffusionShader {
                source: (*source).into(),
                color: color.clone(),
                radius: *radius,
                softness: *softness,
                edge_firmness: *edge_firmness,
                falloff: *falloff,
                intensity: intensity.clone(),
                apply_to: (*apply_to).into(),
                mode: (*mode).into(),
                drift_speed: *drift_speed,
                drift_amount: *drift_amount,
            }),
            VfxMaterialLightBehavior::ConcealedLight {
                source,
                color,
                spread,
                edge_width,
                falloff,
                intensity,
                apply_to,
                mode,
                pulse_speed,
                source_cutoff,
            } => SpatialShaderType::ConcealedLight(ConcealedLightShader {
                source: (*source).into(),
                color: color.clone(),
                spread: *spread,
                edge_width: *edge_width,
                falloff: *falloff,
                intensity: *intensity,
                apply_to: (*apply_to).into(),
                mode: (*mode).into(),
                pulse_speed: *pulse_speed,
                source_cutoff: *source_cutoff,
            }),
            VfxMaterialLightBehavior::EdgeSheen {
                color,
                speed,
                band_width,
                edge_width,
                intensity,
                corner_boost,
                apply_to,
            } => SpatialShaderType::EdgeSheen(EdgeSheenShader {
                color: color.clone(),
                speed: *speed,
                band_width: *band_width,
                edge_width: *edge_width,
                intensity: *intensity,
                corner_boost: *corner_boost,
                apply_to: (*apply_to).into(),
            }),
        }
    }
}

impl From<&crate::models::v3::VfxGuidanceCueShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxGuidanceCueShader) -> Self {
        match &shader.behavior {
            VfxGuidanceCueBehavior::FocusedRow {
                selected_row,
                selected_row_binding,
                selected_row_ratio,
                selected_row_ratio_binding,
                falloff_distance,
                bright_color,
                dim_color,
                apply_to,
            } => SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader {
                selected_row: *selected_row,
                selected_row_binding: selected_row_binding.clone(),
                selected_row_ratio: *selected_row_ratio,
                selected_row_ratio_binding: selected_row_ratio_binding.clone(),
                falloff_distance: *falloff_distance,
                bright_color: bright_color.clone(),
                dim_color: dim_color.clone(),
                apply_to: (*apply_to).into(),
            }),
            VfxGuidanceCueBehavior::FocusField {
                color,
                shape,
                center_x,
                center_y,
                center_x_binding,
                center_y_binding,
                radius_x,
                radius_y,
                rect_x,
                rect_y,
                rect_width,
                rect_height,
                rect_x_binding,
                rect_y_binding,
                rect_width_binding,
                rect_height_binding,
                feather,
                falloff,
                intensity,
                apply_to,
                pulse_speed,
            } => SpatialShaderType::FocusField(FocusFieldShader {
                color: color.clone(),
                shape: (*shape).into(),
                center_x: *center_x,
                center_y: *center_y,
                center_x_binding: center_x_binding.clone(),
                center_y_binding: center_y_binding.clone(),
                radius_x: *radius_x,
                radius_y: *radius_y,
                rect_x: *rect_x,
                rect_y: *rect_y,
                rect_width: *rect_width,
                rect_height: *rect_height,
                rect_x_binding: rect_x_binding.clone(),
                rect_y_binding: rect_y_binding.clone(),
                rect_width_binding: rect_width_binding.clone(),
                rect_height_binding: rect_height_binding.clone(),
                feather: *feather,
                falloff: *falloff,
                intensity: *intensity,
                apply_to: (*apply_to).into(),
                pulse_speed: *pulse_speed,
            }),
            VfxGuidanceCueBehavior::AffordanceWake {
                color,
                zone,
                radius,
                falloff,
                progress,
                progress_binding,
                rest_intensity,
                peak_intensity,
                apply_to,
            } => SpatialShaderType::AffordanceWake(AffordanceWakeShader {
                color: color.clone(),
                zone: (*zone).into(),
                radius: *radius,
                falloff: *falloff,
                progress: *progress,
                progress_binding: progress_binding.clone(),
                rest_intensity: *rest_intensity,
                peak_intensity: *peak_intensity,
                apply_to: (*apply_to).into(),
            }),
            VfxGuidanceCueBehavior::WayfindingNode {
                color,
                nodes,
                radius,
                intensity,
                current_index,
                current_index_binding,
                previous_strength,
                future_strength,
                pulse_speed,
                apply_to,
            } => SpatialShaderType::WayfindingNode(WayfindingNodeShader {
                color: color.clone(),
                nodes: nodes.iter().copied().map(Into::into).collect(),
                radius: *radius,
                intensity: *intensity,
                current_index: *current_index,
                current_index_binding: current_index_binding.clone(),
                previous_strength: *previous_strength,
                future_strength: *future_strength,
                pulse_speed: *pulse_speed,
                apply_to: (*apply_to).into(),
            }),
        }
    }
}

impl From<&crate::models::v3::VfxSurfaceDepthShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxSurfaceDepthShader) -> Self {
        match &shader.behavior {
            VfxSurfaceDepthBehavior::AmbientOcclusion {
                intensity,
                radius,
                edges,
                falloff,
                shadow_color,
            } => SpatialShaderType::AmbientOcclusion(AmbientOcclusionShader {
                intensity: *intensity,
                radius: *radius,
                edges: (*edges).into(),
                falloff: *falloff,
                shadow_color: shadow_color.clone(),
            }),
            VfxSurfaceDepthBehavior::Bevel {
                light_direction,
                highlight_intensity,
                shadow_intensity,
                edge_width,
            } => SpatialShaderType::Bevel(BevelShader {
                light_direction: (*light_direction).into(),
                highlight_intensity: *highlight_intensity,
                shadow_intensity: *shadow_intensity,
                edge_width: *edge_width,
            }),
            VfxSurfaceDepthBehavior::Glow {
                color,
                radius,
                falloff,
                intensity,
                pulse_speed,
            } => SpatialShaderType::Glow(GlowShader {
                color: color.clone(),
                radius: *radius,
                falloff: *falloff,
                intensity: *intensity,
                pulse_speed: *pulse_speed,
            }),
        }
    }
}

impl From<&crate::models::v3::VfxMotionFieldShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxMotionFieldShader) -> Self {
        match &shader.behavior {
            VfxMotionFieldBehavior::PulseWave {
                frequency,
                frequency_binding,
                speed,
                color,
                direction,
                wavelength,
            } => SpatialShaderType::PulseWave(PulseWaveShader {
                frequency: *frequency,
                frequency_binding: frequency_binding.clone(),
                speed: *speed,
                color: color.clone(),
                direction: (*direction).into(),
                wavelength: *wavelength,
            }),
            VfxMotionFieldBehavior::Radar {
                speed,
                tail_length,
                color,
            } => SpatialShaderType::Radar(RadarShader {
                speed: *speed,
                tail_length: *tail_length,
                color: color.clone(),
            }),
            VfxMotionFieldBehavior::Orbit {
                speed,
                dot_count,
                color,
            } => SpatialShaderType::Orbit(OrbitShader {
                speed: *speed,
                dot_count: *dot_count,
                color: color.clone(),
            }),
            VfxMotionFieldBehavior::RadialSpiral {
                arms,
                radial_frequency,
                radial_power,
                speed,
                blend_strength,
                color,
            } => SpatialShaderType::RadialSpiral(RadialSpiralShader {
                arms: *arms,
                radial_frequency: *radial_frequency,
                radial_power: *radial_power,
                speed: *speed,
                blend_strength: *blend_strength,
                color: color.clone(),
            }),
            VfxMotionFieldBehavior::TerminalWater { shader } => {
                SpatialShaderType::TerminalWater(shader.clone())
            }
        }
    }
}

impl From<&crate::models::v3::VfxEdgeDistortionShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxEdgeDistortionShader) -> Self {
        match &shader.behavior {
            VfxEdgeDistortionBehavior::GlitchLines {
                seed,
                intensity,
                max_lines,
                speed,
                flash_chance,
                pulse_color,
                pulse_speed,
                italic_on_flash,
                flash_hold,
                noise_type,
            } => SpatialShaderType::GlitchLines(GlitchLinesShader {
                seed: *seed,
                intensity: *intensity,
                max_lines: *max_lines,
                speed: *speed,
                flash_chance: *flash_chance,
                pulse_color: pulse_color.clone(),
                pulse_speed: *pulse_speed,
                italic_on_flash: *italic_on_flash,
                flash_hold: *flash_hold,
                noise_type: *noise_type,
            }),
            VfxEdgeDistortionBehavior::ChromaticEdge {
                intensity,
                edge_width,
                horizontal,
            } => SpatialShaderType::ChromaticEdge(ChromaticEdgeShader {
                intensity: *intensity,
                edge_width: *edge_width,
                horizontal: *horizontal,
            }),
            VfxEdgeDistortionBehavior::SubCellShake {
                amplitude,
                frequency,
                axis,
                chromatic,
                seed,
                edge_only,
                edge_width,
            } => SpatialShaderType::SubCellShake(SubCellShakeShader {
                amplitude: *amplitude,
                frequency: *frequency,
                axis: (*axis).into(),
                chromatic: *chromatic,
                seed: *seed,
                edge_only: *edge_only,
                edge_width: *edge_width,
            }),
        }
    }
}

impl From<&crate::models::v3::VfxGradientRevealShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxGradientRevealShader) -> Self {
        match &shader.behavior {
            VfxGradientRevealBehavior::LinearGradient {
                gradient,
                angle_deg,
                apply_to,
                intensity,
            } => SpatialShaderType::LinearGradient(LinearGradientShader {
                gradient: gradient.clone(),
                angle_deg: *angle_deg,
                apply_to: *apply_to,
                intensity: *intensity,
            }),
            VfxGradientRevealBehavior::RevealWipe { direction } => {
                SpatialShaderType::RevealWipe(RevealWipeShader {
                    direction: (*direction).into(),
                })
            }
        }
    }
}

impl From<&crate::models::v3::VfxStochasticTextureShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxStochasticTextureShader) -> Self {
        match &shader.behavior {
            VfxStochasticTextureBehavior::NeonFlicker {
                stability,
                seed,
                segment,
                dim_amount,
                speed,
                flash_chance,
                decay_rate,
                noise_type,
            } => SpatialShaderType::NeonFlicker(NeonFlickerShader {
                stability: *stability,
                seed: *seed,
                segment: (*segment).into(),
                dim_amount: *dim_amount,
                speed: *speed,
                flash_chance: *flash_chance,
                decay_rate: *decay_rate,
                noise_type: *noise_type,
            }),
            VfxStochasticTextureBehavior::StochasticSparkle {
                sparkle_density,
                brightness_boost,
                speed,
                seed,
                apply_to,
                noise_type,
            } => SpatialShaderType::StochasticSparkle(StochasticSparkleShader {
                sparkle_density: *sparkle_density,
                brightness_boost: *brightness_boost,
                speed: *speed,
                seed: *seed,
                apply_to: (*apply_to).into(),
                noise_type: *noise_type,
            }),
        }
    }
}

impl From<&crate::models::v3::VfxCursorShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxCursorShader) -> Self {
        SpatialShaderType::Cursor(CursorShader {
            mode: shader.mode.into(),
            tint: shader.tint.clone(),
            primary: shader.primary.as_ref().map(Into::into),
            trail: shader.trail.iter().map(Into::into).collect(),
        })
    }
}

impl From<&crate::models::v3::VfxStripeMotionShader> for SpatialShaderType {
    fn from(shader: &crate::models::v3::VfxStripeMotionShader) -> Self {
        match &shader.behavior {
            VfxStripeMotionBehavior::BarberPole {
                speed,
                stripe_width,
                gap_width,
                color,
            } => SpatialShaderType::BarberPole(BarberPoleShader {
                speed: *speed,
                stripe_width: *stripe_width,
                gap_width: *gap_width,
                color: color.clone(),
            }),
        }
    }
}

impl From<VfxProgressEmphasisApplyTo> for HighlighterApplyTo {
    fn from(v: VfxProgressEmphasisApplyTo) -> Self {
        match v {
            VfxProgressEmphasisApplyTo::Background => Self::Background,
            VfxProgressEmphasisApplyTo::Foreground => Self::Foreground,
            VfxProgressEmphasisApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxProgressEmphasisTextContrast> for TextContrast {
    fn from(v: VfxProgressEmphasisTextContrast) -> Self {
        match v {
            VfxProgressEmphasisTextContrast::Black => Self::Black,
            VfxProgressEmphasisTextContrast::Preserve => Self::Preserve,
            VfxProgressEmphasisTextContrast::Explicit { color } => Self::Explicit { color },
        }
    }
}
impl From<VfxProgressEmphasisMode> for HighlighterMode {
    fn from(v: VfxProgressEmphasisMode) -> Self {
        match v {
            VfxProgressEmphasisMode::Fill => Self::Fill,
            VfxProgressEmphasisMode::Band => Self::Band,
        }
    }
}
impl From<VfxProgressEmphasisDirection> for HighlighterDirection {
    fn from(v: VfxProgressEmphasisDirection) -> Self {
        match v {
            VfxProgressEmphasisDirection::Forward => Self::Forward,
            VfxProgressEmphasisDirection::Reverse => Self::Reverse,
            VfxProgressEmphasisDirection::TopDown => Self::TopDown,
            VfxProgressEmphasisDirection::BottomUp => Self::BottomUp,
            VfxProgressEmphasisDirection::CenterOut => Self::CenterOut,
            VfxProgressEmphasisDirection::EdgesIn => Self::EdgesIn,
        }
    }
}
impl From<VfxProgressEmphasisRowMask> for HighlighterRowMask {
    fn from(v: VfxProgressEmphasisRowMask) -> Self {
        match v {
            VfxProgressEmphasisRowMask::AllRows => Self::AllRows,
            VfxProgressEmphasisRowMask::FirstRow => Self::FirstRow,
            VfxProgressEmphasisRowMask::LastRow => Self::LastRow,
            VfxProgressEmphasisRowMask::TopAndBottom => Self::TopAndBottom,
            VfxProgressEmphasisRowMask::Range { start, end } => Self::Range { start, end },
        }
    }
}
impl From<VfxDiffusionSource> for DiffusionSource {
    fn from(v: VfxDiffusionSource) -> Self {
        match v {
            VfxDiffusionSource::Center => Self::Center,
            VfxDiffusionSource::Top => Self::Top,
            VfxDiffusionSource::Bottom => Self::Bottom,
            VfxDiffusionSource::Left => Self::Left,
            VfxDiffusionSource::Right => Self::Right,
            VfxDiffusionSource::TopLeft => Self::TopLeft,
            VfxDiffusionSource::TopRight => Self::TopRight,
            VfxDiffusionSource::BottomLeft => Self::BottomLeft,
            VfxDiffusionSource::BottomRight => Self::BottomRight,
        }
    }
}
impl From<VfxMaterialLightApplyTo> for DiffusionApplyTo {
    fn from(v: VfxMaterialLightApplyTo) -> Self {
        match v {
            VfxMaterialLightApplyTo::Foreground => Self::Foreground,
            VfxMaterialLightApplyTo::Background => Self::Background,
            VfxMaterialLightApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxMaterialLightApplyTo> for ConcealedLightApplyTo {
    fn from(v: VfxMaterialLightApplyTo) -> Self {
        match v {
            VfxMaterialLightApplyTo::Foreground => Self::Foreground,
            VfxMaterialLightApplyTo::Background => Self::Background,
            VfxMaterialLightApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxMaterialLightApplyTo> for EdgeSheenApplyTo {
    fn from(v: VfxMaterialLightApplyTo) -> Self {
        match v {
            VfxMaterialLightApplyTo::Foreground => Self::Foreground,
            VfxMaterialLightApplyTo::Background => Self::Background,
            VfxMaterialLightApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxDiffusionMode> for DiffusionMode {
    fn from(v: VfxDiffusionMode) -> Self {
        match v {
            VfxDiffusionMode::Static => Self::Static,
            VfxDiffusionMode::WarmDrift => Self::WarmDrift,
            VfxDiffusionMode::CoolDrift => Self::CoolDrift,
            VfxDiffusionMode::Breath => Self::Breath,
        }
    }
}
impl From<VfxConcealedLightSource> for ConcealedLightSource {
    fn from(v: VfxConcealedLightSource) -> Self {
        match v {
            VfxConcealedLightSource::Top => Self::Top,
            VfxConcealedLightSource::Bottom => Self::Bottom,
            VfxConcealedLightSource::Left => Self::Left,
            VfxConcealedLightSource::Right => Self::Right,
        }
    }
}
impl From<VfxConcealedLightMode> for ConcealedLightMode {
    fn from(v: VfxConcealedLightMode) -> Self {
        match v {
            VfxConcealedLightMode::Static => Self::Static,
            VfxConcealedLightMode::Pulse => Self::Pulse,
            VfxConcealedLightMode::Drift => Self::Drift,
        }
    }
}
impl From<VfxGuidanceCueApplyTo> for ApplyToColor {
    fn from(v: VfxGuidanceCueApplyTo) -> Self {
        match v {
            VfxGuidanceCueApplyTo::Foreground => Self::Foreground,
            VfxGuidanceCueApplyTo::Background => Self::Background,
            VfxGuidanceCueApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxGuidanceCueApplyTo> for FocusFieldApplyTo {
    fn from(v: VfxGuidanceCueApplyTo) -> Self {
        match v {
            VfxGuidanceCueApplyTo::Foreground => Self::Foreground,
            VfxGuidanceCueApplyTo::Background => Self::Background,
            VfxGuidanceCueApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxGuidanceCueApplyTo> for AffordanceWakeApplyTo {
    fn from(v: VfxGuidanceCueApplyTo) -> Self {
        match v {
            VfxGuidanceCueApplyTo::Foreground => Self::Foreground,
            VfxGuidanceCueApplyTo::Background => Self::Background,
            VfxGuidanceCueApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxGuidanceCueApplyTo> for WayfindingNodeApplyTo {
    fn from(v: VfxGuidanceCueApplyTo) -> Self {
        match v {
            VfxGuidanceCueApplyTo::Foreground => Self::Foreground,
            VfxGuidanceCueApplyTo::Background => Self::Background,
            VfxGuidanceCueApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxAffordanceWakeZone> for AffordanceWakeZone {
    fn from(v: VfxAffordanceWakeZone) -> Self {
        match v {
            VfxAffordanceWakeZone::AllEdges => Self::AllEdges,
            VfxAffordanceWakeZone::Corners => Self::Corners,
            VfxAffordanceWakeZone::LeftRail => Self::LeftRail,
            VfxAffordanceWakeZone::RightRail => Self::RightRail,
            VfxAffordanceWakeZone::TopRail => Self::TopRail,
            VfxAffordanceWakeZone::BottomRail => Self::BottomRail,
        }
    }
}
impl From<VfxFocusFieldShape> for FocusFieldShape {
    fn from(v: VfxFocusFieldShape) -> Self {
        match v {
            VfxFocusFieldShape::Ellipse => Self::Ellipse,
            VfxFocusFieldShape::Rect => Self::Rect,
        }
    }
}
impl From<VfxWayfindingNode> for WayfindingNode {
    fn from(v: VfxWayfindingNode) -> Self {
        Self { x: v.x, y: v.y }
    }
}
impl From<VfxSurfaceDepthEdges> for AOEdges {
    fn from(v: VfxSurfaceDepthEdges) -> Self {
        match v {
            VfxSurfaceDepthEdges::BottomRight => Self::BottomRight,
            VfxSurfaceDepthEdges::TopLeft => Self::TopLeft,
            VfxSurfaceDepthEdges::All => Self::All,
            VfxSurfaceDepthEdges::Inner => Self::Inner,
        }
    }
}
impl From<VfxSurfaceDepthLightDirection> for LightDirection {
    fn from(v: VfxSurfaceDepthLightDirection) -> Self {
        match v {
            VfxSurfaceDepthLightDirection::TopLeft => Self::TopLeft,
            VfxSurfaceDepthLightDirection::TopRight => Self::TopRight,
            VfxSurfaceDepthLightDirection::BottomLeft => Self::BottomLeft,
            VfxSurfaceDepthLightDirection::BottomRight => Self::BottomRight,
            VfxSurfaceDepthLightDirection::Top => Self::Top,
            VfxSurfaceDepthLightDirection::Bottom => Self::Bottom,
            VfxSurfaceDepthLightDirection::Left => Self::Left,
            VfxSurfaceDepthLightDirection::Right => Self::Right,
        }
    }
}
impl From<VfxMotionFieldDirection> for crate::models::WaveDirection {
    fn from(v: VfxMotionFieldDirection) -> Self {
        match v {
            VfxMotionFieldDirection::Horizontal => Self::Horizontal,
            VfxMotionFieldDirection::Vertical => Self::Vertical,
            VfxMotionFieldDirection::Radial => Self::Radial,
            VfxMotionFieldDirection::Diagonal => Self::Diagonal,
        }
    }
}
impl From<VfxEdgeDistortionAxis> for ShakeAxis {
    fn from(v: VfxEdgeDistortionAxis) -> Self {
        match v {
            VfxEdgeDistortionAxis::Horizontal => Self::Horizontal,
            VfxEdgeDistortionAxis::Vertical => Self::Vertical,
            VfxEdgeDistortionAxis::Both => Self::Both,
        }
    }
}
// (From<VfxRevealDirection> for RevealDirection removed in 0.4.0 — both
// types alias tui_vfx_geometry::WipeDirection after the audit-recommended
// unification, so the implicit identity From<T> for T covers any
// .into() conversion route, and an explicit impl would violate Rust's
// orphan rule for two same-type aliases.)
impl From<VfxTextureSegmentMode> for SegmentMode {
    fn from(v: VfxTextureSegmentMode) -> Self {
        match v {
            VfxTextureSegmentMode::Cell => Self::Cell,
            VfxTextureSegmentMode::Row => Self::Row,
            VfxTextureSegmentMode::Column => Self::Column,
        }
    }
}
impl From<VfxTextureTarget> for SparkleTarget {
    fn from(v: VfxTextureTarget) -> Self {
        match v {
            VfxTextureTarget::Foreground => Self::Foreground,
            VfxTextureTarget::Background => Self::Background,
            VfxTextureTarget::Both => Self::Both,
        }
    }
}
impl From<VfxCursorMode> for CursorShaderMode {
    fn from(v: VfxCursorMode) -> Self {
        match v {
            VfxCursorMode::Off => Self::Off,
            VfxCursorMode::Tint => Self::Tint,
            VfxCursorMode::Ghost => Self::Ghost,
        }
    }
}
impl From<&VfxCursorPrimary> for CursorShaderPrimary {
    fn from(v: &VfxCursorPrimary) -> Self {
        Self {
            position: v.position,
            alpha: v.alpha,
        }
    }
}
impl From<&VfxCursorTrail> for CursorShaderTrail {
    fn from(v: &VfxCursorTrail) -> Self {
        Self {
            position: v.position,
            alpha: v.alpha,
            glyph: v.glyph.clone(),
        }
    }
}
impl From<VfxTravelingBandDirection> for GlistenDirection {
    fn from(v: VfxTravelingBandDirection) -> Self {
        match v {
            VfxTravelingBandDirection::Forward => Self::Forward,
            VfxTravelingBandDirection::Reverse => Self::Reverse,
            VfxTravelingBandDirection::PingPong => Self::PingPong,
        }
    }
}
impl From<VfxTravelingBandApplyTo> for GlistenApplyTo {
    fn from(v: VfxTravelingBandApplyTo) -> Self {
        match v {
            VfxTravelingBandApplyTo::Foreground => Self::Foreground,
            VfxTravelingBandApplyTo::Background => Self::Background,
            VfxTravelingBandApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxTravelingBandApplyTo> for TraceApplyTo {
    fn from(v: VfxTravelingBandApplyTo) -> Self {
        match v {
            VfxTravelingBandApplyTo::Foreground => Self::Foreground,
            VfxTravelingBandApplyTo::Background => Self::Background,
            VfxTravelingBandApplyTo::Both => Self::Both,
        }
    }
}
impl From<VfxTracePathTailMode> for TraceTailMode {
    fn from(v: VfxTracePathTailMode) -> Self {
        match v {
            VfxTracePathTailMode::Path => Self::Path,
            VfxTracePathTailMode::Segment => Self::Segment,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/fnc_try_lower_v3_spatial_shader_family.rs</FILE> - <DESC>Lower grouped V3 spatial shader families back into the executable legacy runtime surface</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
