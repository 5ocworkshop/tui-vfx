// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>VERSION: 2.17.0</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>2.17.0: PreparedFilter::apply builds VfxCellContext once per cell and passes &ctx to every variant arm; drops five positional params from the inner dispatch.</CLOG>

use super::cls_prepare_context::PrepareContext;
use crate::filters::cls_animated_glyph_ramp::AnimatedGlyphRamp;
use crate::filters::cls_bracket_emphasis::BracketEmphasis;
use crate::filters::cls_braille_dust::BrailleDust;
use crate::filters::cls_charset_noise::CharsetNoise;
use crate::filters::cls_color_bridged_shade::ColorBridgedShade;
use crate::filters::cls_crt::Crt;
use crate::filters::cls_dim::Dim;
use crate::filters::cls_dot_indicator::DotIndicator;
use crate::filters::cls_edge_grow::EdgeGrow;
use crate::filters::cls_fade_to_canvas::FadeToCanvas;
use crate::filters::cls_glisten_sweep::GlistenSweep;
use crate::filters::cls_glyph_style::{GlyphStyle, GlyphStyleRule as GlyphStyleRuleImpl};
use crate::filters::cls_greyscale::Greyscale;
use crate::filters::cls_hover_bar::HoverBar;
use crate::filters::cls_interlace_curtain::InterlaceCurtain;
use crate::filters::cls_invert::Invert;
use crate::filters::cls_kitt_scanner::KittScanner;
use crate::filters::cls_matrix_rain::{
    MatrixRain, MatrixRainAffectMode, MatrixRainGlyphPreset, MatrixRainMode,
};
use crate::filters::cls_motion_blur::{MotionBlur, MotionDirection};
use crate::filters::cls_pattern_fill::PatternFill;
use crate::filters::cls_pill_button::PillButton;
use crate::filters::cls_rigid_shake::RigidShake;
use crate::filters::cls_scalar_field_glyph_filter::ScalarFieldGlyphFilter;
use crate::filters::cls_shade_scanner::ShadeScanner;
use crate::filters::cls_sub_cell_shake::SubCellShake;
use crate::filters::cls_sub_pixel_bar::{BarDirection, SubPixelBar};
use crate::filters::cls_subcell_light::{
    LightSampleFrom as ImplLightSampleFrom, SubcellLight,
    SubcellLightRenderMode as ImplSubcellLightRenderMode,
};
use crate::filters::cls_tint::Tint;
use crate::filters::cls_glyph_timeline::GlyphTimeline;
use crate::filters::cls_underline_wipe::UnderlineWipe;
use crate::filters::cls_vignette::Vignette;
use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::{
    FilterSpec, GlyphEncoderSpec, GlyphRecolorSpec, PatternType, SamplerRef,
};
use smallvec::SmallVec;
use tui_vfx_style::models::cls_fire_field_signal::FireFieldSignal;
use tui_vfx_style::models::cls_water_field_signal::WaterFieldSignal;
use tui_vfx_types::glyph::GlyphEncoder;
use tui_vfx_types::{Cell, Color, VfxCellContext};

pub(crate) enum PreparedFilter {
    Dim {
        filter: Dim,
        factor: f32,
    },
    Invert(Invert),
    Tint(Tint),
    FadeToCanvas(FadeToCanvas),
    Vignette(Vignette),
    Crt(Crt),
    PatternFill(PatternFill),
    Greyscale(Greyscale),
    BrailleDust(BrailleDust),
    CharsetNoise(CharsetNoise),
    AnimatedGlyphRamp(AnimatedGlyphRamp),
    MatrixRain(MatrixRain),
    InterlaceCurtain(InterlaceCurtain),
    MotionBlur(MotionBlur),
    ColorBridgedShade(ColorBridgedShade),
    SubPixelBar(SubPixelBar),
    SubcellLight(SubcellLight),
    SubCellShake(SubCellShake),
    RigidShake(RigidShake),
    HoverBar(HoverBar),
    UnderlineWipe(UnderlineWipe),
    BracketEmphasis(BracketEmphasis),
    DotIndicator(DotIndicator),
    EdgeGrow(EdgeGrow),
    PillButton(PillButton),
    GlistenSweep(GlistenSweep),
    KittScanner(KittScanner),
    ShadeScanner(ShadeScanner),
    GlyphStyle(GlyphStyle),
    /// [`ScalarFieldGlyphFilter`] backed by a [`WaterFieldSignal`] sampler.
    ScalarFieldGlyphWater(ScalarFieldGlyphFilter<WaterFieldSignal>),
    /// [`ScalarFieldGlyphFilter`] backed by a [`FireFieldSignal`] sampler.
    ScalarFieldGlyphFire(ScalarFieldGlyphFilter<FireFieldSignal>),
    /// Per-cell scripted glyph + color timeline (TTE-style scenes).
    /// See [`crate::filters::cls_glyph_timeline::GlyphTimeline`] for the
    /// trigger model and frame semantics.
    GlyphTimeline(GlyphTimeline),
}

