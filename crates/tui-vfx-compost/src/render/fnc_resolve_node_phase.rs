// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_node_phase.rs</FILE> - <DESC>Resolve native timing for one graph node</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Node execution routes through one timing seam before lifecycle-specific clocks are supported.</WCTX>
// <CLOG>0.1.0: INIT — derive per-node render timing from SampleContext.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::render::{RenderTiming, SampleContext, is_node_active};

pub(crate) fn resolve_node_phase(sample: &SampleContext, node: &NodeSpec) -> RenderTiming {
    debug_assert!(
        is_node_active(node),
        "LoadedRecipe rejects activePhases before render"
    );
    RenderTiming::from_sample(sample)
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_node_phase.rs</FILE> - <DESC>Resolve native timing for one graph node</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
