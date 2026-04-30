// <FILE>tui-vfx-compositor/src/filters/cls_animated_glyph_ramp.rs</FILE> - <DESC>Synchronised glyph + colour cycling filter driven by one shared phase signal</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>1.0.1: migrate apply signature to &VfxCellContext.</CLOG>

//! Synchronised glyph + colour cycling driven by one shared phase signal.
//!
//! Closes the audit-1.6 synthesis gap: in tui-vfx today, [`CharsetNoise`]
//! cycles glyphs and [`Tint`] / signal-driven shaders cycle colours, but
//! composing them produces independent evolution. The TTE Waves effect
//! (and Sweep, and SynthGrid dissolve) all assume the glyph at progress
//! `p` and the colour at progress `p` are sampled from the same `p` —
//! e.g. the glyph `█` should appear precisely when the gradient reaches
//! its peak colour, not at a random other moment.
//!
//! This filter samples one shared phase signal `phase(x, y, t)` and uses
//! it for both the glyph index and the colour lookup, so the two stay in
//! lockstep regardless of `cycles_per_second`, per-cell stagger, or
//! gradient shape.
//!
//! # Phase model
//!
//! The per-cell phase in seconds is:
//!
//! ```text
//! phase_s = t + x * phase_offset_x_ms / 1000.0 + y * phase_offset_y_ms / 1000.0
//! ```
//!
//! `t` is the time argument the compositor passes to [`Filter::apply`];
//! the linear `phase_offset_*` terms shift each cell so adjacent cells
//! land at slightly different points in the ramp. Set both offsets to
//! `0.0` and every affected cell cycles in lockstep — useful for
//! shader-style "everything pulses together" effects. Set
//! `phase_offset_x_ms = 20.0` to make the wave travel left-to-right at
//! 20 ms per column, etc.
//!
//! Wrapped to `[0, 1)` and multiplied by `cycles_per_second`, that
//! becomes the normalised cycle progress; multiplying by `glyphs.len()`
//! gives the integer glyph index, and the same fraction drives the
//! colour lookup.
//!
//! # Colour modes
//!
//! - **Discrete** — supply a `Vec<Color>` whose length **must** equal
//!   `glyphs.len()`. Glyph `i` always uses colour `i`. This is the
//!   exact representation a faithful TTE port wants (TTE Waves
//!   hard-codes a 5-stop colour cycle that pairs with its 15-glyph
//!   ramp).
//! - **Gradient** — supply a [`Gradient`]; the filter samples it at
//!   `index / (glyphs.len() - 1)`. Convenient when you have a colour
//!   curve that should be evenly distributed across the glyph ramp.
//!
//! Exactly one of `colors` and `color_gradient` must be non-empty;
//! `prepare_animated_glyph_ramp` enforces this at lowering time and
//! callers that build the filter directly are expected to validate
//! before construction.
//!
//! # Affect modes
//!
//! Mirrors [`CharsetNoise`]'s `AffectMode`:
//!
//! - `All` — replace every cell, including whitespace.
//! - `NonEmpty` (default) — skip space (`' '`) and empty braille
//!   (`'\u{2800}'`). Useful when you want the ramp to "ride" existing
//!   text glyphs rather than fill the whole widget.

use crate::traits::filter::Filter;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_style::models::Gradient;
use tui_vfx_types::{Cell, Color, VfxCellContext};

use super::cls_charset_noise::AffectMode;

/// Which channel(s) the colour ramp writes into.
///
/// Mirrors the public [`tui_vfx_style::models::ApplyToColor`] enum but
/// is duplicated here so the filter has no hard dependency on a
/// shader-side type. Conversion happens at the prepared-filter
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatedGlyphRampApplyTo {
    /// Write into `cell.fg` only (default — matches the most common
    /// "glyph turns colour X" intent).
    #[default]
    Foreground,
    /// Write into `cell.bg` only.
    Background,
    /// Write into both `cell.fg` and `cell.bg`.
    Both,
}