impl PreparedFilter {
    pub(crate) fn apply(
        &self,
        cell: &mut Cell,
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        loop_t: f64,
    ) {
        let ctx = VfxCellContext::new(local_x, local_y, width, height, 0, 0, loop_t);
        match self {
            PreparedFilter::Dim { filter, factor } => {
                let dim_ctx = VfxCellContext::new(local_x, local_y, width, height, 0, 0, *factor as f64);
                filter.apply(cell, &dim_ctx);
            }
            PreparedFilter::Invert(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::Tint(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::FadeToCanvas(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::Vignette(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::Crt(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::PatternFill(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::Greyscale(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::BrailleDust(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::CharsetNoise(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::AnimatedGlyphRamp(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::MatrixRain(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::InterlaceCurtain(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::MotionBlur(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::ColorBridgedShade(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::SubPixelBar(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::SubcellLight(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::SubCellShake(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::RigidShake(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::HoverBar(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::UnderlineWipe(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::BracketEmphasis(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::DotIndicator(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::EdgeGrow(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::PillButton(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::GlistenSweep(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::KittScanner(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::ShadeScanner(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::GlyphStyle(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::ScalarFieldGlyphWater(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::ScalarFieldGlyphFire(filter) => {
                filter.apply(cell, &ctx);
            }
            PreparedFilter::GlyphTimeline(filter) => {
                filter.apply(cell, &ctx);
            }
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        match self {
            PreparedFilter::Dim { .. } => "Dim",
            PreparedFilter::Invert(_) => "Invert",
            PreparedFilter::Tint(_) => "Tint",
            PreparedFilter::FadeToCanvas(_) => "FadeToCanvas",
            PreparedFilter::Vignette(_) => "Vignette",
            PreparedFilter::Crt(_) => "Crt",
            PreparedFilter::PatternFill(_) => "PatternFill",
            PreparedFilter::Greyscale(_) => "Greyscale",
            PreparedFilter::BrailleDust(_) => "BrailleDust",
            PreparedFilter::CharsetNoise(_) => "CharsetNoise",
            PreparedFilter::AnimatedGlyphRamp(_) => "AnimatedGlyphRamp",
            PreparedFilter::MatrixRain(_) => "MatrixRain",
            PreparedFilter::InterlaceCurtain(_) => "InterlaceCurtain",
            PreparedFilter::MotionBlur(_) => "MotionBlur",
            PreparedFilter::ColorBridgedShade(_) => "ColorBridgedShade",
            PreparedFilter::SubPixelBar(_) => "SubPixelBar",
            PreparedFilter::SubcellLight(_) => "SubcellLight",
            PreparedFilter::SubCellShake(_) => "SubCellShake",
            PreparedFilter::RigidShake(_) => "RigidShake",
            PreparedFilter::HoverBar(_) => "HoverBar",
            PreparedFilter::UnderlineWipe(_) => "UnderlineWipe",
            PreparedFilter::BracketEmphasis(_) => "BracketEmphasis",
            PreparedFilter::DotIndicator(_) => "DotIndicator",
            PreparedFilter::EdgeGrow(_) => "EdgeGrow",
            PreparedFilter::PillButton(_) => "PillButton",
            PreparedFilter::GlistenSweep(_) => "GlistenSweep",
            PreparedFilter::KittScanner(_) => "KittScanner",
            PreparedFilter::ShadeScanner(_) => "ShadeScanner",
            PreparedFilter::GlyphStyle(_) => "GlyphStyle",
            PreparedFilter::ScalarFieldGlyphWater(_) => "ScalarFieldGlyphWater",
            PreparedFilter::ScalarFieldGlyphFire(_) => "ScalarFieldGlyphFire",
            PreparedFilter::GlyphTimeline(_) => "GlyphTimeline",
        }
    }
}

/// Convert spec PatternType to filter PatternType
fn convert_pattern_type(spec: &PatternType) -> crate::filters::cls_pattern_fill::PatternType {
    use crate::filters::cls_pattern_fill::PatternType as ImplPatternType;
    match spec {
        PatternType::Single { char } => ImplPatternType::Single { char: *char },
        PatternType::Checkerboard { char_a, char_b } => ImplPatternType::Checkerboard {
            char_a: *char_a,
            char_b: *char_b,
        },
        PatternType::HorizontalLines { line_char, spacing } => ImplPatternType::HorizontalLines {
            line_char: *line_char,
            spacing: *spacing,
        },
        PatternType::VerticalLines { line_char, spacing } => ImplPatternType::VerticalLines {
            line_char: *line_char,
            spacing: *spacing,
        },
    }
}

pub(crate) fn prepare_filter(
    spec: &FilterSpec,
    prepare_ctx: &PrepareContext,
) -> Option<PreparedFilter> {
    // Shadow bindings keep the existing match arms (which reference `loop_t`
    // and `signal_ctx` extensively) untouched. Per-filter BindableValue lifts
    // land in follow-up commits and reach for `prepare_ctx.runtime_params`
    // alongside these.
    let loop_t = prepare_ctx.loop_t;
    let signal_ctx = &prepare_ctx.signal_ctx;
    match spec {
        FilterSpec::None => None,
        FilterSpec::Dim { factor, apply_to } => {
            let evaluated_factor = factor.evaluate(loop_t, signal_ctx).unwrap_or(1.0);
            Some(PreparedFilter::Dim {
                filter: Dim::new(*apply_to),
                factor: evaluated_factor,
            })
        }
        FilterSpec::Invert { apply_to } => Some(PreparedFilter::Invert(Invert::new(*apply_to))),
        FilterSpec::Tint {
            color,
            strength,
            apply_to,
        } => {
            let evaluated_strength = strength.evaluate(loop_t, signal_ctx).unwrap_or(1.0);
            // ColorConfig → tui_vfx_types::Color
            let tint_color: Color = (*color).into();
            Some(PreparedFilter::Tint(Tint {
                color: tint_color,
                strength: evaluated_strength,
                apply_to: *apply_to,
            }))
        }
        FilterSpec::FadeToCanvas {
            canvas_color,
            canvas_color_binding,
            strength,
            apply_to,
        } => {
            let evaluated_strength = strength
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            // Resolve canvas_color_binding to an RGB runtime param when
            // present; missing bindings and non-Rgb kinds fall back to
            // the static canvas_color ColorConfig.
            let canvas: Color = canvas_color_binding
                .as_deref()
                .and_then(|key| prepare_ctx.runtime_params.get_color(key))
                .unwrap_or_else(|| (*canvas_color).into());
            Some(PreparedFilter::FadeToCanvas(FadeToCanvas {
                canvas_color: canvas,
                strength: evaluated_strength,
                apply_to: *apply_to,
            }))
        }
        FilterSpec::Vignette {
            strength,
            radius,
            sides,
            dither_amount,
            temporal_dither_hz,
        } => {
            let evaluated_strength = strength.evaluate(loop_t, signal_ctx).unwrap_or(0.5);
            let evaluated_radius = radius.evaluate(loop_t, signal_ctx).unwrap_or(0.8);
            Some(PreparedFilter::Vignette(
                Vignette::new(evaluated_strength, evaluated_radius)
                    .with_sides(
                        sides
                            .iter()
                            .map(|side| match side {
                                crate::types::cls_filter_spec::VignetteEdge::Top => {
                                    crate::filters::cls_vignette::VignetteEdge::Top
                                }
                                crate::types::cls_filter_spec::VignetteEdge::Bottom => {
                                    crate::filters::cls_vignette::VignetteEdge::Bottom
                                }
                                crate::types::cls_filter_spec::VignetteEdge::Left => {
                                    crate::filters::cls_vignette::VignetteEdge::Left
                                }
                                crate::types::cls_filter_spec::VignetteEdge::Right => {
                                    crate::filters::cls_vignette::VignetteEdge::Right
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                    .with_dither(*dither_amount, *temporal_dither_hz),
            ))
        }
        FilterSpec::Crt {
            scanline_strength,
            glow,
        } => {
            let evaluated_scanline = scanline_strength
                .evaluate(loop_t, signal_ctx)
                .unwrap_or(0.3);
            let evaluated_glow = glow.evaluate(loop_t, signal_ctx).unwrap_or(0.1);
            Some(PreparedFilter::Crt(Crt::new(
                evaluated_scanline,
                evaluated_glow,
            )))
        }
        FilterSpec::PatternFill {
            pattern,
            color,
            only_empty,
        } => {
            let mut filter =
                PatternFill::new(convert_pattern_type(pattern)).only_empty(*only_empty);
            if let Some(color_config) = color {
                let c: Color = (*color_config).into();
                filter = filter.with_fg(c);
            }
            Some(PreparedFilter::PatternFill(filter))
        }
        FilterSpec::Greyscale { strength, apply_to } => {
            let evaluated_strength = strength.evaluate(loop_t, signal_ctx).unwrap_or(1.0);
            Some(PreparedFilter::Greyscale(Greyscale {
                strength: evaluated_strength,
                apply_to: *apply_to,
            }))
        }
        FilterSpec::BrailleDust {
            density,
            hz,
            seed,
            pattern,
            color,
            drift,
        } => {
            use crate::filters::cls_braille_dust::BraillePattern;
            let braille_pattern = match pattern {
                crate::types::cls_filter_spec::BraillePatternType::SingleDot => {
                    BraillePattern::SingleDot
                }
                crate::types::cls_filter_spec::BraillePatternType::OneToTwoDots => {
                    BraillePattern::OneToTwoDots
                }
                crate::types::cls_filter_spec::BraillePatternType::OneToThreeDots => {
                    BraillePattern::OneToThreeDots
                }
                crate::types::cls_filter_spec::BraillePatternType::OneToFourDots => {
                    BraillePattern::OneToFourDots
                }
            };
            let mut filter = BrailleDust::new()
                .with_density(*density)
                .with_hz(*hz)
                .with_seed(*seed)
                .with_pattern(braille_pattern);
            if let Some(color_config) = color {
                let c: Color = (*color_config).into();
                filter = filter.with_fg(c);
            }
            if *drift != 0.0 {
                filter = filter.with_drift(*drift);
            }
            Some(PreparedFilter::BrailleDust(filter))
        }
        FilterSpec::CharsetNoise {
            hz,
            seed,
            jitter,
            affect,
            chars,
            gradient,
        } => {
            use crate::filters::cls_charset_noise::{
                AffectMode, CharsetGradientStop, CharsetNoise as CharsetNoiseFilter,
            };
            let affect_mode = match affect {
                crate::types::cls_filter_spec::CharsetNoiseAffect::All => AffectMode::All,
                crate::types::cls_filter_spec::CharsetNoiseAffect::NonEmpty => AffectMode::NonEmpty,
            };
            // Build gradient stops: prefer explicit gradient, fall back to flat chars
            let stops = if let Some(g) = gradient {
                g.iter()
                    .map(|s| CharsetGradientStop {
                        at: s.at,
                        chars: s.chars.chars().collect(),
                    })
                    .collect()
            } else if let Some(c) = chars {
                vec![CharsetGradientStop {
                    at: 0.0,
                    chars: c.chars().collect(),
                }]
            } else {
                vec![CharsetGradientStop {
                    at: 0.0,
                    chars: "█▓▒░".chars().collect(),
                }]
            };
            Some(PreparedFilter::CharsetNoise(CharsetNoiseFilter::new(
                *seed,
                *hz,
                *jitter,
                affect_mode,
                stops,
            )))
        }
        FilterSpec::AnimatedGlyphRamp {
            glyphs,
            cycles_per_second,
            apply_to,
            affect,
            phase_offset_x_ms,
            phase_offset_y_ms,
            colors,
            color_gradient,
            ease,
        } => {
            use crate::filters::cls_animated_glyph_ramp::{
                AnimatedGlyphRamp as AnimatedGlyphRampFilter,
                AnimatedGlyphRampApplyTo as ImplApplyTo, discrete_color_mode, gradient_color_mode,
            };
            use crate::filters::cls_charset_noise::AffectMode;
            let glyph_chars: Vec<char> = glyphs.chars().collect();
            if glyph_chars.is_empty() {
                return None;
            }
            let color_mode = match (colors.as_ref(), color_gradient.as_ref()) {
                (Some(colors), None) => {
                    if colors.len() != glyph_chars.len() {
                        return None;
                    }
                    discrete_color_mode(colors.iter().map(|c| Color::from(*c)).collect())
                }
                (None, Some(gradient)) => gradient_color_mode(gradient.clone()),
                _ => return None,
            };
            let apply_to_mode = match apply_to {
                crate::types::cls_filter_spec::AnimatedGlyphRampApplyTo::Foreground => {
                    ImplApplyTo::Foreground
                }
                crate::types::cls_filter_spec::AnimatedGlyphRampApplyTo::Background => {
                    ImplApplyTo::Background
                }
                crate::types::cls_filter_spec::AnimatedGlyphRampApplyTo::Both => ImplApplyTo::Both,
            };
            let affect_mode = match affect {
                crate::types::cls_filter_spec::AnimatedGlyphRampAffect::All => AffectMode::All,
                crate::types::cls_filter_spec::AnimatedGlyphRampAffect::NonEmpty => {
                    AffectMode::NonEmpty
                }
            };
            Some(PreparedFilter::AnimatedGlyphRamp(
                AnimatedGlyphRampFilter::new(
                    glyph_chars,
                    color_mode,
                    *cycles_per_second,
                    *ease,
                    apply_to_mode,
                    affect_mode,
                    *phase_offset_x_ms,
                    *phase_offset_y_ms,
                ),
            ))
        }
        FilterSpec::GlyphTimeline {
            frames,
            trigger,
            on_complete,
            apply_to,
            affect,
        } => {
            use crate::filters::cls_charset_noise::AffectMode;
            use crate::filters::cls_glyph_timeline::{
                Frame, GlyphTimelineApplyTo as ImplApplyTo, JitterConfig as ImplJitter,
                TimelineCompletion as ImplCompletion, TimelineTrigger as ImplTrigger,
                WavefrontAxis as ImplAxis, WavefrontTriggerConfig as ImplWavefront,
            };
            use crate::types::cls_filter_spec::{
                GlyphTimelineApplyTo as SpecApplyTo, GlyphTimelineAffect as SpecAffect,
                GlyphTimelineCompletion as SpecCompletion,
                GlyphTimelineLaneAxis as SpecLaneAxis,
                GlyphTimelineTriggerSpec as SpecTrigger,
                GlyphTimelineWavefrontAxis as SpecAxis,
            };
            use std::sync::Arc;
            use tui_vfx_style::schedules::{
                LaneAxis, PoissonBurstScheduleConfig, poisson_burst_schedule,
            };

            if frames.is_empty() {
                // Mirror the validate-time guard at construction time.
                return None;
            }

            let prepared_frames: Vec<Frame> = frames
                .iter()
                .map(|f| {
                    use crate::filters::cls_glyph_timeline::FrameColor;
                    use crate::types::cls_filter_spec::FrameColorSpec;
                    let fg = f.fg.as_ref().map(|fc| match fc {
                        FrameColorSpec::Static(c) => FrameColor::Static(Color::from(c.clone())),
                        FrameColorSpec::Palette { palette, seed } => FrameColor::Palette {
                            colors: palette.iter().cloned().map(Color::from).collect(),
                            seed: *seed,
                        },
                    });
                    Frame::new_with_fg(f.glyph, fg, f.bg.clone().map(Color::from), f.duration_ticks)
                })
                .collect();

            let map_axis = |a: &SpecAxis| match a {
                SpecAxis::LeftToRight => ImplAxis::LeftToRight,
                SpecAxis::RightToLeft => ImplAxis::RightToLeft,
                SpecAxis::TopToBottom => ImplAxis::TopToBottom,
                SpecAxis::BottomToTop => ImplAxis::BottomToTop,
                SpecAxis::DiagonalTlBr => ImplAxis::DiagonalTlBr,
                SpecAxis::DiagonalTrBl => ImplAxis::DiagonalTrBl,
            };

            let prepared_trigger = match trigger {
                SpecTrigger::Immediate => ImplTrigger::Immediate,
                SpecTrigger::PhaseOffset {
                    base_offset_seconds,
                    phase_offset_x_ms,
                    phase_offset_y_ms,
                } => ImplTrigger::PhaseOffset {
                    base_offset_seconds: *base_offset_seconds,
                    phase_offset_x_ms: *phase_offset_x_ms,
                    phase_offset_y_ms: *phase_offset_y_ms,
                },
                SpecTrigger::Wavefront {
                    axis,
                    total_duration_seconds,
                    base_offset_seconds,
                    easing,
                    jitter,
                } => ImplTrigger::Wavefront(ImplWavefront {
                    axis: map_axis(axis),
                    total_duration_seconds: *total_duration_seconds,
                    base_offset_seconds: *base_offset_seconds,
                    easing: *easing,
                    jitter: jitter.as_ref().map(|j| ImplJitter {
                        seed: j.seed,
                        amount_seconds: j.amount_seconds,
                    }),
                }),
                SpecTrigger::PoissonBurst {
                    lane_axis,
                    batch_period_frames,
                    batch_size_min,
                    batch_size_max,
                    lane_speed_min,
                    lane_speed_max,
                    shuffle_seed,
                    batch_seed,
                    speed_seed,
                    fps,
                    direction_seed,
                    jitter,
                } => {
                    let cfg = PoissonBurstScheduleConfig {
                        lane_axis: match lane_axis {
                            SpecLaneAxis::Row => LaneAxis::Row,
                            SpecLaneAxis::Column => LaneAxis::Column,
                        },
                        batch_period_frames: *batch_period_frames,
                        batch_size_min: *batch_size_min,
                        batch_size_max: *batch_size_max,
                        lane_speed_min: *lane_speed_min,
                        lane_speed_max: *lane_speed_max,
                        shuffle_seed: *shuffle_seed,
                        batch_seed: *batch_seed,
                        speed_seed: *speed_seed,
                        fps: *fps,
                        direction_seed: *direction_seed,
                        jitter: jitter
                            .as_ref()
                            .map(|j| (j.seed, j.amount_seconds)),
                    };
                    let trigger_times = Arc::new(poisson_burst_schedule(
                        prepare_ctx.width,
                        prepare_ctx.height,
                        &cfg,
                    ));
                    ImplTrigger::PerCellSchedule {
                        trigger_times,
                        width: prepare_ctx.width,
                    }
                }
            };

            let prepared_completion = match on_complete {
                SpecCompletion::Hold => ImplCompletion::Hold,
                SpecCompletion::Hide => ImplCompletion::Hide,
                SpecCompletion::Loop => ImplCompletion::Loop,
            };
            let prepared_apply_to = match apply_to {
                SpecApplyTo::Foreground => ImplApplyTo::Foreground,
                SpecApplyTo::Background => ImplApplyTo::Background,
                SpecApplyTo::Both => ImplApplyTo::Both,
            };
            let prepared_affect = match affect {
                SpecAffect::All => AffectMode::All,
                SpecAffect::NonEmpty => AffectMode::NonEmpty,
            };

            Some(PreparedFilter::GlyphTimeline(GlyphTimeline::new(
                prepared_frames,
                prepared_trigger,
                prepared_completion,
                prepared_apply_to,
                prepared_affect,
            )))
        }
        FilterSpec::MatrixRain {
            mode,
            density,
            speed_multiplier,
            speed_min,
            speed_max,
            trail_min,
            trail_max,
            glyph_change_hz,
            seed,
            affect,
            preset,
            chars,
            head_color,
            tail_color,
        } => {
            let resolved_density = density
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.5);
            let resolved_speed_multiplier = speed_multiplier
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(1.0);
            let affect = match affect {
                crate::types::cls_filter_spec::MatrixRainAffect::All => MatrixRainAffectMode::All,
                crate::types::cls_filter_spec::MatrixRainAffect::OnlyBlank => {
                    MatrixRainAffectMode::OnlyBlank
                }
            };
            let preset = match preset {
                crate::types::cls_filter_spec::MatrixRainCharsetPreset::Matrix => {
                    MatrixRainGlyphPreset::Matrix
                }
                crate::types::cls_filter_spec::MatrixRainCharsetPreset::Binary => {
                    MatrixRainGlyphPreset::Binary
                }
                crate::types::cls_filter_spec::MatrixRainCharsetPreset::Hex => {
                    MatrixRainGlyphPreset::Hex
                }
                crate::types::cls_filter_spec::MatrixRainCharsetPreset::Ascii => {
                    MatrixRainGlyphPreset::Ascii
                }
            };
            let mode = match mode {
                crate::types::cls_filter_spec::MatrixRainMode::Modern => MatrixRainMode::Modern,
                crate::types::cls_filter_spec::MatrixRainMode::Classic => MatrixRainMode::Classic,
            };

            let mut filter = MatrixRain::new()
                .with_mode(mode)
                .with_density(resolved_density)
                .with_speed_multiplier(resolved_speed_multiplier)
                .with_speed_range(*speed_min, *speed_max)
                .with_trail_range(*trail_min, *trail_max)
                .with_glyph_change_hz(*glyph_change_hz)
                .with_seed(*seed)
                .with_affect(affect)
                .with_preset(preset)
                .with_head_color((*head_color).into())
                .with_tail_color((*tail_color).into());
            if let Some(chars) = chars {
                filter = filter.with_custom_glyphs(chars.clone());
            }
            Some(PreparedFilter::MatrixRain(filter))
        }
        FilterSpec::InterlaceCurtain {
            density,
            dim_factor,
            scroll_speed,
        } => {
            let filter = InterlaceCurtain::new()
                .with_density(*density)
                .with_dim_factor(*dim_factor)
                .with_scroll_speed(*scroll_speed);
            Some(PreparedFilter::InterlaceCurtain(filter))
        }
        FilterSpec::MotionBlur {
            trail_length,
            opacity_decay,
            direction,
        } => {
            let motion_dir = match direction {
                crate::types::cls_filter_spec::MotionBlurDirection::Left => MotionDirection::Left,
                crate::types::cls_filter_spec::MotionBlurDirection::Right => MotionDirection::Right,
                crate::types::cls_filter_spec::MotionBlurDirection::Up => MotionDirection::Up,
                crate::types::cls_filter_spec::MotionBlurDirection::Down => MotionDirection::Down,
            };
            let filter = MotionBlur::new(*trail_length, *opacity_decay, motion_dir);
            Some(PreparedFilter::MotionBlur(filter))
        }
        FilterSpec::ColorBridgedShade {
            opacity,
            fg_color,
            bg_color,
        } => {
            let fg: Color = (*fg_color).into();
            let bg: Color = (*bg_color).into();
            let filter = ColorBridgedShade::new(*opacity, fg, bg);
            Some(PreparedFilter::ColorBridgedShade(filter))
        }
        FilterSpec::SubPixelBar {
            progress,
            direction,
            filled_color,
            unfilled_color,
            animated,
        } => {
            let bar_direction = match direction {
                crate::types::cls_filter_spec::SubPixelBarDirection::Horizontal => {
                    BarDirection::Horizontal
                }
                crate::types::cls_filter_spec::SubPixelBarDirection::Vertical => {
                    BarDirection::Vertical
                }
            };
            let filled: Color = (*filled_color).into();
            let unfilled: Color = (*unfilled_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = SubPixelBar::new(evaluated_progress)
                .with_direction(bar_direction)
                .with_filled_color(filled)
                .with_unfilled_color(unfilled)
                .animated(*animated);
            Some(PreparedFilter::SubPixelBar(filter))
        }
        FilterSpec::SubcellLight {
            lit_color,
            unlit_color,
            render_mode,
            sample_from,
            threshold,
            temporal_dither_hz,
            only_blank,
        } => Some(PreparedFilter::SubcellLight(SubcellLight {
            lit_color: (*lit_color).into(),
            unlit_color: (*unlit_color).into(),
            render_mode: match render_mode {
                crate::types::cls_filter_spec::SubcellLightRenderMode::Braille => {
                    ImplSubcellLightRenderMode::Braille
                }
                crate::types::cls_filter_spec::SubcellLightRenderMode::Horizontal => {
                    ImplSubcellLightRenderMode::Horizontal
                }
                crate::types::cls_filter_spec::SubcellLightRenderMode::Vertical => {
                    ImplSubcellLightRenderMode::Vertical
                }
            },
            sample_from: match sample_from {
                crate::types::cls_filter_spec::LightSampleFrom::Foreground => {
                    ImplLightSampleFrom::Foreground
                }
                crate::types::cls_filter_spec::LightSampleFrom::Background => {
                    ImplLightSampleFrom::Background
                }
            },
            threshold: *threshold,
            temporal_dither_hz: *temporal_dither_hz,
            only_blank: *only_blank,
        })),
        FilterSpec::SubCellShake {
            amplitude,
            frequency,
            seed,
            edge_only,
            filled_color,
            bg_color,
        } => {
            let filled: Color = (*filled_color).into();
            let bg: Color = (*bg_color).into();
            let filter = SubCellShake::new()
                .with_amplitude(*amplitude)
                .with_frequency(*frequency)
                .with_seed(*seed)
                .edge_only(*edge_only)
                .with_filled_color(filled)
                .with_bg_color(bg);
            Some(PreparedFilter::SubCellShake(filter))
        }
        FilterSpec::RigidShake {
            shake_period,
            num_shakes,
            num_shakes_binding,
            pause_duration,
            max_eighths,
            base_eighths,
            damping,
            damping_scale_binding,
            element_color,
            bg_color,
            inner_width,
            margin_width,
        } => {
            let element: Color = (*element_color).into();
            let bg: Color = (*bg_color).into();
            // Convert Vec to [f32; 8], padding with 0.0 if needed
            let mut damping_arr = [0.0_f32; 8];
            for (i, &v) in damping.iter().take(8).enumerate() {
                damping_arr[i] = v;
            }
            // Resolve damping_scale from the runtime binding when present.
            // The resolved f32 is clamped to 0.1..=10.0 and multiplied into
            // every element of the damping array, so a scale of 2.0 doubles
            // the decay rate of the whole curve and a scale of 0.5 halves
            // it. Missing bindings leave the static damping curve
            // untouched (equivalent to a scale of 1.0).
            let resolved_damping_scale = damping_scale_binding
                .as_deref()
                .and_then(|key| prepare_ctx.runtime_params.get_f32(key))
                .map(|s| s.clamp(0.1, 10.0));
            if let Some(scale) = resolved_damping_scale {
                for v in damping_arr.iter_mut() {
                    *v *= scale;
                }
            }
            // Resolve num_shakes from the runtime binding when present.
            // The runtime param map exposes get_u16; the downstream filter
            // clamps further to the 0-8 shake cap. Missing bindings fall
            // back to the static num_shakes.
            let resolved_num_shakes = num_shakes_binding
                .as_deref()
                .and_then(|key| prepare_ctx.runtime_params.get_u16(key))
                .map(|n| n.min(u8::MAX as u16) as u8)
                .unwrap_or(*num_shakes);
            let filter = RigidShake::new()
                .with_shake_period(*shake_period)
                .with_num_shakes(resolved_num_shakes)
                .with_pause_duration(*pause_duration)
                .with_max_eighths(*max_eighths)
                .with_base_eighths(*base_eighths)
                .with_damping(damping_arr)
                .with_element_color(element)
                .with_bg_color(bg)
                .with_inner_width(*inner_width)
                .with_margin_width(*margin_width);
            Some(PreparedFilter::RigidShake(filter))
        }
        FilterSpec::HoverBar {
            base_eighths,
            max_eighths,
            position,
            bar_color,
            bg_color,
            progress,
            margin_width,
        } => {
            let bar: Color = (*bar_color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = HoverBar::new()
                .with_base_eighths(*base_eighths)
                .with_max_eighths(*max_eighths)
                .with_position(*position)
                .with_bar_color(bar)
                .with_bg_color(bg)
                .with_progress(evaluated_progress)
                .with_margin_width(*margin_width);
            Some(PreparedFilter::HoverBar(filter))
        }
        FilterSpec::UnderlineWipe {
            direction,
            color,
            bg_color,
            line_char,
            row_offset,
            progress,
            gradient,
            glisten,
        } => {
            let line_color: Color = (*color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = UnderlineWipe::new()
                .with_direction(*direction)
                .with_color(line_color)
                .with_bg_color(bg)
                .with_line_char(*line_char)
                .with_row_offset(*row_offset)
                .with_progress(evaluated_progress)
                .with_gradient(*gradient)
                .with_glisten(*glisten);
            Some(PreparedFilter::UnderlineWipe(filter))
        }
        FilterSpec::BracketEmphasis {
            left,
            right,
            color,
            bg_color,
            progress,
        } => {
            let bracket_color: Color = (*color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = BracketEmphasis::new()
                .with_left(*left)
                .with_right(*right)
                .with_color(bracket_color)
                .with_bg_color(bg)
                .with_progress(evaluated_progress);
            Some(PreparedFilter::BracketEmphasis(filter))
        }
        FilterSpec::DotIndicator {
            indicator_char,
            position,
            color,
            bg_color,
            progress,
        } => {
            let dot_color: Color = (*color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = DotIndicator::new()
                .with_char(*indicator_char)
                .with_position(*position)
                .with_color(dot_color)
                .with_bg_color(bg)
                .with_progress(evaluated_progress);
            Some(PreparedFilter::DotIndicator(filter))
        }
        FilterSpec::EdgeGrow {
            rest_eighths,
            peak_eighths,
            edge,
            fill_color,
            bg_color,
            progress,
            margin_width,
        } => {
            let fill: Color = (*fill_color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            Some(PreparedFilter::EdgeGrow(EdgeGrow {
                rest_eighths: *rest_eighths,
                peak_eighths: *peak_eighths,
                edge: *edge,
                fill_color: fill,
                bg_color: bg,
                progress: evaluated_progress,
                margin_width: *margin_width,
            }))
        }
        FilterSpec::PillButton {
            button_color,
            bg_color,
            edge_width,
            glisten,
            progress,
        } => {
            let btn_color: Color = (*button_color).into();
            let bg: Color = (*bg_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = PillButton::new()
                .with_button_color(btn_color)
                .with_bg_color(bg)
                .with_edge_width(*edge_width)
                .with_glisten(*glisten)
                .with_progress(evaluated_progress);
            Some(PreparedFilter::PillButton(filter))
        }
        FilterSpec::GlistenSweep {
            boost,
            band_width,
            speed,
            progress,
            powerline_mode,
            boost_separator_bg,
        } => {
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = GlistenSweep::new()
                .with_boost(*boost)
                .with_band_width(*band_width)
                .with_speed(*speed)
                .with_progress(evaluated_progress)
                .with_powerline_mode(*powerline_mode)
                .with_boost_separator_bg(*boost_separator_bg);
            Some(PreparedFilter::GlistenSweep(filter))
        }
        FilterSpec::KittScanner {
            boost,
            band_width,
            bpm,
            bps,
            progress,
            motion_mode,
            axis,
            apply_to,
            powerline_mode,
            boost_separator_bg,
        } => {
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = {
                let base = KittScanner::new()
                    .with_boost(*boost)
                    .with_band_width(*band_width)
                    .with_progress(evaluated_progress)
                    .with_motion_mode(*motion_mode)
                    .with_axis(*axis)
                    .with_apply_to(*apply_to)
                    .with_powerline_mode(*powerline_mode)
                    .with_boost_separator_bg(*boost_separator_bg);
                if let Some(bpm) = bpm {
                    base.with_bpm(*bpm)
                } else {
                    base.with_bps(*bps)
                }
            };
            Some(PreparedFilter::KittScanner(filter))
        }
        FilterSpec::ShadeScanner {
            shade_color,
            bps,
            progress,
        } => {
            let shade: Color = (*shade_color).into();
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = ShadeScanner::new()
                .with_shade_color(shade)
                .with_bps(*bps)
                .with_progress(evaluated_progress);
            Some(PreparedFilter::ShadeScanner(filter))
        }
        FilterSpec::GlyphStyle { rules } => {
            // Resolve ColorConfig → Color per rule at prepare time; the
            // inner GlyphStyleRule (filter impl) holds concrete colors
            // while the spec-side rule holds unresolved ColorConfig.
            let resolved: Vec<GlyphStyleRuleImpl> = rules
                .iter()
                .map(|rule| GlyphStyleRuleImpl {
                    chars: rule.chars.chars().collect(),
                    fg: rule.fg.map(|c| c.into()),
                    bg: rule.bg.map(|c| c.into()),
                    bg_alternate: rule.bg_alternate.map(|c| c.into()),
                    fg_alternate: rule.fg_alternate.map(|c| c.into()),
                })
                .collect();
            Some(PreparedFilter::GlyphStyle(GlyphStyle::new(resolved)))
        }
        FilterSpec::ScalarFieldGlyph {
            sampler,
            encoder,
            threshold,
            only_blank,
            recolor,
        } => {
            // Convert spec-shape GlyphEncoderSpec → runtime GlyphEncoder.
            let runtime_encoder = match encoder {
                GlyphEncoderSpec::BrailleSubcell {
                    threshold: enc_thresh,
                } => GlyphEncoder::BrailleSubcell {
                    threshold: *enc_thresh,
                },
                GlyphEncoderSpec::BrailleEighths { rotated } => {
                    GlyphEncoder::BrailleEighths { rotated: *rotated }
                }
                GlyphEncoderSpec::BlockHorizontal => GlyphEncoder::BlockHorizontal,
                GlyphEncoderSpec::BlockVertical => GlyphEncoder::BlockVertical,
                GlyphEncoderSpec::Ramp { chars } => GlyphEncoder::Ramp(chars.clone().into()),
            };
            // Convert optional GlyphRecolorSpec → runtime Color pair.
            let runtime_recolor = recolor.as_ref().map(|GlyphRecolorSpec { lit, unlit }| {
                let lit_color: Color = (*lit).into();
                let unlit_color: Color = (*unlit).into();
                (lit_color, unlit_color)
            });
            // Build the signal sampler from the SamplerRef variant.
            // Option A: each SamplerRef variant carries its own parameters;
            // no cross-step reference needed.
            match sampler {
                SamplerRef::TerminalWater { shader } => {
                    let signal = WaterFieldSignal::new(shader.clone());
                    let filter = ScalarFieldGlyphFilter {
                        sampler: signal,
                        encoder: runtime_encoder,
                        recolor: runtime_recolor,
                        threshold: *threshold,
                        only_blank: *only_blank,
                        frame: 0,
                        seed: 0,
                    };
                    Some(PreparedFilter::ScalarFieldGlyphWater(filter))
                }
                SamplerRef::TerminalFire { shader } => {
                    let signal = FireFieldSignal::new(shader.clone());
                    let filter = ScalarFieldGlyphFilter {
                        sampler: signal,
                        encoder: runtime_encoder,
                        recolor: runtime_recolor,
                        threshold: *threshold,
                        only_blank: *only_blank,
                        frame: 0,
                        seed: 0,
                    };
                    Some(PreparedFilter::ScalarFieldGlyphFire(filter))
                }
            }
        }
    }
}

pub(crate) fn prepare_filters(
    filters: &[FilterSpec],
    prepare_ctx: &PrepareContext,
) -> SmallVec<[PreparedFilter; 3]> {
    let mut prepared = SmallVec::new();
    for filter in filters {
        if let Some(prepared_filter) = prepare_filter(filter, prepare_ctx) {
            prepared.push(prepared_filter);
        }
    }
    prepared
}

#[cfg(test)]
mod tests {
    //! Inline tests live here (rather than in `tests/pipeline/`) because
    //! `prepare_filter` is `pub(crate)` — the lowest-friction place to
    //! exercise the spec → PreparedFilter conversion directly is inside
    //! the crate.

    use super::*;
    use crate::types::BindableValue;
    use crate::types::cls_filter_spec::{ApplyTo, ScannerAxis, ScannerMotionMode};
    use serde_json::json;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn kitt_spec_with_progress(progress: BindableValue) -> FilterSpec {
        FilterSpec::KittScanner {
            boost: 50,
            band_width: 0.15,
            bpm: None,
            bps: 1.2,
            progress,
            motion_mode: ScannerMotionMode::default(),
            axis: ScannerAxis::default(),
            apply_to: ApplyTo::Both,
            powerline_mode: false,
            boost_separator_bg: false,
        }
    }

    #[test]
    fn kitt_scanner_progress_binding_resolves_from_runtime_params() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("demo_progress", 0.7_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);

        let spec = kitt_spec_with_progress(BindableValue::Binding("demo_progress".into()));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.7),
            other => panic!(
                "expected PreparedFilter::KittScanner, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn kitt_scanner_progress_binding_missing_param_falls_back_to_zero() {
        // Empty runtime_params → the Binding resolves to None → unwrap_or(0.0).
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);

        let spec = kitt_spec_with_progress(BindableValue::Binding("missing".into()));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.0),
            other => panic!(
                "expected PreparedFilter::KittScanner, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn kitt_scanner_progress_static_literal_still_works() {
        // Regression guard: a pre-lift-style literal still passes through cleanly.
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);

        let spec = kitt_spec_with_progress(BindableValue::static_f32(0.5));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.5),
            other => panic!(
                "expected PreparedFilter::KittScanner, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn kitt_scanner_progress_binding_updates_between_frames() {
        // Two different PrepareContexts with different runtime_params values
        // should produce two different prepared filters. Proves the binding
        // re-resolves per frame rather than capturing a snapshot.
        let spec = kitt_spec_with_progress(BindableValue::Binding("demo_progress".into()));

        let mut rp_a = ShaderRuntimeParams::new();
        rp_a.insert("demo_progress", 0.25_f32);
        let ctx_a = PrepareContext::new(0.0, &rp_a, 80, 24);

        let mut rp_b = ShaderRuntimeParams::new();
        rp_b.insert("demo_progress", 0.9_f32);
        let ctx_b = PrepareContext::new(0.016, &rp_b, 80, 24);

        let prepared_a = prepare_filter(&spec, &ctx_a).unwrap();
        let prepared_b = prepare_filter(&spec, &ctx_b).unwrap();

        let (PreparedFilter::KittScanner(fa), PreparedFilter::KittScanner(fb)) =
            (&prepared_a, &prepared_b)
        else {
            panic!("expected PreparedFilter::KittScanner from both frames");
        };
        assert_eq!(fa.progress, 0.25);
        assert_eq!(fb.progress, 0.9);
    }

    #[test]
    fn kitt_scanner_bpm_overrides_bps_when_present() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);

        let spec = FilterSpec::KittScanner {
            boost: 50,
            band_width: 0.15,
            bpm: Some(84.0),
            bps: 9.9,
            progress: BindableValue::static_f32(1.0),
            motion_mode: ScannerMotionMode::default(),
            axis: ScannerAxis::default(),
            apply_to: ApplyTo::Both,
            powerline_mode: false,
            boost_separator_bg: false,
        };
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert!((filter.bps - 1.4).abs() < 0.001),
            other => panic!(
                "expected PreparedFilter::KittScanner, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    // --- Binding coverage for the remaining 8 lifted filters ----------------
    //
    // One test per filter that drives its progress field via a named runtime
    // parameter and asserts the resolved value lands on the prepared filter.
    // The per-filter regression (literal still works) is covered transitively
    // by the existing FilterSpec serde roundtrip tests plus the green-gate
    // pipeline-validator runs against the existing recipe corpus.

    fn bind_ctx(key: &'static str, value: f32) -> ShaderRuntimeParams {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert(key, value);
        rp
    }

    #[test]
    fn v3_payload_prepare_smoke_covers_all_filter_variants() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.25, &rp, 80, 24);
        let cases = [
            ("none", json!({ "type": "none" }), None),
            (
                "dim",
                json!({ "type": "dim", "factor": 0.25, "apply_to": "both" }),
                Some("Dim"),
            ),
            (
                "invert",
                json!({ "type": "invert", "apply_to": "foreground" }),
                Some("Invert"),
            ),
            (
                "tint",
                json!({
                    "type": "tint",
                    "color": { "type": "rgb", "r": 255, "g": 32, "b": 32 },
                    "strength": 0.3,
                    "apply_to": "background"
                }),
                Some("Tint"),
            ),
            (
                "fade_to_canvas",
                json!({
                    "type": "fade_to_canvas",
                    "canvas_color": { "type": "rgb", "r": 8, "g": 12, "b": 20 },
                    "strength": 1.0,
                    "apply_to": "both"
                }),
                Some("FadeToCanvas"),
            ),
            ("vignette", json!({ "type": "vignette" }), Some("Vignette")),
            ("crt", json!({ "type": "crt" }), Some("Crt")),
            (
                "pattern_fill",
                json!({
                    "type": "pattern_fill",
                    "pattern": { "type": "single", "char": "." }
                }),
                Some("PatternFill"),
            ),
            (
                "greyscale",
                json!({ "type": "greyscale", "apply_to": "both" }),
                Some("Greyscale"),
            ),
            (
                "braille_dust",
                json!({ "type": "braille_dust" }),
                Some("BrailleDust"),
            ),
            (
                "charset_noise",
                json!({ "type": "charset_noise" }),
                Some("CharsetNoise"),
            ),
            (
                "animated_glyph_ramp",
                json!({
                    "type": "animated_glyph_ramp",
                    "glyphs": "AB",
                    "colors": [
                        { "type": "rgb", "r": 255, "g": 0, "b": 0 },
                        { "type": "rgb", "r": 0, "g": 0, "b": 255 }
                    ],
                    "cycles_per_second": 1.0,
                    "ease": "Linear"
                }),
                Some("AnimatedGlyphRamp"),
            ),
            (
                "matrix_rain",
                json!({ "type": "matrix_rain" }),
                Some("MatrixRain"),
            ),
            (
                "interlace_curtain",
                json!({ "type": "interlace_curtain" }),
                Some("InterlaceCurtain"),
            ),
            (
                "motion_blur",
                json!({ "type": "motion_blur" }),
                Some("MotionBlur"),
            ),
            (
                "color_bridged_shade",
                json!({ "type": "color_bridged_shade" }),
                Some("ColorBridgedShade"),
            ),
            (
                "sub_pixel_bar",
                json!({ "type": "sub_pixel_bar" }),
                Some("SubPixelBar"),
            ),
            (
                "subcell_light",
                json!({ "type": "subcell_light" }),
                Some("SubcellLight"),
            ),
            (
                "sub_cell_shake",
                json!({ "type": "sub_cell_shake" }),
                Some("SubCellShake"),
            ),
            (
                "rigid_shake",
                json!({ "type": "rigid_shake" }),
                Some("RigidShake"),
            ),
            (
                "hover_bar",
                json!({ "type": "hover_bar" }),
                Some("HoverBar"),
            ),
            (
                "underline_wipe",
                json!({ "type": "underline_wipe" }),
                Some("UnderlineWipe"),
            ),
            (
                "bracket_emphasis",
                json!({ "type": "bracket_emphasis" }),
                Some("BracketEmphasis"),
            ),
            (
                "dot_indicator",
                json!({ "type": "dot_indicator" }),
                Some("DotIndicator"),
            ),
            (
                "edge_grow",
                json!({ "type": "edge_grow" }),
                Some("EdgeGrow"),
            ),
            (
                "pill_button",
                json!({ "type": "pill_button" }),
                Some("PillButton"),
            ),
            (
                "glisten_sweep",
                json!({ "type": "glisten_sweep" }),
                Some("GlistenSweep"),
            ),
            (
                "kitt_scanner",
                json!({ "type": "kitt_scanner" }),
                Some("KittScanner"),
            ),
            (
                "shade_scanner",
                json!({ "type": "shade_scanner" }),
                Some("ShadeScanner"),
            ),
            (
                "glyph_style",
                json!({
                    "type": "glyph_style",
                    "rules": [{
                        "chars": "Glyph",
                        "fg": { "type": "rgb", "r": 220, "g": 240, "b": 255 }
                    }]
                }),
                Some("GlyphStyle"),
            ),
        ];

        let mut prepared_variant_count = 0usize;
        for (label, payload, expected_prepared_name) in cases {
            let spec = FilterSpec::try_from_v3_payload(payload.clone()).unwrap_or_else(|err| {
                panic!("{label} V3 payload should parse: {payload:?}: {err}")
            });
            let prepared = prepare_filter(&spec, &ctx);
            match expected_prepared_name {
                None => assert!(
                    prepared.is_none(),
                    "{label} should stay unprepared because FilterSpec::None is a no-op"
                ),
                Some(expected_name) => {
                    let prepared = prepared
                        .unwrap_or_else(|| panic!("{label} should prepare into {expected_name}"));
                    assert_eq!(
                        prepared.name(),
                        expected_name,
                        "{label} should prepare into the matching PreparedFilter arm"
                    );
                    assert_eq!(
                        spec.name(),
                        expected_name,
                        "{label} should preserve the public FilterSpec name"
                    );
                    prepared_variant_count += 1;
                }
            }
        }

        assert_eq!(
            prepared_variant_count, 29,
            "all non-None FilterSpec variants should prepare successfully"
        );
    }

    #[test]
    fn sub_pixel_bar_progress_binding_resolves() {
        use crate::types::cls_filter_spec::SubPixelBarDirection;
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.6);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::SubPixelBar {
            progress: BindableValue::Binding("p".into()),
            direction: SubPixelBarDirection::default(),
            filled_color: ColorConfig::Green,
            unfilled_color: ColorConfig::Gray,
            animated: false,
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::SubPixelBar(f) => assert_eq!(f.progress, 0.6),
            other => panic!(
                "expected SubPixelBar, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn hover_bar_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.42);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::HoverBar {
            base_eighths: 4,
            max_eighths: 12,
            position: Default::default(),
            bar_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: BindableValue::Binding("p".into()),
            margin_width: 2,
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::HoverBar(f) => assert_eq!(f.progress, 0.42),
            other => panic!(
                "expected HoverBar, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn underline_wipe_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.8);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::UnderlineWipe {
            direction: Default::default(),
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            line_char: '_',
            row_offset: 0,
            progress: BindableValue::Binding("p".into()),
            gradient: true,
            glisten: true,
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::UnderlineWipe(f) => assert_eq!(f.progress, 0.8),
            other => panic!(
                "expected UnderlineWipe, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn bracket_emphasis_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.33);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::BracketEmphasis {
            left: '[',
            right: ']',
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::BracketEmphasis(f) => assert_eq!(f.progress, 0.33),
            other => panic!(
                "expected BracketEmphasis, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn dot_indicator_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 1.0);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::DotIndicator {
            indicator_char: '*',
            position: Default::default(),
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::DotIndicator(f) => assert_eq!(f.progress, 1.0),
            other => panic!(
                "expected DotIndicator, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn pill_button_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.15);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::PillButton {
            button_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            edge_width: 3,
            glisten: true,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::PillButton(f) => assert_eq!(f.progress, 0.15),
            other => panic!(
                "expected PillButton, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn glisten_sweep_progress_binding_resolves() {
        let rp = bind_ctx("p", 0.55);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::GlistenSweep {
            boost: 40,
            band_width: 0.2,
            speed: 0.5,
            progress: BindableValue::Binding("p".into()),
            powerline_mode: false,
            boost_separator_bg: false,
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::GlistenSweep(f) => assert_eq!(f.progress, 0.55),
            other => panic!(
                "expected GlistenSweep, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn shade_scanner_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.75);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = FilterSpec::ShadeScanner {
            shade_color: ColorConfig::Gray,
            bps: 1.0,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::ShadeScanner(f) => assert_eq!(f.progress, 0.75),
            other => panic!(
                "expected ShadeScanner, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    // --- P0.5: rigid_shake num_shakes_binding -------------------------------

    fn rigid_shake_spec_with(num_shakes: u8, num_shakes_binding: Option<String>) -> FilterSpec {
        rigid_shake_spec_full(num_shakes, num_shakes_binding, None)
    }

    fn rigid_shake_spec_full(
        num_shakes: u8,
        num_shakes_binding: Option<String>,
        damping_scale_binding: Option<String>,
    ) -> FilterSpec {
        use tui_vfx_style::models::ColorConfig;
        FilterSpec::RigidShake {
            shake_period: 0.29,
            num_shakes,
            num_shakes_binding,
            pause_duration: 0.52,
            max_eighths: 12,
            base_eighths: 3,
            damping: vec![1.0, 0.7, 0.4, 0.2],
            damping_scale_binding,
            element_color: ColorConfig::Gray,
            bg_color: ColorConfig::Black,
            inner_width: 10,
            margin_width: 2,
        }
    }

    #[test]
    fn rigid_shake_num_shakes_binding_resolves_to_runtime_param() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("severity", 6_u16);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_with(1, Some("severity".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                assert_eq!(filter.num_shakes(), 6);
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_num_shakes_binding_clamps_to_filter_cap() {
        // The runtime param can report anything; RigidShake::with_num_shakes
        // clamps to 8. Binding 99 should land on the cap.
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("severity", 99_u16);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_with(1, Some("severity".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                assert_eq!(filter.num_shakes(), 8);
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_num_shakes_missing_binding_falls_back_to_static() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_with(3, Some("missing".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => assert_eq!(filter.num_shakes(), 3),
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_no_binding_uses_static_field() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_with(4, None);
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => assert_eq!(filter.num_shakes(), 4),
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    // --- O-P0.C: rigid_shake damping_scale_binding coverage ----------------

    #[test]
    fn rigid_shake_damping_scale_binding_doubles_curve_when_scale_is_two() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("severity_damping", 2.0_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, Some("severity_damping".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                // Static damping curve: [1.0, 0.7, 0.4, 0.2, 0.0, 0.0, 0.0, 0.0]
                // Scale 2.0 doubles every element.
                let expected = [2.0, 1.4, 0.8, 0.4, 0.0, 0.0, 0.0, 0.0];
                let actual = filter.damping();
                for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
                    assert!(
                        (a - e).abs() < 1e-6,
                        "damping[{}] = {} but expected {}",
                        i,
                        a,
                        e
                    );
                }
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_damping_scale_binding_halves_curve_when_scale_is_half() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("severity_damping", 0.5_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, Some("severity_damping".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                let expected = [0.5, 0.35, 0.2, 0.1, 0.0, 0.0, 0.0, 0.0];
                let actual = filter.damping();
                for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
                    assert!(
                        (a - e).abs() < 1e-6,
                        "damping[{}] = {} but expected {}",
                        i,
                        a,
                        e
                    );
                }
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_damping_scale_binding_clamps_below_lower_bound() {
        // Scale 0.01 must clamp to 0.1 so damping[0] lands on 0.1 (= 1.0 * 0.1)
        // rather than 0.01 which would stall the shake entirely.
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("scale", 0.01_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, Some("scale".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                let damping = filter.damping();
                assert!(
                    (damping[0] - 0.1).abs() < 1e-6,
                    "damping[0] = {} after clamp to 0.1; expected 0.1",
                    damping[0]
                );
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_damping_scale_binding_clamps_above_upper_bound() {
        // Scale 999.0 must clamp to 10.0 so damping[0] lands on 10.0
        // (= 1.0 * 10.0) rather than blowing out the numeric range.
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("scale", 999.0_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, Some("scale".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                let damping = filter.damping();
                assert!(
                    (damping[0] - 10.0).abs() < 1e-6,
                    "damping[0] = {} after clamp to 10.0; expected 10.0",
                    damping[0]
                );
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_damping_scale_binding_missing_falls_back_to_unscaled_curve() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, Some("missing".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                // Missing binding: the damping array passes through unchanged.
                let expected = [1.0, 0.7, 0.4, 0.2, 0.0, 0.0, 0.0, 0.0];
                let actual = filter.damping();
                for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
                    assert!(
                        (a - e).abs() < 1e-6,
                        "damping[{}] = {} but expected {}",
                        i,
                        a,
                        e
                    );
                }
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn rigid_shake_damping_scale_binding_no_binding_leaves_curve_untouched() {
        // Regression guard: when damping_scale_binding is None (the common
        // case for recipes authored before O-P0.C), the damping array must
        // be the untouched static curve — no silent scaling by 1.0 that
        // could mask a regression later.
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = rigid_shake_spec_full(4, None, None);
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                let expected = [1.0, 0.7, 0.4, 0.2, 0.0, 0.0, 0.0, 0.0];
                let actual = filter.damping();
                for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
                    assert!(
                        (a - e).abs() < 1e-6,
                        "damping[{}] = {} but expected {}",
                        i,
                        a,
                        e
                    );
                }
            }
            other => panic!(
                "expected RigidShake, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    // --- FadeToCanvas canvas_color_binding coverage (O-P0.B) ----------------

    fn fade_to_canvas_spec(
        canvas_color: tui_vfx_style::models::ColorConfig,
        canvas_color_binding: Option<String>,
    ) -> FilterSpec {
        FilterSpec::FadeToCanvas {
            canvas_color,
            canvas_color_binding,
            strength: BindableValue::static_f32(1.0),
            apply_to: ApplyTo::Both,
        }
    }

    #[test]
    fn fade_to_canvas_canvas_color_binding_resolves_runtime_rgb() {
        use tui_vfx_style::models::ColorConfig;
        use tui_vfx_style::traits::ShaderRuntimeParamValue;

        let mut rp = ShaderRuntimeParams::new();
        rp.insert(
            "terminal_bg",
            ShaderRuntimeParamValue::Rgb {
                r: 240,
                g: 241,
                b: 242,
            },
        );
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = fade_to_canvas_spec(ColorConfig::Black, Some("terminal_bg".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::FadeToCanvas(filter) => {
                assert_eq!(filter.canvas_color, Color::rgb(240, 241, 242));
            }
            other => panic!(
                "expected FadeToCanvas, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn fade_to_canvas_canvas_color_binding_missing_falls_back_to_static() {
        use tui_vfx_style::models::ColorConfig;

        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = fade_to_canvas_spec(
            ColorConfig::Rgb {
                r: 10,
                g: 20,
                b: 30,
            },
            Some("missing".to_string()),
        );
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::FadeToCanvas(filter) => {
                assert_eq!(filter.canvas_color, Color::rgb(10, 20, 30));
            }
            other => panic!(
                "expected FadeToCanvas, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn fade_to_canvas_canvas_color_binding_non_rgb_kind_falls_back_to_static() {
        use tui_vfx_style::models::ColorConfig;
        use tui_vfx_style::traits::ShaderRuntimeParamValue;

        // A runtime param that happens to exist under the binding key but
        // reports a non-Rgb kind (e.g. a stray integer) must fall through
        // to the static canvas_color, not silently corrupt the fade target.
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("terminal_bg", ShaderRuntimeParamValue::Integer(42));
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = fade_to_canvas_spec(
            ColorConfig::Rgb { r: 7, g: 8, b: 9 },
            Some("terminal_bg".to_string()),
        );
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::FadeToCanvas(filter) => {
                assert_eq!(filter.canvas_color, Color::rgb(7, 8, 9));
            }
            other => panic!(
                "expected FadeToCanvas, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn fade_to_canvas_no_binding_uses_static_canvas_color() {
        use tui_vfx_style::models::ColorConfig;

        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = fade_to_canvas_spec(
            ColorConfig::Rgb {
                r: 180,
                g: 180,
                b: 190,
            },
            None,
        );
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::FadeToCanvas(filter) => {
                assert_eq!(filter.canvas_color, Color::rgb(180, 180, 190));
            }
            other => panic!(
                "expected FadeToCanvas, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    fn matrix_rain_spec(density: BindableValue, speed_multiplier: BindableValue) -> FilterSpec {
        use tui_vfx_style::models::ColorConfig;

        FilterSpec::MatrixRain {
            mode: crate::types::cls_filter_spec::MatrixRainMode::Modern,
            density,
            speed_multiplier,
            speed_min: 5.0,
            speed_max: 15.0,
            trail_min: 8,
            trail_max: 20,
            glyph_change_hz: 8.0,
            seed: 42,
            affect: crate::types::cls_filter_spec::MatrixRainAffect::All,
            preset: crate::types::cls_filter_spec::MatrixRainCharsetPreset::Matrix,
            chars: None,
            head_color: ColorConfig::Rgb {
                r: 220,
                g: 255,
                b: 220,
            },
            tail_color: ColorConfig::Rgb { r: 0, g: 160, b: 0 },
        }
    }

    #[test]
    fn matrix_rain_density_binding_resolves_runtime_param() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("density", 0.85_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = matrix_rain_spec(
            BindableValue::Binding("density".to_owned()),
            BindableValue::static_f32(1.0),
        );
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::MatrixRain(filter) => {
                assert!((filter.density - 0.85).abs() < 0.001);
            }
            other => panic!(
                "expected MatrixRain, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }

    #[test]
    fn matrix_rain_speed_multiplier_binding_resolves_runtime_param() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("speed_multiplier", 1.75_f32);
        let ctx = PrepareContext::new(0.0, &rp, 80, 24);
        let spec = matrix_rain_spec(
            BindableValue::static_f32(0.5),
            BindableValue::Binding("speed_multiplier".to_owned()),
        );
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::MatrixRain(filter) => {
                assert!((filter.speed_multiplier - 1.75).abs() < 0.001);
            }
            other => panic!(
                "expected MatrixRain, got {:?}",
                std::mem::discriminant(&other)
            ),
        }
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 2.17.0</VERS>
