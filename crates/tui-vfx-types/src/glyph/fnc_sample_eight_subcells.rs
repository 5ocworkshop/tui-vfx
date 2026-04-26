// <FILE>crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs</FILE> - <DESC>sample_eight_subcells and sample_eight_subcells_with_slope: sample any Signal at the eight braille subcell positions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 3: subcell sampling helpers for water/fire/future field-effect glyph encoding</WCTX>
// <CLOG>0.1.0: initial implementation with TDD coverage and rustdoc</CLOG>

use mixed_signals::traits::{Signal, SignalContext, SignalTime};
use mixed_signals::traits::{SignalWithSlope, SlopeSample};

/// Subcell offset table — eight `(dx, dy)` pairs in `[0.0, 1.0]` space, ordered
/// by braille dot number minus one.
///
/// `SUBCELL_OFFSETS[0]` is dot 1, `[1]` is dot 2, ..., `[7]` is dot 8.
///
/// Mapping from the braille dot diagram (see `tui-vfx-types::braille`):
///
/// ```text
/// ┌───┐
/// │1 4│   Left column (dx=0.25):  dots 1, 2, 3, 7
/// │2 5│   Right column (dx=0.75): dots 4, 5, 6, 8
/// │3 6│   Rows (dy): row1=0.125, row2=0.375, row3=0.625, row4=0.875
/// │7 8│
/// └───┘
/// ```
///
/// | Index | Dot | (dx, dy)       |
/// |-------|-----|----------------|
/// | 0     | 1   | (0.25, 0.125)  |
/// | 1     | 2   | (0.25, 0.375)  |
/// | 2     | 3   | (0.25, 0.625)  |
/// | 3     | 4   | (0.75, 0.125)  |
/// | 4     | 5   | (0.75, 0.375)  |
/// | 5     | 6   | (0.75, 0.625)  |
/// | 6     | 7   | (0.25, 0.875)  |
/// | 7     | 8   | (0.75, 0.875)  |
pub const SUBCELL_OFFSETS: [(f32, f32); 8] = [
    (0.25, 0.125), // dot 1
    (0.25, 0.375), // dot 2
    (0.25, 0.625), // dot 3
    (0.75, 0.125), // dot 4
    (0.75, 0.375), // dot 5
    (0.75, 0.625), // dot 6
    (0.25, 0.875), // dot 7
    (0.75, 0.875), // dot 8
];

/// Sample any `Signal` at the eight braille subcell positions inside one cell.
///
/// The returned array is indexed by braille dot number minus one (matches
/// [`GlyphEncoder::encode_subcell`]'s expected order), so a `BrailleSubcell`
/// encoder can pack the result directly without remap.
///
/// Each subcell call clones `ctx`, applies `with_subcell_offset(dx, dy)`, and
/// invokes `signal.sample_with_context(t, &sub_ctx)`. The cost is eight full
/// signal evaluations per cell. For samplers with cheap analytic gradients,
/// prefer [`sample_eight_subcells_with_slope`] which calls once.
///
/// # Example
///
/// ```rust
/// use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalTime};
/// use tui_vfx_types::glyph::sample_eight_subcells;
///
/// struct ConstantSignal;
/// impl Signal for ConstantSignal {
///     fn sample(&self, _t: SignalTime) -> f32 { 0.5 }
/// }
///
/// let signal = ConstantSignal;
/// let ctx = SignalContext::default();
/// let subcells = sample_eight_subcells(&signal, &ctx, 0.0);
/// assert_eq!(subcells, [0.5; 8]);
/// ```
pub fn sample_eight_subcells<S: Signal>(
    signal: &S,
    ctx: &SignalContext,
    t: SignalTime,
) -> [f32; 8] {
    let mut result = [0.0_f32; 8];
    for (i, &(dx, dy)) in SUBCELL_OFFSETS.iter().enumerate() {
        let sub_ctx = ctx.clone().with_subcell_offset(dx, dy);
        result[i] = signal.sample_with_context(t, &sub_ctx);
    }
    result
}

/// Sample any `SignalWithSlope` once and approximate eight subcell scalars
/// using the analytic slope.
///
/// For each subcell offset `(dx, dy)`, the result is:
/// ```text
/// value + (dx - 0.5) * slope.dx + (dy - 0.5) * slope.dy
/// ```
/// where `slope` is the [`SlopeSample`] returned by
/// `signal.sample_with_slope(t, ctx)`. This is linear interpolation from the
/// cell-center sample and is an O(1)-evaluation shortcut for the hot path.
///
/// Use this when the sampler can supply slopes cheaply (e.g., a water shader
/// that already computes slope_x/slope_y inside its field math). Use
/// [`sample_eight_subcells`] when slopes are unavailable or expensive.
///
/// The default `SignalWithSlope` implementation uses symmetric numeric
/// differencing (three `sample_with_context` calls), which is cheaper than
/// eight full evaluations but more expensive than analytic gradients.
///
/// # Example
///
/// ```rust
/// use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalTime};
/// use mixed_signals::traits::{SignalWithSlope, SlopeSample};
/// use tui_vfx_types::glyph::{sample_eight_subcells_with_slope, SUBCELL_OFFSETS};
///
/// struct FlatSignal;
/// impl Signal for FlatSignal {
///     fn sample(&self, _t: SignalTime) -> f32 { 0.5 }
/// }
/// impl SignalWithSlope for FlatSignal {}
///
/// let signal = FlatSignal;
/// let ctx = SignalContext::default();
/// let subcells = sample_eight_subcells_with_slope(&signal, &ctx, 0.0);
/// // Flat signal: all subcells ≈ 0.5
/// for v in subcells {
///     assert!((v - 0.5).abs() < 0.01);
/// }
/// ```
pub fn sample_eight_subcells_with_slope<S: SignalWithSlope>(
    signal: &S,
    ctx: &SignalContext,
    t: SignalTime,
) -> [f32; 8] {
    let SlopeSample { value, dx, dy } = signal.sample_with_slope(t, ctx);
    let mut result = [0.0_f32; 8];
    for (i, &(ox, oy)) in SUBCELL_OFFSETS.iter().enumerate() {
        result[i] = value + (ox - 0.5) * dx + (oy - 0.5) * dy;
    }
    result
}

#[cfg(test)]
#[path = "test_fnc_sample_eight_subcells.rs"]
mod tests;

// <FILE>crates/tui-vfx-types/src/glyph/fnc_sample_eight_subcells.rs</FILE> - <DESC>sample_eight_subcells and sample_eight_subcells_with_slope: sample any Signal at the eight braille subcell positions</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
