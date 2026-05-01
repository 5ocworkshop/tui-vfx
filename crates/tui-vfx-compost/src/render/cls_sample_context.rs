// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>SampleContext carries explicit normalized and absolute render-time values without global state.</WCTX>
// <CLOG>0.3.1: PATCH — keep timing derivation in RenderTiming and leave SampleContext as sample data.
// 0.3.0: MINOR — add explicit lifecycle phase sampling for active graph nodes.
// 0.2.0: MINOR — add loop and absolute clocks with explicit timing helpers.
// 0.1.0: INIT — add sample context type.</CLOG>

use tui_vfx_contract::LifecyclePhase;

/// Explicit sample context for native compost rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SampleContext {
    /// Normalized animation phase used by migrated primitives.
    pub phase_t: f64,
    /// Optional normalized loop clock for loop-driven effects.
    pub loop_t: Option<f64>,
    /// Optional absolute sample timestamp in milliseconds.
    pub absolute_time_ms: Option<u64>,
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
            lifecycle_phase: None,
        }
    }

    /// Attach an explicit normalized loop clock.
    pub fn with_loop_t(mut self, loop_t: f64) -> Self {
        self.loop_t = Some(loop_t);
        self
    }

    /// Attach an absolute sample timestamp in milliseconds.
    pub fn with_absolute_time_ms(mut self, absolute_time_ms: u64) -> Self {
        self.absolute_time_ms = Some(absolute_time_ms);
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
// <VERS>END OF VERSION: 0.3.1</VERS>
