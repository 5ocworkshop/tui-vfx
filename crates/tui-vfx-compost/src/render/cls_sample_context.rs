// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>SampleContext separates normalized progress from elapsed sample clocks.</WCTX>
// <CLOG>0.4.0: MINOR — add explicit phase and loop elapsed milliseconds beside normalized phase/loop coordinates.</CLOG>

use tui_vfx_contract::LifecyclePhase;

/// Explicit sample context for native compost rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SampleContext {
    /// Normalized lifecycle phase coordinate for this deterministic sample.
    pub phase_t: f64,
    /// Optional normalized loop coordinate for loop-driven effects.
    pub loop_t: Option<f64>,
    /// Optional absolute recipe sample timestamp in milliseconds.
    pub absolute_time_ms: Option<u64>,
    /// Optional elapsed time in the active lifecycle phase, in milliseconds.
    pub phase_time_ms: Option<u64>,
    /// Optional elapsed time in the current loop, in milliseconds.
    pub loop_time_ms: Option<u64>,
    /// Optional named lifecycle phase active for this sample.
    pub lifecycle_phase: Option<LifecyclePhase>,
}

impl SampleContext {
    /// Build a sample context with explicit normalized phase progress.
    pub fn new(phase_t: f64) -> Self {
        Self {
            phase_t,
            loop_t: None,
            absolute_time_ms: None,
            phase_time_ms: None,
            loop_time_ms: None,
            lifecycle_phase: None,
        }
    }

    /// Attach an explicit normalized loop coordinate.
    pub fn with_loop_t(mut self, loop_t: f64) -> Self {
        self.loop_t = Some(loop_t);
        self
    }

    /// Attach an absolute recipe sample timestamp in milliseconds.
    pub fn with_absolute_time_ms(mut self, absolute_time_ms: u64) -> Self {
        self.absolute_time_ms = Some(absolute_time_ms);
        self
    }

    /// Attach elapsed time in the active lifecycle phase.
    pub fn with_phase_time_ms(mut self, phase_time_ms: u64) -> Self {
        self.phase_time_ms = Some(phase_time_ms);
        self
    }

    /// Attach elapsed time in the current loop.
    pub fn with_loop_time_ms(mut self, loop_time_ms: u64) -> Self {
        self.loop_time_ms = Some(loop_time_ms);
        self
    }

    /// Attach the named lifecycle phase active for this sample.
    pub fn with_lifecycle_phase(mut self, lifecycle_phase: LifecyclePhase) -> Self {
        self.lifecycle_phase = Some(lifecycle_phase);
        self
    }
}

impl Default for SampleContext {
    fn default() -> Self {
        Self::new(0.0)
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
