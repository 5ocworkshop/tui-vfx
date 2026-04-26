// <FILE>tui-vfx-style/src/models/cls_water_field_signal.rs</FILE> - <DESC>Signal wrapper around TerminalWaterShader's WaterField for the glyph rendering framework</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 5: expose TerminalWaterShader's per-cell field via mixed_signals::Signal so ScalarFieldGlyphFilter (Phase 4) and water glyph recipes (Phase 6) can render water as braille/block/ramp glyphs.</WCTX>
// <CLOG>0.1.0: initial Signal + SignalWithSlope wrapper around TerminalWaterShader::sample_field_at; emits light_scalar as the unit-range signal output, returns analytic slope_x/slope_y to skip numeric differencing in subcell sampling.</CLOG>

//! Signal wrapper around [`TerminalWaterShader`]'s field math.
//!
//! [`WaterFieldSignal`] adapts the existing `TerminalWaterShader` field
//! evaluation (Gerstner waves + ripple/rain/flow/wake + foam/glint) to the
//! [`mixed_signals::traits::Signal`] surface so the
//! [`ScalarFieldGlyphFilter`] in `tui-vfx-compositor` can drive any glyph
//! encoder (braille subcell, eighths, block bars, ramps) from the same
//! water field that paints the upstream water shader.
//!
//! The signal output is the field's `light_scalar` — the same per-cell
//! lighting intensity the shader uses for color blending — clamped to the
//! unit range. The accompanying [`mixed_signals::traits::SignalWithSlope`]
//! impl returns the cached `slope_x`/`slope_y` from
//! [`super::cls_terminal_water_shader::WaterFieldSample`] so the
//! eight-subcell sampling helper in `tui-vfx-types::glyph` can use the
//! one-evaluation slope shortcut instead of eight full field samples.
//!
//! # Coordinate convention
//!
//! Mirrors the shader's existing `sample_field_for_ctx`:
//!
//! - `x = ctx.cell_x + subcell_offset.dx` (cell-space, fractional via subcell)
//! - `y = (ctx.cell_y + subcell_offset.dy) * 2.0` (water doubles y so braille
//!   subcell aspect ratio reads as visually square)
//! - `t = ctx.absolute_t / 1000.0` (tui-vfx writes elapsed milliseconds into
//!   `SignalContext::absolute_t` per the V3 direct-playback convention; the
//!   shader's field math expects seconds).
//!
//! # Send / Sync
//!
//! [`TerminalWaterShader`] holds only `Send + Sync` fields; the wrapper is
//! `Send + Sync` by default, satisfying [`mixed_signals::traits::Signal`]'s
//! bound.

use mixed_signals::traits::{
    Signal, SignalContext, SignalRange, SignalTime, SignalWithSlope, SlopeSample,
};

use crate::models::cls_terminal_water_shader::TerminalWaterShader;

/// [`Signal`] adapter around [`TerminalWaterShader`].
///
/// Construct via [`WaterFieldSignal::new`] from an authored or runtime-built
/// [`TerminalWaterShader`]. The wrapper consumes the shader by value
/// (the shader is `Clone`); recipe-time construction is a single move.
///
/// # Output
///
/// `sample` and `sample_with_context` return the field's `light_scalar`
/// in `[0.0, 1.0]`. [`Signal::output_range`] reports [`SignalRange::UNIT`].
///
/// # Example
///
/// ```ignore
/// use mixed_signals::traits::{Signal, SignalContext};
/// use tui_vfx_style::models::{
///     ColorConfig, TerminalWaterShader, WaterApplyTo, WaterFieldSignal, WaterWaveMode,
/// };
///
/// let shader = TerminalWaterShader::default();
/// let signal = WaterFieldSignal::new(shader);
///
/// let ctx = SignalContext::default()
///     .with_dimensions(80, 24)
///     .with_cell_position(40, 12)
///     .with_absolute_time(1000.0); // 1 second in ms
///
/// let intensity = signal.sample_with_context(0.0, &ctx);
/// assert!((0.0..=1.0).contains(&intensity));
/// ```
#[derive(Debug, Clone)]
pub struct WaterFieldSignal {
    shader: TerminalWaterShader,
}

