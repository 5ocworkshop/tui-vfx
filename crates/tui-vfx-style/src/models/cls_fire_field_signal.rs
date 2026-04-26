// <FILE>tui-vfx-style/src/models/cls_fire_field_signal.rs</FILE> - <DESC>Signal wrapper around TerminalFireShader's emission field for the glyph rendering framework</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 6: expose TerminalFireShader's per-cell emissive field via mixed_signals::Signal so ScalarFieldGlyphFilter can render fire as braille/block/ramp glyphs without a parallel sampler.</WCTX>
// <CLOG>0.1.0: initial Signal wrapper around TerminalFireShader::sample_field_at; emits intensity as the unit-range output. Uses SignalWithSlope's default central-differencing impl rather than overriding — fire's pipeline does not yield analytic slopes for free, so the 3-call default is the right tradeoff.</CLOG>

//! Signal wrapper around [`TerminalFireShader`]'s emissive field.
//!
//! [`FireFieldSignal`] adapts the existing `TerminalFireShader` field
//! evaluation (rising turbulence + temperature/density/smoke/blue-core +
//! sparks) to the [`mixed_signals::traits::Signal`] surface so the
//! [`ScalarFieldGlyphFilter`] in `tui-vfx-compositor` can drive any glyph
//! encoder (braille subcell, eighths, block bars, ramps) from the same
//! fire field that paints the upstream fire shader.
//!
//! The signal output is the field's `intensity` — the same per-cell
//! emissive value the shader uses for color blending — clamped to the
//! unit range. [`mixed_signals::traits::SignalWithSlope`] is implemented
//! with the trait's *default* central-differencing impl rather than
//! overridden: unlike water (whose Gerstner waves yield free analytic
//! `slope_x`/`slope_y` during normal evaluation), fire's combined
//! pipeline (mask × density × smoothsteps × exp × spark Gaussians) does
//! not produce gradients cheaply. The 3-call default is the right
//! tradeoff vs the cost of either differentiating every term or caching
//! finite differences.
//!
//! # Coordinate convention
//!
//! Mirrors the shader's [`TerminalFireShader::sample_field_at`] entry
//! point (raw cell coords, no y-doubling — fire's own normalization
//! handles aspect):
//!
//! - `x = ctx.cell_x + subcell_offset.dx` (cell-space, fractional via subcell)
//! - `y = ctx.cell_y + subcell_offset.dy` (cell-space; fire's internal
//!   `normalized_fire_coord` flips this so `y=0` ends up at the flame base)
//! - `t = ctx.absolute_t / 1000.0` (tui-vfx writes elapsed milliseconds
//!   into `SignalContext::absolute_t` per the V3 direct-playback
//!   convention; the shader's field math expects seconds).
//!
//! # Send / Sync
//!
//! [`TerminalFireShader`] holds only `Send + Sync` fields; the wrapper
//! is `Send + Sync` by default, satisfying [`mixed_signals::traits::Signal`]'s
//! bound.

use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalTime, SignalWithSlope};

use crate::models::cls_terminal_fire_shader::TerminalFireShader;

/// [`Signal`] adapter around [`TerminalFireShader`].
///
/// Construct via [`FireFieldSignal::new`] from an authored or runtime-built
/// [`TerminalFireShader`]. The wrapper consumes the shader by value
/// (the shader is `Clone`); recipe-time construction is a single move.
///
/// # Output
///
/// `sample` and `sample_with_context` return the field's `intensity`
/// in `[0.0, 1.0]`. [`Signal::output_range`] reports [`SignalRange::UNIT`].
///
/// # Example
///
/// ```ignore
/// use mixed_signals::traits::{Signal, SignalContext};
/// use tui_vfx_style::models::{FireFieldSignal, FireMode, TerminalFireShader};
///
/// let shader = TerminalFireShader {
///     mode: FireMode::Candle,
///     ..TerminalFireShader::default()
/// };
/// let signal = FireFieldSignal::new(shader);
///
/// let ctx = SignalContext::default()
///     .with_dimensions(40, 12)
///     .with_cell_position(20, 11)
///     .with_absolute_time(1000.0); // 1 second in ms
///
/// let intensity = signal.sample_with_context(0.0, &ctx);
/// assert!((0.0..=1.0).contains(&intensity));
/// ```
#[derive(Debug, Clone)]
pub struct FireFieldSignal {
    shader: TerminalFireShader,
}

impl FireFieldSignal {
    /// Wrap a [`TerminalFireShader`] for use as a [`Signal`].
    pub fn new(shader: TerminalFireShader) -> Self {
        Self { shader }
    }

    /// Borrow the underlying shader (for inspection in tests / introspection).
    pub fn shader(&self) -> &TerminalFireShader {
        &self.shader
    }

    /// Resolve the `(x, y, t, width, height)` tuple the field math
    /// consumes from a [`SignalContext`]. Pulled out so
    /// [`Signal::sample_with_context`] and any future sibling code
    /// share one coordinate translation site.
    fn coords_from_ctx(&self, t: SignalTime, ctx: &SignalContext) -> (f32, f32, f32, u16, u16) {
        let cell_x = ctx.cell_x.unwrap_or(0) as f32;
        let cell_y = ctx.cell_y.unwrap_or(0) as f32;
        let (sub_dx, sub_dy) = ctx.subcell_offset.unwrap_or((0.0, 0.0));
        let x = cell_x + sub_dx;
        let y = cell_y + sub_dy;
        let t_seconds = ctx
            .absolute_t
            .map(|ms| (ms / 1000.0) as f32)
            .unwrap_or(t as f32);
        let width = ctx.width.max(1);
        let height = ctx.height.max(1);
        (x, y, t_seconds, width, height)
    }
}

impl Signal for FireFieldSignal {
    /// Cell-agnostic sample; returns the field's `intensity` at
    /// `(x=0, y=0, w=1, h=1)` at time `t` (seconds). Most consumers
    /// should call [`Signal::sample_with_context`] instead so spatial
    /// state threads through.
    fn sample(&self, t: SignalTime) -> f32 {
        self.shader
            .sample_field_at(0.0, 0.0, 1, 1, t as f32)
            .intensity
    }

    fn sample_with_context(&self, t: SignalTime, ctx: &SignalContext) -> f32 {
        let (x, y, t_seconds, width, height) = self.coords_from_ctx(t, ctx);
        self.shader
            .sample_field_at(x, y, width, height, t_seconds)
            .intensity
    }

    fn output_range(&self) -> SignalRange {
        SignalRange::UNIT
    }
}

/// Use the [`SignalWithSlope`] default central-differencing impl.
///
/// Fire's combined pipeline (mask × density × smoothsteps × exp × spark
/// Gaussians) does not yield free analytic gradients during normal
/// evaluation — the way water's Gerstner waves do. Computing slopes
/// would either require manually differentiating every term (brittle)
/// or running ~3 evaluations and caching differences (no win over the
/// trait default). The 3-call default is the right tradeoff here.
impl SignalWithSlope for FireFieldSignal {}

#[cfg(test)]
#[path = "test_cls_fire_field_signal.rs"]
mod tests;

// <FILE>tui-vfx-style/src/models/cls_fire_field_signal.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
