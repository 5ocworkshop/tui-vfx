// <FILE>crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Generic scalar-field-to-glyph filter: samples any Signal and encodes intensity via GlyphEncoder</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Glyph rendering framework Phase 4: unifying filter for water/fire/terrain field effects</WCTX>
// <CLOG>0.2.0: drop unwired temporal_dither_hz field — encoders no longer take a time argument, so the snap-to-step value flowed nowhere; SubcellLight retains its own inline temporal-dither logic.</CLOG>

use mixed_signals::traits::{Signal, SignalContext};
use tui_vfx_types::{Cell, Color, glyph::GlyphEncoder, glyph::sample_eight_subcells};

use crate::traits::filter::Filter;

/// Generic scalar-field-to-glyph filter.
///
/// `S` is the field sampler (any [`Signal`]). On each [`Filter::apply`] call the
/// filter builds a [`SignalContext`] from `(x, y, width, height, t)`, samples
/// the signal once (or eight times for [`GlyphEncoder::BrailleSubcell`]), and
/// encodes the resulting intensity to a glyph character via [`GlyphEncoder`].
///
/// # Hot-path contract
///
/// `apply` makes no heap allocations. The `SignalContext` is constructed on the
/// stack via builder methods. `S` is monomorphized at compile time, keeping
/// dispatch out of the per-cell loop.
///
/// # Fields
///
/// - `sampler` — the signal to query per cell.
/// - `encoder` — glyph encoder variant (braille, block, ramp, etc.).
/// - `recolor` — `Some((lit, unlit))` overwrites `cell.fg`/`cell.bg` keyed by
///   which end of the axis the intensity falls on; `None` preserves the
///   producer's colors (typical for water/fire where the upstream shader has
///   already painted the cell).
/// - `threshold` — skip-guard for single-scalar encoders; cell is left
///   unchanged when `intensity <= threshold`.
/// - `only_blank` — skip non-blank cells (`cell.ch != ' '`) when `true`.
/// - `frame` / `seed` — forwarded into the [`SignalContext`] for signals that
///   use them for determinism or noise.
///
/// # Temporal dither
///
/// Time-stepped variation across frames belongs to the *signal* (sample at
/// quantized `absolute_t`) or to a future encoder variant that consumes time
/// (e.g. a hypothetical `BrailleEighthsTimeDither`). The legacy
/// [`crate::filters::cls_subcell_light::SubcellLight`] keeps its own inline
/// temporal-dither logic; this filter does not duplicate it.
///
/// # Example
///
/// ```rust,ignore
/// use mixed_signals::traits::{Signal, SignalContext, SignalTime};
/// use tui_vfx_compositor::filters::ScalarFieldGlyphFilter;
/// use tui_vfx_types::{Cell, Color, Modifiers, glyph::GlyphEncoder};
/// use tui_vfx_compositor::traits::filter::Filter;
///
/// struct ConstSignal(f32);
/// impl Signal for ConstSignal {
///     fn sample(&self, _t: SignalTime) -> f32 { self.0 }
/// }
///
/// let filter = ScalarFieldGlyphFilter {
///     sampler: ConstSignal(0.5),
///     encoder: GlyphEncoder::BlockHorizontal,
///     recolor: None,
///     threshold: 0.0,
///     only_blank: false,
///     frame: 0,
///     seed: 0,
/// };
/// let mut cell = Cell::styled(' ', Color::WHITE, Color::BLACK, Modifiers::NONE);
/// filter.apply(&mut cell, 4, 0, 8, 1, 0.0);
/// assert_eq!(cell.ch, '▌');
/// ```
// Public type; constructed from non-test code starting in Phase 6 of
// `docs/design/tui-vfx-glyph-rendering-framework-plan.md` when `FilterSpec`
// gains a `ScalarFieldGlyph` discriminant. The expectation applies only in
// non-test builds — tests construct the type directly, so `dead_code` does
// not fire under `--all-targets`. `#[expect]` auto-retires the moment Phase 6
// wires a non-test caller; the build fails if the expectation no longer
// applies in lib-only mode.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "wired in Phase 6 via FilterSpec::ScalarFieldGlyph")
)]
pub struct ScalarFieldGlyphFilter<S: Signal> {
    /// The field sampler; sampled once per `apply` call (or eight times for
    /// `BrailleSubcell`).
    pub sampler: S,
    /// Glyph encoder variant applied to the sampled intensity.
    pub encoder: GlyphEncoder,
    /// `Some((lit, unlit))` paints `cell.fg`/`cell.bg` after encoding.
    /// `None` leaves colors untouched.
    pub recolor: Option<(Color, Color)>,
    /// Minimum intensity below which the cell is left unchanged.
    /// Checked after sampling; not applied to `BrailleSubcell` (which has its
    /// own per-dot threshold).
    pub threshold: f32,
    /// When `true`, cells whose `ch` is not `' '` are skipped.
    pub only_blank: bool,
    /// Frame counter forwarded into `SignalContext`.
    pub frame: u64,
    /// Noise seed forwarded into `SignalContext`.
    pub seed: u64,
}

impl<S: Signal> Filter for ScalarFieldGlyphFilter<S> {
    /// Apply the scalar-field-to-glyph transform to one cell.
    ///
    /// No allocations. The `SignalContext` is stack-constructed via builder
    /// methods. All fields that are not `width`/`height`/`cell_x`/`cell_y`/
    /// `absolute_t` default to `None`/`0` via `SignalContext::new`.
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        if self.only_blank && cell.ch != ' ' {
            return;
        }

        let ctx = SignalContext::new(self.frame, self.seed)
            .with_dimensions(width, height)
            .with_cell_position(x, y)
            .with_absolute_time(t);

        let new_ch = match &self.encoder {
            GlyphEncoder::BrailleSubcell { .. } => {
                let subcells = sample_eight_subcells(&self.sampler, &ctx, t);
                self.encoder.encode_subcell(subcells, x, y)
            }
            _ => {
                let v = self.sampler.sample_with_context(t, &ctx);
                if v <= self.threshold {
                    return;
                }
                self.encoder.encode_one(v, x, y)
            }
        };

        cell.ch = new_ch;
        if let Some((lit, unlit)) = self.recolor {
            cell.fg = lit;
            cell.bg = unlit;
        }
    }
}

#[cfg(test)]
#[path = "test_cls_scalar_field_glyph_filter.rs"]
mod tests;

// <FILE>crates/tui-vfx-compositor/src/filters/cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Generic scalar-field-to-glyph filter: samples any Signal and encodes intensity via GlyphEncoder</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