/// Synchronised glyph + colour cycling filter.
///
/// See module-level docs for the phase model, colour modes, and affect
/// behaviour. Construction goes through [`AnimatedGlyphRamp::new`],
/// which clamps `cycles_per_second` to a small positive minimum so the
/// phase math is always well-defined.
pub struct AnimatedGlyphRamp {
    glyphs: Vec<char>,
    color_mode: ColorMode,
    cycles_per_second: f32,
    ease: EasingCurve,
    apply_to: AnimatedGlyphRampApplyTo,
    affect: AffectMode,
    phase_offset_x_ms: f32,
    phase_offset_y_ms: f32,
}

/// Colour-source representation for the ramp.
///
/// Construct via [`discrete_color_mode`] or [`gradient_color_mode`] —
/// the variants are intentionally non-public so the lowering layer
/// always passes through one of those two helpers.
pub enum ColorMode {
    /// Parallel colour vector — must match `glyphs.len()`.
    Discrete(Vec<Color>),
    /// Gradient sampled at `index / (glyphs.len() - 1)`.
    Gradient(Gradient),
}

impl AnimatedGlyphRamp {
    /// Build a new animated glyph ramp.
    ///
    /// `glyphs` must be non-empty. If `color_mode` is `Discrete`, its
    /// vector length must equal `glyphs.len()` — callers are expected
    /// to validate beforehand. `cycles_per_second` is clamped to at
    /// least `0.001` so phase math stays well-defined; supply `0.001`
    /// or smaller for "effectively static" intent.
    pub fn new(
        glyphs: Vec<char>,
        color_mode: ColorMode,
        cycles_per_second: f32,
        ease: EasingCurve,
        apply_to: AnimatedGlyphRampApplyTo,
        affect: AffectMode,
        phase_offset_x_ms: f32,
        phase_offset_y_ms: f32,
    ) -> Self {
        Self {
            glyphs,
            color_mode,
            cycles_per_second: cycles_per_second.max(0.001),
            ease,
            apply_to,
            affect,
            phase_offset_x_ms,
            phase_offset_y_ms,
        }
    }

    /// Convenience constructor for tests covering the discrete-colour
    /// path with the common defaults.
    #[cfg(test)]
    pub fn discrete(glyphs: Vec<char>, colors: Vec<Color>, cycles_per_second: f32) -> Self {
        Self::new(
            glyphs,
            ColorMode::Discrete(colors),
            cycles_per_second,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::default(),
            AffectMode::default(),
            0.0,
            0.0,
        )
    }

    /// Convenience constructor for tests covering the gradient-colour
    /// path with the common defaults.
    #[cfg(test)]
    pub fn from_gradient(glyphs: Vec<char>, gradient: Gradient, cycles_per_second: f32) -> Self {
        Self::new(
            glyphs,
            ColorMode::Gradient(gradient),
            cycles_per_second,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::default(),
            AffectMode::default(),
            0.0,
            0.0,
        )
    }

    fn should_affect(&self, cell: &Cell) -> bool {
        match self.affect {
            AffectMode::All => true,
            AffectMode::NonEmpty => !cell.ch.is_whitespace() && cell.ch != '\u{2800}',
        }
    }

    /// Compute the integer glyph index for a cell at `(x, y)` at time `t`.
    ///
    /// Public for tests and for downstream tooling that wants to read
    /// the same index the filter would write (e.g. an I/O hint that
    /// emits `current_glyph_index` for a downstream shader). The
    /// returned value is always `< glyphs.len()`.
    pub fn glyph_index(&self, x: u16, y: u16, t: f64) -> usize {
        let n = self.glyphs.len();
        if n == 0 {
            return 0;
        }
        let phase_s = t
            + (x as f64) * (self.phase_offset_x_ms as f64) / 1000.0
            + (y as f64) * (self.phase_offset_y_ms as f64) / 1000.0;
        let cycle = phase_s * self.cycles_per_second as f64;
        let mut frac = cycle - cycle.floor();
        if frac < 0.0 {
            frac += 1.0;
        }
        let eased = self.ease.ease(frac).clamp(0.0, 0.999_999) as f64;
        ((eased * n as f64).floor() as usize).min(n - 1)
    }

