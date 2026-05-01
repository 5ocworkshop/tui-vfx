// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>SampleContext carries explicit normalized and absolute render-time values without global state.</WCTX>
// <CLOG>0.2.0: MINOR — add loop and absolute clocks with explicit timing helpers.
// 0.1.0: INIT — add sample context type.</CLOG>

/// Explicit sample context for native compost rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SampleContext {
    /// Normalized animation phase used by migrated primitives.
    pub phase_t: f64,
    /// Optional normalized loop clock for loop-driven effects.
    pub loop_t: Option<f64>,
    /// Optional absolute sample timestamp in milliseconds.
    pub absolute_time_ms: Option<u64>,
}

impl SampleContext {
    /// Build a sample context with explicit normalized phase progress.
    pub fn new(phase_t: f64) -> Self {
        Self {
            phase_t,
            loop_t: None,
            absolute_time_ms: None,
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

    /// Return the loop clock used by loop-driven primitives.
    pub fn effective_loop_t(&self) -> f64 {
        self.loop_t.unwrap_or(self.phase_t).clamp(0.0, 1.0)
    }

    /// Return the normalized progress used by spatial shader execution.
    pub fn shader_phase_t(&self) -> f64 {
        self.effective_loop_t()
    }
}

impl Default for SampleContext {
    fn default() -> Self {
        Self::new(0.0)
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
