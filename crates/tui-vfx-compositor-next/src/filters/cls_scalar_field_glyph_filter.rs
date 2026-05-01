// <FILE>crates/tui-vfx-compositor-next/src/filters/cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Generic scalar-field-to-glyph filter: samples any Signal and encodes intensity via GlyphEncoder</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>0.4.1: migrate apply signature to &VfxCellContext; extract x/y/width/height/t from ctx fields.</CLOG>

use mixed_signals::traits::{Signal, SignalContext};
use tui_vfx_types::{
    Cell, Color, VfxCellContext, glyph::GlyphEncoder, glyph::sample_eight_subcells,
};

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
/// use tui_vfx_compositor_next::filters::ScalarFieldGlyphFilter;
/// use tui_vfx_types::{Cell, Color, Modifiers, glyph::GlyphEncoder};
/// use tui_vfx_compositor_next::traits::filter::Filter;
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
/// filter.apply(&mut cell, &VfxCellContext::new(4, 0, 8, 1, 0, 0, 0.0));
/// assert_eq!(cell.ch, '▌');
/// ```
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
    /// methods. All fields that are not `width`/`height`/`cell_x`/`cell_y`
    /// default to `None`/`0` via `SignalContext::new`.
    ///
    /// **Time threading.** The Filter trait's `t` is normalized loop progress
    /// (0.0 = start, 1.0 = end of the recipe's clock period) — not elapsed
    /// milliseconds. We therefore do **not** call `with_absolute_time(t)`:
    /// `Signal::sample_with_context` falls back to using `t` directly when
    /// `ctx.absolute_t` is `None`, which matches the convention
    /// `StyleShader::style_at` uses for the same shaders' non-glyph path.
    /// Setting `absolute_t = t` here would cause WaterFieldSignal /
    /// FireFieldSignal (and any future field signal that follows the same
    /// "absolute_t is in milliseconds" convention) to divide by 1000 and
    /// freeze the field at near-zero time.
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
        let x = ctx.local_x;
        let y = ctx.local_y;
        let width = ctx.width;
        let height = ctx.height;
        let t = ctx.t;
        if self.only_blank && cell.ch != ' ' {
            return;
        }

        let ctx = SignalContext::new(self.frame, self.seed)
            .with_dimensions(width, height)
            .with_cell_position(x, y);

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

// <FILE>crates/tui-vfx-compositor-next/src/filters/cls_scalar_field_glyph_filter.rs</FILE>
// <DESC>Generic scalar-field-to-glyph filter: samples any Signal and encodes intensity via GlyphEncoder</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
