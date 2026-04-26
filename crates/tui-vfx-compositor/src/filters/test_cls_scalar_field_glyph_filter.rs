// <FILE>crates/tui-vfx-compositor/src/filters/test_cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Tests for ScalarFieldGlyphFilter — encoding, threshold, only_blank, recolor, SignalContext wiring</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Glyph rendering framework Phase 4: TDD tests for ScalarFieldGlyphFilter</WCTX>
// <CLOG>0.2.0: drop temporal_dither_hz field references and the dead test that asserted on it (the field never wired through to the encoder; SubcellLight retains its inline temporal-dither path).</CLOG>

use std::sync::{Arc, Mutex};

use mixed_signals::traits::{Signal, SignalContext, SignalTime};
use tui_vfx_types::{Cell, Color, Modifiers, glyph::GlyphEncoder};

use super::ScalarFieldGlyphFilter;
use crate::traits::filter::Filter;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

fn make_cell(ch: char, fg: Color, bg: Color) -> Cell {
    Cell::styled(ch, fg, bg, Modifiers::NONE)
}

/// A signal that always returns the same scalar.
struct ConstSignal(f32);

impl Signal for ConstSignal {
    fn sample(&self, _t: SignalTime) -> f32 {
        self.0
    }
}

/// A signal that returns `cell_x / width` from the SignalContext.
struct SpatialXSignal;

impl Signal for SpatialXSignal {
    fn sample(&self, _t: SignalTime) -> f32 {
        0.0
    }

    fn sample_with_context(&self, _t: SignalTime, ctx: &SignalContext) -> f32 {
        let x = ctx.cell_x.unwrap_or(0) as f32;
        let w = ctx.width as f32;
        if w > 0.0 { x / w } else { 0.0 }
    }
}

/// A signal that records the SignalContext it was called with.
struct ContextCapture {
    captured: Arc<Mutex<Option<SignalContext>>>,
}

impl Signal for ContextCapture {
    fn sample(&self, _t: SignalTime) -> f32 {
        0.5
    }

    fn sample_with_context(&self, _t: SignalTime, ctx: &SignalContext) -> f32 {
        *self.captured.lock().unwrap() = Some(ctx.clone());
        0.5
    }
}

/// A signal that records the `t` arg it was called with. Used to prove
/// the filter threads loop_t through to the signal's primary time arg
/// (rather than burying it in `absolute_t` which signals interpret as ms).
struct TimeCapture {
    captured_t: Arc<Mutex<Option<SignalTime>>>,
}

impl Signal for TimeCapture {
    fn sample(&self, t: SignalTime) -> f32 {
        *self.captured_t.lock().unwrap() = Some(t);
        0.5
    }

    fn sample_with_context(&self, t: SignalTime, _ctx: &SignalContext) -> f32 {
        *self.captured_t.lock().unwrap() = Some(t);
        0.5
    }
}

fn default_filter(
    sampler: impl Signal,
    encoder: GlyphEncoder,
) -> ScalarFieldGlyphFilter<impl Signal> {
    ScalarFieldGlyphFilter {
        sampler,
        encoder,
        recolor: None,
        threshold: 0.0,
        only_blank: false,
        frame: 0,
        seed: 0,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// A constant-intensity signal routes through BlockHorizontal and writes the
/// expected partial-block character.
#[test]
fn test_apply_writes_glyph_via_encoder() {
    let intensity = 0.5;
    let encoder = GlyphEncoder::BlockHorizontal;
    let expected = encoder.encode_one(intensity, 0, 0);

    let filter = default_filter(ConstSignal(intensity), GlyphEncoder::BlockHorizontal);
    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);

    assert_eq!(
        cell.ch, expected,
        "expected encoder output '{expected}', got '{}'",
        cell.ch
    );
}

/// When intensity falls at or below threshold the cell character must not change.
#[test]
fn test_apply_skips_below_threshold() {
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(0.5),
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.6,
        recolor: None,
        only_blank: false,
        frame: 0,
        seed: 0,
    };
    let original = make_cell(' ', Color::WHITE, Color::BLACK);
    let mut cell = original;
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
    assert_eq!(
        cell, original,
        "cell must be unchanged when intensity <= threshold"
    );
}

