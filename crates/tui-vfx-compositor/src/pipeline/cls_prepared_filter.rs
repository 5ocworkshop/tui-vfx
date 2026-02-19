// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>VERSION: 2.13.0</VERS>
// <WCTX>Add boost_separator_bg for continuous powerline backgrounds</WCTX>
// <CLOG>Pass boost_separator_bg to KittScanner and GlistenSweep</CLOG>

use crate::filters::cls_bracket_emphasis::BracketEmphasis;
use crate::filters::cls_braille_dust::BrailleDust;
use crate::filters::cls_color_bridged_shade::ColorBridgedShade;
use crate::filters::cls_crt::Crt;
use crate::filters::cls_dim::Dim;
use crate::filters::cls_dot_indicator::DotIndicator;
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
use mixed_signals::traits::SignalContext;
use smallvec::SmallVec;
use tui_vfx_types::{Cell, Color};

pub(crate) enum PreparedFilter {
    Dim { filter: Dim, factor: f32 },
    Invert(Invert),
    Tint(Tint),
    Vignette(Vignette),
    Crt(Crt),
    PatternFill(PatternFill),
    Greyscale(Greyscale),
    BrailleDust(BrailleDust),
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
            PreparedFilter::Vignette(_) => "Vignette",
            PreparedFilter::Crt(_) => "Crt",
            PreparedFilter::PatternFill(_) => "PatternFill",
            PreparedFilter::Greyscale(_) => "Greyscale",
            PreparedFilter::BrailleDust(_) => "BrailleDust",
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
    loop_t: f64,
    signal_ctx: &SignalContext,
) -> Option<PreparedFilter> {
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
            Some(PreparedFilter::BrailleDust(filter))
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
            let filter = SubPixelBar::new(*progress)
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
            let filter = RigidShake::new()
                .with_shake_period(*shake_period)
                .with_num_shakes(*num_shakes)
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
            let filter = HoverBar::new()
                .with_base_eighths(*base_eighths)
                .with_max_eighths(*max_eighths)
                .with_position(*position)
                .with_bar_color(bar)
                .with_bg_color(bg)
                .with_progress(*progress)
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
            let filter = UnderlineWipe::new()
                .with_direction(*direction)
                .with_color(line_color)
                .with_bg_color(bg)
                .with_line_char(*line_char)
                .with_row_offset(*row_offset)
                .with_progress(*progress)
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
            let filter = BracketEmphasis::new()
                .with_left(*left)
                .with_right(*right)
                .with_color(bracket_color)
                .with_bg_color(bg)
                .with_progress(*progress);
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
            let filter = DotIndicator::new()
                .with_char(*indicator_char)
                .with_position(*position)
                .with_color(dot_color)
                .with_bg_color(bg)
                .with_progress(*progress);
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
            let filter = PillButton::new()
                .with_button_color(btn_color)
                .with_bg_color(bg)
                .with_edge_width(*edge_width)
                .with_glisten(*glisten)
                .with_progress(*progress);
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
            let filter = GlistenSweep::new()
                .with_boost(*boost)
                .with_band_width(*band_width)
                .with_speed(*speed)
                .with_progress(*progress)
                .with_powerline_mode(*powerline_mode)
                .with_boost_separator_bg(*boost_separator_bg);
            Some(PreparedFilter::GlistenSweep(filter))
        }
        FilterSpec::KittScanner {
            boost,
            band_width,
            bps,
            progress,
            apply_to,
            powerline_mode,
            boost_separator_bg,
        } => {
            let filter = KittScanner::new()
                .with_boost(*boost)
                .with_band_width(*band_width)
                .with_bps(*bps)
                .with_progress(*progress)
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
            let filter = ShadeScanner::new()
                .with_shade_color(shade)
                .with_bps(*bps)
                .with_progress(*progress);
            Some(PreparedFilter::ShadeScanner(filter))
        }
    }
}

pub(crate) fn prepare_filters(
    filters: &[FilterSpec],
    loop_t: f64,
) -> SmallVec<[PreparedFilter; 3]> {
    let signal_ctx = SignalContext::for_loop(loop_t, 0);
    let mut prepared = SmallVec::new();
    for filter in filters {
        if let Some(prepared_filter) = prepare_filter(filter, loop_t, &signal_ctx) {
            prepared.push(prepared_filter);
        }
    }
    prepared
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_prepared_filter.rs</FILE> - <DESC>Prepared filter enum for pipeline rendering</DESC>
// <VERS>END OF VERSION: 2.13.0</VERS>
