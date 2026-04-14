// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>VERSION: 2.14.0</VERS>
// <WCTX>Phase 0 P0.1 — thread PrepareContext (loop_t, signal_ctx, runtime_params) through prepare_filter</WCTX>
// <CLOG>Replace loose (loop_t, signal_ctx) params with a single &PrepareContext on prepare_filter and prepare_filters</CLOG>

use super::cls_prepare_context::PrepareContext;
use crate::filters::cls_bracket_emphasis::BracketEmphasis;
use crate::filters::cls_braille_dust::BrailleDust;
use crate::filters::cls_charset_noise::CharsetNoise;
use crate::filters::cls_color_bridged_shade::ColorBridgedShade;
use crate::filters::cls_crt::Crt;
use crate::filters::cls_dim::Dim;
use crate::filters::cls_dot_indicator::DotIndicator;
use crate::filters::cls_fade_to_canvas::FadeToCanvas;
use crate::filters::cls_glisten_sweep::GlistenSweep;
use crate::filters::cls_greyscale::Greyscale;
use crate::filters::cls_hover_bar::HoverBar;
use crate::filters::cls_interlace_curtain::InterlaceCurtain;
use crate::filters::cls_invert::Invert;
use crate::filters::cls_kitt_scanner::KittScanner;
use crate::filters::cls_motion_blur::{MotionBlur, MotionDirection};
use crate::filters::cls_pattern_fill::PatternFill;
use crate::filters::cls_pill_button::PillButton;
use crate::filters::cls_rigid_shake::RigidShake;
use crate::filters::cls_shade_scanner::ShadeScanner;
use crate::filters::cls_sub_cell_shake::SubCellShake;
use crate::filters::cls_sub_pixel_bar::{BarDirection, SubPixelBar};
use crate::filters::cls_tint::Tint;
use crate::filters::cls_underline_wipe::UnderlineWipe;
use crate::filters::cls_vignette::Vignette;
use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::{FilterSpec, PatternType};
use smallvec::SmallVec;
use tui_vfx_types::{Cell, Color};

