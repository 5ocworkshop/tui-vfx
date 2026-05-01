// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_sample_context.rs</FILE> - <DESC>Direct v3.1 render sample context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep sample-time input separate from render orchestration.</WCTX>
// <CLOG>0.1.0: INIT — extract V31SampleContext.</CLOG>

/// Explicit sample context for direct v3.1 compositor-next rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V31SampleContext {
    /// Normalized lifecycle/sample progress for phase-driven effects.
    pub phase_t: f64,
}

impl Default for V31SampleContext {
    fn default() -> Self {
        Self { phase_t: 0.0 }
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_sample_context.rs</FILE> - <DESC>Direct v3.1 render sample context</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
