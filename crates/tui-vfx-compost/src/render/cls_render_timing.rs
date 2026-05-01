// <FILE>crates/tui-vfx-compost/src/render/cls_render_timing.rs</FILE> - <DESC>Resolved native render timing bundle</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Render timing centralizes normalized phase and loop timing before loop-dependent primitives migrate.</WCTX>
// <CLOG>0.1.1: PATCH — drop unused absolute-time storage from normalized render timing.
// 0.1.0: INIT — add clamped render timing derived from SampleContext.</CLOG>

use crate::render::SampleContext;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RenderTiming {
    phase_t: f64,
    loop_t: Option<f64>,
}

impl RenderTiming {
    pub(crate) fn from_sample(sample: &SampleContext) -> Self {
        Self {
            phase_t: sample.phase_t.clamp(0.0, 1.0),
            loop_t: sample.loop_t.map(|loop_t| loop_t.clamp(0.0, 1.0)),
        }
    }

    pub(crate) fn effective_loop_t(&self) -> f64 {
        self.loop_t.unwrap_or(self.phase_t)
    }

    pub(crate) fn shader_phase_t(&self) -> f64 {
        self.effective_loop_t().clamp(0.0, 1.0)
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_timing.rs</FILE> - <DESC>Resolved native render timing bundle</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
