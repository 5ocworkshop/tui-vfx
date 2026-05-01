// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>SampleContext carries explicit render-time values without global state.</WCTX>
// <CLOG>0.1.0: INIT — add sample context type.</CLOG>

/// Explicit sample context for native compost rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SampleContext {
    /// Normalized animation phase used by migrated primitives.
    pub phase_t: f64,
}

impl Default for SampleContext {
    fn default() -> Self {
        Self { phase_t: 0.0 }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_sample_context.rs</FILE> - <DESC>Native render sample context</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
