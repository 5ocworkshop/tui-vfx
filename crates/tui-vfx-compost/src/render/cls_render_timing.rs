// <FILE>crates/tui-vfx-compost/src/render/cls_render_timing.rs</FILE> - <DESC>Resolved native render timing bundle</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Render timing centralizes normalized phase and loop timing for shader sampling.</WCTX>
// <CLOG>0.3.1: PATCH — own timing derivation tests after SampleContext returned to data-only shape.
// 0.3.0: PATCH — leave lifecycle gating on SampleContext and keep RenderTiming focused on numeric progress.
// 0.2.0: MINOR — carry lifecycle phase into resolved render timing.
// 0.1.1: PATCH — drop unused absolute-time storage from normalized render timing.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_time_overrides_phase_time_and_clamps() {
        let sample = SampleContext::new(1.25).with_loop_t(-0.25);
        let timing = RenderTiming::from_sample(&sample);

        assert_eq!(timing.effective_loop_t(), 0.0);
        assert_eq!(timing.shader_phase_t(), 0.0);
    }

    #[test]
    fn phase_time_drives_shader_sampling_when_loop_time_is_absent() {
        let sample = SampleContext::new(0.75);
        let timing = RenderTiming::from_sample(&sample);

        assert_eq!(timing.effective_loop_t(), 0.75);
        assert_eq!(timing.shader_phase_t(), 0.75);
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_timing.rs</FILE> - <DESC>Resolved native render timing bundle</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