    /// Compute the colour for a given glyph index using the configured
    /// colour mode.
    fn color_at(&self, index: usize) -> Color {
        match &self.color_mode {
            ColorMode::Discrete(colors) => {
                if colors.is_empty() {
                    Color::default()
                } else {
                    colors[index.min(colors.len() - 1)]
                }
            }
            ColorMode::Gradient(gradient) => {
                let n = self.glyphs.len();
                let t = if n > 1 {
                    index as f32 / (n - 1) as f32
                } else {
                    0.0
                };
                gradient.sample(t)
            }
        }
    }
}

impl Filter for AnimatedGlyphRamp {
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
        let x = ctx.local_x;
        let y = ctx.local_y;
        let t = ctx.t;
        if self.glyphs.is_empty() || !self.should_affect(cell) {
            return;
        }
        let index = self.glyph_index(x, y, t);
        cell.ch = self.glyphs[index];
        let color = self.color_at(index);
        match self.apply_to {
            AnimatedGlyphRampApplyTo::Foreground => cell.fg = color,
            AnimatedGlyphRampApplyTo::Background => cell.bg = color,
            AnimatedGlyphRampApplyTo::Both => {
                cell.fg = color;
                cell.bg = color;
            }
        }
    }
}

/// Constructor entry point for [`ColorMode::Discrete`].
///
/// Public so the prepared-filter lowering layer can build a ramp
/// without exposing the private [`ColorMode`] enum.
pub fn discrete_color_mode(colors: Vec<Color>) -> ColorMode {
    ColorMode::Discrete(colors)
}