/// A non-space cell with only_blank = true must be left untouched.
#[test]
fn test_apply_only_blank_skips_non_space_cells() {
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(1.0),
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: true,
        frame: 0,
        seed: 0,
    };
    let original = make_cell('X', Color::WHITE, Color::BLACK);
    let mut cell = original;
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
    assert_eq!(
        cell, original,
        "non-blank cell must be untouched when only_blank is true"
    );
}

/// A blank cell with only_blank = true must be processed.
#[test]
fn test_apply_only_blank_processes_space_cells() {
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(1.0),
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: true,
        frame: 0,
        seed: 0,
    };
    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
    assert_ne!(
        cell.ch, ' ',
        "blank cell should be encoded when only_blank is true"
    );
}

/// recolor: Some overwrites fg/bg with the provided lit/unlit colors.
#[test]
fn test_apply_recolor_some_overrides_colors() {
    let lit = Color::WHITE;
    let unlit = Color::BLACK;
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(1.0),
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: Some((lit, unlit)),
        only_blank: false,
        frame: 0,
        seed: 0,
    };
    let mut cell = make_cell(' ', Color::rgb(10, 20, 30), Color::rgb(40, 50, 60));
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
    assert_eq!(cell.fg, lit, "fg must be overwritten by lit");
    assert_eq!(cell.bg, unlit, "bg must be overwritten by unlit");
}

/// recolor: None leaves fg/bg unchanged.
#[test]
fn test_apply_recolor_none_preserves_colors() {
    let orig_fg = Color::rgb(10, 20, 30);
    let orig_bg = Color::rgb(40, 50, 60);
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(1.0),
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: false,
        frame: 0,
        seed: 0,
    };
    let mut cell = make_cell(' ', orig_fg, orig_bg);
    filter.apply(&mut cell, 0, 0, 10, 5, 0.0);
    assert_eq!(
        cell.fg, orig_fg,
        "fg must be preserved when recolor is None"
    );
    assert_eq!(
        cell.bg, orig_bg,
        "bg must be preserved when recolor is None"
    );
}

/// width, height, cell_x, cell_y must be populated; absolute_t must NOT be set.
///
/// This test pins the `SignalContext::new(...).with_dimensions(...).with_cell_position(...)`
/// construction form. The Filter trait's `t` is normalized loop progress
/// (0.0..=1.0), not elapsed milliseconds — so we deliberately do NOT call
/// `with_absolute_time(t)`. Signals receiving this context fall back to
/// using the `t` arg of `sample_with_context` directly, which matches the
/// `StyleShader::style_at` convention for the same shaders' non-glyph path.
/// (See file's CLOG 0.4.0 for the Phase 6 follow-up that fixed the freeze.)
#[test]
fn test_apply_constructs_signal_context_with_default_form() {
    let captured: Arc<Mutex<Option<SignalContext>>> = Arc::new(Mutex::new(None));
    let filter = ScalarFieldGlyphFilter {
        sampler: ContextCapture {
            captured: captured.clone(),
        },
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: false,
        frame: 42,
        seed: 7,
    };

    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 3, 5, 16, 9, 1.5);

    let ctx = captured
        .lock()
        .unwrap()
        .take()
        .expect("signal must have been called");

    // Populated fields
    assert_eq!(ctx.width, 16, "width must be set");
    assert_eq!(ctx.height, 9, "height must be set");
    assert_eq!(ctx.cell_x, Some(3), "cell_x must be set");
    assert_eq!(ctx.cell_y, Some(5), "cell_y must be set");
    assert_eq!(ctx.frame, 42, "frame must be forwarded");
    assert_eq!(ctx.seed, 7, "seed must be forwarded");

    // absolute_t must NOT be set: signals receive normalized loop_t through
    // the `t` arg of sample_with_context, not via absolute_t (which they
    // would interpret as elapsed milliseconds and divide by 1000, freezing
    // the field at near-zero time).
    assert!(
        ctx.absolute_t.is_none(),
        "absolute_t must remain None; the Filter trait passes normalized loop_t, not ms"
    );

    // Optional fields default (not set by the filter)
    assert!(ctx.phase.is_none(), "phase must default to None");
    assert!(ctx.phase_t.is_none(), "phase_t must default to None");
    assert!(ctx.loop_t.is_none(), "loop_t must default to None");
    assert!(ctx.char_index.is_none(), "char_index must default to None");
    assert!(
        ctx.subcell_offset.is_none(),
        "subcell_offset must default to None"
    );
}