impl WaterFieldSignal {
    /// Wrap a [`TerminalWaterShader`] for use as a [`Signal`].
    pub fn new(shader: TerminalWaterShader) -> Self {
        Self { shader }
    }

    /// Borrow the underlying shader (e.g. to inspect parameters in tests).
    pub fn shader(&self) -> &TerminalWaterShader {
        &self.shader
    }

    /// Resolve the (x, y, t) tuple the field math consumes from a
    /// [`SignalContext`]. Pulled out so [`Signal::sample_with_context`] and
    /// [`SignalWithSlope::sample_with_slope`] share one coordinate
    /// translation site.
    fn coords_from_ctx(&self, t: SignalTime, ctx: &SignalContext) -> (f32, f32, f32, u16, u16) {
        let cell_x = ctx.cell_x.unwrap_or(0) as f32;
        let cell_y = ctx.cell_y.unwrap_or(0) as f32;
        let (sub_dx, sub_dy) = ctx.subcell_offset.unwrap_or((0.0, 0.0));
        let x = cell_x + sub_dx;
        let y = (cell_y + sub_dy) * 2.0;
        let t_seconds = ctx
            .absolute_t
            .map(|ms| (ms / 1000.0) as f32)
            .unwrap_or(t as f32);
        let width = ctx.width.max(1);
        let height = ctx.height.max(1);
        (x, y, t_seconds, width, height)
    }
}

impl Signal for WaterFieldSignal {
    /// Cell-agnostic sample; returns the field's `light_scalar` at
    /// `(x=0, y=0, w=1, h=1)` at time `t` (seconds). Most consumers
    /// should call [`Signal::sample_with_context`] instead so spatial
    /// state threads through.
    fn sample(&self, t: SignalTime) -> f32 {
        self.shader
            .sample_field_at(0.0, 0.0, 1, 1, t as f32)
            .light_scalar
    }

    fn sample_with_context(&self, t: SignalTime, ctx: &SignalContext) -> f32 {
        let (x, y, t_seconds, width, height) = self.coords_from_ctx(t, ctx);
        self.shader
            .sample_field_at(x, y, width, height, t_seconds)
            .light_scalar
    }

    fn output_range(&self) -> SignalRange {
        SignalRange::UNIT
    }
}

impl SignalWithSlope for WaterFieldSignal {
    /// Return the field value plus its analytic gradient `(∂value/∂cell_x,
    /// ∂value/∂cell_y)` from a single field evaluation, using the cached
    /// `slope_x`/`slope_y` retained on [`crate::models::cls_terminal_water_shader::WaterFieldSample`].
    ///
    /// The shader's slopes are `∂height/∂x` and `∂height/∂y` in cell-space
    /// units. We expose them as the gradient of the *signal output*
    /// (`light_scalar`) to a first-order approximation: the relationship
    /// between height and `light_scalar` is dominated by the diffuse +
    /// foam terms, both monotone in height for typical water parameters,
    /// so the height gradient is a close-enough proxy for subcell linear
    /// extrapolation. Saves seven `sample_field_at` calls per cell when
    /// sampling braille subcells.
    fn sample_with_slope(&self, t: SignalTime, ctx: &SignalContext) -> SlopeSample {
        let (x, y, t_seconds, width, height) = self.coords_from_ctx(t, ctx);
        let sample = self.shader.sample_field_at(x, y, width, height, t_seconds);
        SlopeSample {
            value: sample.light_scalar,
            dx: sample.slope_x,
            dy: sample.slope_y,
        }
    }
}

#[cfg(test)]
#[path = "test_cls_water_field_signal.rs"]
mod tests;

// <FILE>tui-vfx-style/src/models/cls_water_field_signal.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