/// Constructor entry point for [`ColorMode::Gradient`].
pub fn gradient_color_mode(gradient: Gradient) -> ColorMode {
    ColorMode::Gradient(gradient)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_style::models::ColorSpace;

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgb(r, g, b)
    }

    fn make_cell(ch: char) -> Cell {
        Cell {
            ch,
            ..Cell::default()
        }
    }

    fn waves_glyphs() -> Vec<char> {
        "▁▂▃▄▅▆▇█▇▆▅▄▃▂▁".chars().collect()
    }

    fn three_color_discrete() -> Vec<Color> {
        // Length 15 to match the waves glyph ramp.
        let red = rgb(255, 0, 0);
        let green = rgb(0, 255, 0);
        let blue = rgb(0, 0, 255);
        vec![
            red, red, red, red, red, green, green, green, green, green, blue, blue, blue, blue,
            blue,
        ]
    }

    #[test]
    fn glyph_index_matches_wrapped_phase() {
        let ramp = AnimatedGlyphRamp::discrete(waves_glyphs(), three_color_discrete(), 1.0);
        // At t=0, phase=0, index=0
        assert_eq!(ramp.glyph_index(0, 0, 0.0), 0);
        // At t=0.5s with cycles_per_second=1.0, phase=0.5 -> index 7 (15*0.5 = 7.5 -> floor 7)
        assert_eq!(ramp.glyph_index(0, 0, 0.5), 7);
        // At t=1.0s phase wraps to 0.0
        assert_eq!(ramp.glyph_index(0, 0, 1.0), 0);
    }

    #[test]
    fn glyph_and_color_index_share_phase() {
        let ramp = AnimatedGlyphRamp::discrete(waves_glyphs(), three_color_discrete(), 1.0);
        // For every cell across many time samples, the colour at
        // glyph_index(x,y,t) must equal what apply() writes into fg.
        let mut cell = make_cell('⣿');
        for t_ms in (0..1000).step_by(50) {
            let t = t_ms as f64 / 1000.0;
            cell.ch = '⣿';
            cell.fg = Color::default();
            ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, t));
            let idx = ramp.glyph_index(0, 0, t);
            assert_eq!(cell.ch, waves_glyphs()[idx]);
            assert_eq!(cell.fg, three_color_discrete()[idx]);
        }
    }

    #[test]
    fn phase_offset_x_creates_wave_travel() {
        // 20 ms per column at 1 Hz means at t=0.02s, column 1 has the
        // same phase as column 0 had at t=0.0.
        let ramp = AnimatedGlyphRamp::new(
            waves_glyphs(),
            ColorMode::Discrete(three_color_discrete()),
            1.0,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::Foreground,
            AffectMode::All,
            20.0,
            0.0,
        );
        let idx_col0_t0 = ramp.glyph_index(0, 5, 0.0);
        let idx_col1_t_minus_20 = ramp.glyph_index(1, 5, -0.020);
        assert_eq!(idx_col0_t0, idx_col1_t_minus_20);
    }

    #[test]
    fn phase_offset_y_creates_vertical_wave() {
        let ramp = AnimatedGlyphRamp::new(
            waves_glyphs(),
            ColorMode::Discrete(three_color_discrete()),
            1.0,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::Foreground,
            AffectMode::All,
            0.0,
            20.0,
        );
        let idx_row0 = ramp.glyph_index(5, 0, 0.0);
        let idx_row1 = ramp.glyph_index(5, 1, -0.020);
        assert_eq!(idx_row0, idx_row1);
    }

    #[test]
    fn discrete_colors_pair_with_glyphs_directly() {
        let glyphs = vec!['A', 'B', 'C'];
        let colors = vec![rgb(255, 0, 0), rgb(0, 255, 0), rgb(0, 0, 255)];
        let ramp = AnimatedGlyphRamp::discrete(glyphs.clone(), colors.clone(), 3.0);
        // At t=0: index 0
        let mut cell = make_cell('⣿');
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.fg, rgb(255, 0, 0));
        // At t=1/9 (cycles=3, so 1/9 * 3 = 1/3, floor(1/3 * 3) = 1): index 1
        cell.ch = '⣿';
        ramp.apply(
            &mut cell,
            &VfxCellContext::new(0, 0, 10, 10, 0, 0, 1.0 / 9.0),
        );
        assert_eq!(cell.ch, 'B');
        assert_eq!(cell.fg, rgb(0, 255, 0));
    }

    #[test]
    fn gradient_color_samples_at_index_fraction() {
        let glyphs = vec!['A', 'B', 'C'];
        let gradient = Gradient {
            stops: vec![(0.0, rgb(0, 0, 0)), (1.0, rgb(255, 255, 255))],
            space: ColorSpace::Rgb,
        };
        let ramp = AnimatedGlyphRamp::from_gradient(glyphs.clone(), gradient, 1.0);
        // At t=0: index 0 -> sample at 0.0 -> black
        let mut cell = make_cell('⣿');
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.fg, rgb(0, 0, 0));
        // At index 2 of 3 (last): sample at 1.0 -> white. Find a t
        // that lands at index 2.
        // 3 glyphs, cycles=1.0. Index 2 happens at frac in [2/3, 1.0).
        // Pick t=0.7 -> frac=0.7, idx=floor(0.7*3)=2
        cell.ch = '⣿';
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.7));
        assert_eq!(cell.ch, 'C');
        assert_eq!(cell.fg, rgb(255, 255, 255));
    }

    #[test]
    fn apply_to_background_writes_bg_only() {
        let ramp = AnimatedGlyphRamp::new(
            vec!['X'],
            ColorMode::Discrete(vec![rgb(99, 88, 77)]),
            1.0,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::Background,
            AffectMode::All,
            0.0,
            0.0,
        );
        let mut cell = make_cell('⣿');
        cell.fg = rgb(1, 2, 3);
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell.ch, 'X');
        assert_eq!(cell.fg, rgb(1, 2, 3), "fg must be untouched");
        assert_eq!(cell.bg, rgb(99, 88, 77));
    }

    #[test]
    fn apply_to_both_writes_fg_and_bg() {
        let ramp = AnimatedGlyphRamp::new(
            vec!['X'],
            ColorMode::Discrete(vec![rgb(11, 22, 33)]),
            1.0,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::Both,
            AffectMode::All,
            0.0,
            0.0,
        );
        let mut cell = make_cell('⣿');
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(cell.fg, rgb(11, 22, 33));
        assert_eq!(cell.bg, rgb(11, 22, 33));
    }

    #[test]
    fn affect_non_empty_skips_space_and_braille_blank() {
        let ramp = AnimatedGlyphRamp::discrete(vec!['X'], vec![rgb(255, 0, 0)], 1.0);
        let mut space = make_cell(' ');
        let mut braille_blank = make_cell('\u{2800}');
        ramp.apply(&mut space, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        ramp.apply(
            &mut braille_blank,
            &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0),
        );
        assert_eq!(space.ch, ' ');
        assert_eq!(braille_blank.ch, '\u{2800}');
    }

    #[test]
    fn affect_all_overrides_whitespace() {
        let ramp = AnimatedGlyphRamp::new(
            vec!['X'],
            ColorMode::Discrete(vec![rgb(255, 0, 0)]),
            1.0,
            EasingCurve::default(),
            AnimatedGlyphRampApplyTo::Foreground,
            AffectMode::All,
            0.0,
            0.0,
        );
        let mut space = make_cell(' ');
        ramp.apply(&mut space, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(space.ch, 'X');
    }

    #[test]
    fn empty_glyphs_is_noop() {
        let ramp = AnimatedGlyphRamp::discrete(vec![], vec![], 1.0);
        let mut cell = make_cell('⣿');
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.5));
        assert_eq!(cell.ch, '⣿');
    }

    #[test]
    fn deterministic() {
        let ramp = AnimatedGlyphRamp::discrete(waves_glyphs(), three_color_discrete(), 1.5);
        let mut a = make_cell('⣿');
        let mut b = make_cell('⣿');
        ramp.apply(&mut a, &VfxCellContext::new(3, 4, 10, 10, 0, 0, 0.123));
        ramp.apply(&mut b, &VfxCellContext::new(3, 4, 10, 10, 0, 0, 0.123));
        assert_eq!(a.ch, b.ch);
        assert_eq!(a.fg, b.fg);
    }

    #[test]
    fn cycles_per_second_clamps_to_positive_minimum() {
        // Zero or negative cycles_per_second would produce NaN / divide-by-zero;
        // constructor must clamp.
        let ramp = AnimatedGlyphRamp::discrete(vec!['A', 'B'], vec![rgb(0, 0, 0); 2], 0.0);
        // Should not panic.
        let _ = ramp.glyph_index(0, 0, 1.0);
    }

    #[test]
    fn easing_shapes_shared_glyph_and_colour_phase() {
        let ramp = AnimatedGlyphRamp::new(
            vec!['A', 'B', 'C', 'D'],
            ColorMode::Discrete(vec![
                rgb(10, 0, 0),
                rgb(20, 0, 0),
                rgb(30, 0, 0),
                rgb(40, 0, 0),
            ]),
            1.0,
            tui_vfx_geometry::easing::EasingType::QuadIn.into(),
            AnimatedGlyphRampApplyTo::Foreground,
            AffectMode::All,
            0.0,
            0.0,
        );
        // Linear 0.5 would land at index 2. QuadIn eases 0.5 to 0.25,
        // so both glyph and colour stay paired at index 1.
        let mut cell = make_cell('x');
        ramp.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.5));
        assert_eq!(cell.ch, 'B');
        assert_eq!(cell.fg, rgb(20, 0, 0));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_animated_glyph_ramp.rs</FILE> - <DESC>Synchronised glyph + colour cycling filter driven by one shared phase signal</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