pub(crate) enum PreparedFilter {
    Dim { filter: Dim, factor: f32 },
    Invert(Invert),
    Tint(Tint),
    FadeToCanvas(FadeToCanvas),
    Vignette(Vignette),
    Crt(Crt),
    PatternFill(PatternFill),
    Greyscale(Greyscale),
    BrailleDust(BrailleDust),
    CharsetNoise(CharsetNoise),
    InterlaceCurtain(InterlaceCurtain),
    MotionBlur(MotionBlur),
    ColorBridgedShade(ColorBridgedShade),
    SubPixelBar(SubPixelBar),
    SubCellShake(SubCellShake),
    RigidShake(RigidShake),
    HoverBar(HoverBar),
    UnderlineWipe(UnderlineWipe),
    BracketEmphasis(BracketEmphasis),
    DotIndicator(DotIndicator),
    PillButton(PillButton),
    GlistenSweep(GlistenSweep),
    KittScanner(KittScanner),
    ShadeScanner(ShadeScanner),
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
        match self {
            PreparedFilter::Dim { filter, factor } => {
                filter.apply(cell, local_x, local_y, width, height, *factor as f64);
            }
            PreparedFilter::Invert(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::Tint(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::FadeToCanvas(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::Vignette(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::Crt(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::PatternFill(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::Greyscale(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::BrailleDust(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::CharsetNoise(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::InterlaceCurtain(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::MotionBlur(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::ColorBridgedShade(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::SubPixelBar(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::SubCellShake(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::RigidShake(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::HoverBar(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::UnderlineWipe(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::BracketEmphasis(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::DotIndicator(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::PillButton(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::GlistenSweep(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::KittScanner(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
            }
            PreparedFilter::ShadeScanner(filter) => {
                filter.apply(cell, local_x, local_y, width, height, loop_t);
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
            PreparedFilter::InterlaceCurtain(_) => "InterlaceCurtain",
            PreparedFilter::MotionBlur(_) => "MotionBlur",
            PreparedFilter::ColorBridgedShade(_) => "ColorBridgedShade",
            PreparedFilter::SubPixelBar(_) => "SubPixelBar",
            PreparedFilter::SubCellShake(_) => "SubCellShake",
            PreparedFilter::RigidShake(_) => "RigidShake",
            PreparedFilter::HoverBar(_) => "HoverBar",
            PreparedFilter::UnderlineWipe(_) => "UnderlineWipe",
            PreparedFilter::BracketEmphasis(_) => "BracketEmphasis",
            PreparedFilter::DotIndicator(_) => "DotIndicator",
            PreparedFilter::PillButton(_) => "PillButton",
            PreparedFilter::GlistenSweep(_) => "GlistenSweep",
            PreparedFilter::KittScanner(_) => "KittScanner",
            PreparedFilter::ShadeScanner(_) => "ShadeScanner",
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
            strength,
            apply_to,
        } => {
            let evaluated_strength = strength
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let canvas: Color = (*canvas_color).into();
            Some(PreparedFilter::FadeToCanvas(FadeToCanvas {
                canvas_color: canvas,
                strength: evaluated_strength,
                apply_to: *apply_to,
            }))
        }
        FilterSpec::Vignette { strength, radius } => {
            let evaluated_strength = strength.evaluate(loop_t, signal_ctx).unwrap_or(0.5);
            let evaluated_radius = radius.evaluate(loop_t, signal_ctx).unwrap_or(0.8);
            Some(PreparedFilter::Vignette(Vignette::new(
                evaluated_strength,
                evaluated_radius,
            )))
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
            bps,
            progress,
            motion_mode,
            apply_to,
            powerline_mode,
            boost_separator_bg,
        } => {
            let evaluated_progress = progress
                .evaluate(loop_t, signal_ctx, prepare_ctx.runtime_params)
                .unwrap_or(0.0);
            let filter = KittScanner::new()
                .with_boost(*boost)
                .with_band_width(*band_width)
                .with_bps(*bps)
                .with_progress(evaluated_progress)
                .with_motion_mode(*motion_mode)
                .with_apply_to(*apply_to)
                .with_powerline_mode(*powerline_mode)
                .with_boost_separator_bg(*boost_separator_bg);
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
    use crate::types::cls_filter_spec::{ApplyTo, ScannerMotionMode};
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn kitt_spec_with_progress(progress: BindableValue) -> FilterSpec {
        FilterSpec::KittScanner {
            boost: 50,
            band_width: 0.15,
            bps: 1.0,
            progress,
            motion_mode: ScannerMotionMode::default(),
            apply_to: ApplyTo::Both,
            powerline_mode: false,
            boost_separator_bg: false,
        }
    }

    #[test]
    fn kitt_scanner_progress_binding_resolves_from_runtime_params() {
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("demo_progress", 0.7_f32);
        let ctx = PrepareContext::new(0.0, &rp);

        let spec = kitt_spec_with_progress(BindableValue::Binding("demo_progress".into()));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.7),
            other => panic!("expected PreparedFilter::KittScanner, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn kitt_scanner_progress_binding_missing_param_falls_back_to_zero() {
        // Empty runtime_params → the Binding resolves to None → unwrap_or(0.0).
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp);

        let spec = kitt_spec_with_progress(BindableValue::Binding("missing".into()));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.0),
            other => panic!("expected PreparedFilter::KittScanner, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn kitt_scanner_progress_static_literal_still_works() {
        // Regression guard: a pre-lift-style literal still passes through cleanly.
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp);

        let spec = kitt_spec_with_progress(BindableValue::static_f32(0.5));
        let prepared = prepare_filter(&spec, &ctx).expect("KittScanner prepares");

        match prepared {
            PreparedFilter::KittScanner(filter) => assert_eq!(filter.progress, 0.5),
            other => panic!("expected PreparedFilter::KittScanner, got {:?}", std::mem::discriminant(&other)),
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
        let ctx_a = PrepareContext::new(0.0, &rp_a);

        let mut rp_b = ShaderRuntimeParams::new();
        rp_b.insert("demo_progress", 0.9_f32);
        let ctx_b = PrepareContext::new(0.016, &rp_b);

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
    fn sub_pixel_bar_progress_binding_resolves() {
        use crate::types::cls_filter_spec::SubPixelBarDirection;
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.6);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = FilterSpec::SubPixelBar {
            progress: BindableValue::Binding("p".into()),
            direction: SubPixelBarDirection::default(),
            filled_color: ColorConfig::Green,
            unfilled_color: ColorConfig::Gray,
            animated: false,
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::SubPixelBar(f) => assert_eq!(f.progress, 0.6),
            other => panic!("expected SubPixelBar, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn hover_bar_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.42);
        let ctx = PrepareContext::new(0.0, &rp);
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
            other => panic!("expected HoverBar, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn underline_wipe_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.8);
        let ctx = PrepareContext::new(0.0, &rp);
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
            other => panic!("expected UnderlineWipe, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn bracket_emphasis_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.33);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = FilterSpec::BracketEmphasis {
            left: '[',
            right: ']',
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::BracketEmphasis(f) => assert_eq!(f.progress, 0.33),
            other => panic!("expected BracketEmphasis, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn dot_indicator_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 1.0);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = FilterSpec::DotIndicator {
            indicator_char: '*',
            position: Default::default(),
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::DotIndicator(f) => assert_eq!(f.progress, 1.0),
            other => panic!("expected DotIndicator, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn pill_button_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.15);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = FilterSpec::PillButton {
            button_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            edge_width: 3,
            glisten: true,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::PillButton(f) => assert_eq!(f.progress, 0.15),
            other => panic!("expected PillButton, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn glisten_sweep_progress_binding_resolves() {
        let rp = bind_ctx("p", 0.55);
        let ctx = PrepareContext::new(0.0, &rp);
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
            other => panic!("expected GlistenSweep, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn shade_scanner_progress_binding_resolves() {
        use tui_vfx_style::models::ColorConfig;

        let rp = bind_ctx("p", 0.75);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = FilterSpec::ShadeScanner {
            shade_color: ColorConfig::Gray,
            bps: 1.0,
            progress: BindableValue::Binding("p".into()),
        };
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::ShadeScanner(f) => assert_eq!(f.progress, 0.75),
            other => panic!("expected ShadeScanner, got {:?}", std::mem::discriminant(&other)),
        }
    }

    // --- P0.5: rigid_shake num_shakes_binding -------------------------------

    fn rigid_shake_spec_with(
        num_shakes: u8,
        num_shakes_binding: Option<String>,
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
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = rigid_shake_spec_with(1, Some("severity".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                assert_eq!(filter.num_shakes(), 6);
            }
            other => panic!("expected RigidShake, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn rigid_shake_num_shakes_binding_clamps_to_filter_cap() {
        // The runtime param can report anything; RigidShake::with_num_shakes
        // clamps to 8. Binding 99 should land on the cap.
        let mut rp = ShaderRuntimeParams::new();
        rp.insert("severity", 99_u16);
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = rigid_shake_spec_with(1, Some("severity".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => {
                assert_eq!(filter.num_shakes(), 8);
            }
            other => panic!("expected RigidShake, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn rigid_shake_num_shakes_missing_binding_falls_back_to_static() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = rigid_shake_spec_with(3, Some("missing".to_string()));
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => assert_eq!(filter.num_shakes(), 3),
            other => panic!("expected RigidShake, got {:?}", std::mem::discriminant(&other)),
        }
    }

    #[test]
    fn rigid_shake_no_binding_uses_static_field() {
        let rp = ShaderRuntimeParams::new();
        let ctx = PrepareContext::new(0.0, &rp);
        let spec = rigid_shake_spec_with(4, None);
        match prepare_filter(&spec, &ctx).unwrap() {
            PreparedFilter::RigidShake(filter) => assert_eq!(filter.num_shakes(), 4),
            other => panic!("expected RigidShake, got {:?}", std::mem::discriminant(&other)),
        }
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 2.14.0</VERS>
