// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_shader_phase_t.rs</FILE> - <DESC>Resolve shader phase time for effect sampling</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Shader sampling derives normalized phase time from the explicit sample context.</WCTX>
// <CLOG>0.1.0: INIT — derive shader phase time from SampleContext through RenderTiming.</CLOG>

use crate::render::{RenderTiming, SampleContext};

pub(crate) fn resolve_shader_phase_t(sample: &SampleContext) -> f64 {
    RenderTiming::from_sample(sample).shader_phase_t()
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_shader_phase_t.rs</FILE> - <DESC>Resolve shader phase time for effect sampling</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