/// Regression: the filter's `t` arg must thread through to
/// `Signal::sample_with_context(t, ctx)`, not get buried in `ctx.absolute_t`.
/// Field signals (WaterFieldSignal, FireFieldSignal) interpret `absolute_t`
/// as elapsed milliseconds and divide by 1000; threading normalized loop_t
/// through there freezes the field. This test pins the corrected contract:
/// loop_t arrives at the signal's `t` arg with the same value the caller
/// passed to `apply`.
#[test]
fn test_apply_threads_loop_t_through_signal_t_arg() {
    let captured_t: Arc<Mutex<Option<SignalTime>>> = Arc::new(Mutex::new(None));
    let filter = ScalarFieldGlyphFilter {
        sampler: TimeCapture {
            captured_t: captured_t.clone(),
        },
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: false,
        frame: 0,
        seed: 0,
    };

    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 0, 0, 4, 4, 0.42);

    let t = captured_t
        .lock()
        .unwrap()
        .take()
        .expect("signal must have been called");
    assert!(
        (t - 0.42).abs() < 1e-9,
        "filter must pass loop_t (0.42) to signal.t-arg, got {t}"
    );

    // And again at a different t — proves animation isn't pinned at zero.
    filter.apply(&mut cell, 0, 0, 4, 4, 0.83);
    let t2 = captured_t
        .lock()
        .unwrap()
        .take()
        .expect("signal must have been called twice");
    assert!(
        (t2 - 0.83).abs() < 1e-9,
        "second call must thread its loop_t (0.83), got {t2}"
    );
}

/// From the plan §7: `(cell_x as f32) / (width as f32)` at x=4, w=8 → 0.5 → '▌'.
/// Proves end-to-end signal-driven BlockHorizontal encoding.
#[test]
fn test_apply_spatial_x_signal_blank_cell_produces_half_block() {
    let filter = ScalarFieldGlyphFilter {
        sampler: SpatialXSignal,
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: false,
        frame: 0,
        seed: 0,
    };
    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 4, 0, 8, 1, 0.0);
    assert_eq!(cell.ch, '▌', "x=4, w=8 → intensity=0.5 → '▌'");
}

/// only_blank=true applied to a non-blank cell leaves it untouched (plan §7).
#[test]
fn test_apply_only_blank_skips_text() {
    let filter = ScalarFieldGlyphFilter {
        sampler: SpatialXSignal,
        encoder: GlyphEncoder::BlockHorizontal,
        threshold: 0.0,
        recolor: None,
        only_blank: true,
        frame: 0,
        seed: 0,
    };
    let original = make_cell('X', Color::WHITE, Color::BLACK);
    let mut cell = original;
    filter.apply(&mut cell, 4, 0, 8, 1, 0.0);
    assert_eq!(
        cell, original,
        "non-blank cell must survive only_blank=true"
    );
}

/// BrailleSubcell path calls encode_subcell via sample_eight_subcells.
/// A signal that varies in x across the cell must produce a glyph with at least
/// one dot lit (plan §7 test_scalar_field_glyph_filter_braille_subcell_uses_eight_samples).
#[test]
fn test_apply_braille_subcell_uses_eight_samples() {
    use tui_vfx_types::braille::braille_bits;

    // Signal returns 0.8 everywhere — all subcells above threshold 0.5.
    let filter = ScalarFieldGlyphFilter {
        sampler: ConstSignal(0.8),
        encoder: GlyphEncoder::BrailleSubcell { threshold: 0.5 },
        threshold: 0.0,
        recolor: None,
        only_blank: false,
        frame: 0,
        seed: 0,
    };
    let mut cell = make_cell(' ', Color::WHITE, Color::BLACK);
    filter.apply(&mut cell, 0, 0, 8, 4, 0.0);

    let bits = braille_bits(cell.ch).expect("must produce a braille char");
    assert!(
        bits.count_ones() > 0,
        "BrailleSubcell with intensity 0.8 > threshold 0.5 must light at least one dot"
    );
}

// <FILE>crates/tui-vfx-compositor/src/filters/test_cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Tests for ScalarFieldGlyphFilter — encoding, threshold, only_blank, recolor, SignalContext wiring</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
