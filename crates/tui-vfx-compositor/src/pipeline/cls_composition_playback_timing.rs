// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_playback_timing.rs</FILE> - <DESC>Shared playback timing for compositor-facing composition surfaces</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Multiple callers still derive effective loop/shader timing from composition surfaces separately. Extending this bundle keeps more runtime-facing timing ownership inside the compositor layer.</WCTX>
// <CLOG>0.2.0: add helpers for effective loop/shader timing and construction from CompositionSpec/CompositionOptions.</CLOG>

use mixed_signals::traits::Phase;

use crate::pipeline::{CompositionOptions, CompositionSpec};

/// Shared playback timing for compositor-facing composition surfaces.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompositionPlaybackTiming {
    pub t: f64,
    pub loop_t: Option<f64>,
    pub phase: Option<Phase>,
}

impl CompositionPlaybackTiming {
    /// Construct one playback timing bundle, clamping normalized progress to
    /// compositor-friendly ranges.
    pub fn new(t: f64, loop_t: Option<f64>, phase: Option<Phase>) -> Self {
        Self {
            t: t.clamp(0.0, 1.0),
            loop_t: loop_t.map(|value| value.clamp(0.0, 1.0)),
            phase,
        }
    }

    /// Build timing from a serializable composition spec.
    pub fn from_spec(spec: &CompositionSpec) -> Self {
        Self::new(spec.t, spec.loop_t, spec.phase)
    }

    /// Build timing from runtime composition options.
    pub fn from_options(options: &CompositionOptions<'_>) -> Self {
        Self::new(options.t, options.loop_t, options.phase)
    }

    /// Effective loop clock used by filters, samplers, and runtime bindings.
    pub fn effective_loop_t(&self) -> f64 {
        self.loop_t.unwrap_or(self.t)
    }

    /// Effective shader progress used by spatial shader evaluation.
    pub fn shader_t(&self) -> f64 {
        self.effective_loop_t().clamp(0.0, 1.0)
    }
}
