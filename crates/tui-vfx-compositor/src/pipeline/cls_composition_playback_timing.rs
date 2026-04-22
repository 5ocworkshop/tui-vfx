// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_playback_timing.rs</FILE> - <DESC>Shared playback timing for compositor-facing composition surfaces</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Multiple callers build CompositionSpec/CompositionOptions by hand and repeat the same t/loop_t/phase clamping. Centralizing that timing bundle here keeps runtime-facing timing ownership inside the compositor layer.</WCTX>
// <CLOG>0.1.0: add CompositionPlaybackTiming with clamped constructor for runtime-facing composition surfaces.</CLOG>

use mixed_signals::traits::Phase;

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
}
