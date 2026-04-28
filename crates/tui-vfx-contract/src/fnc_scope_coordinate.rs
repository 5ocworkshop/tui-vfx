// <FILE>crates/tui-vfx-contract/src/fnc_scope_coordinate.rs</FILE> - <DESC>Select scope coordinate from evaluation input</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase C preflight OFPF split.</WCTX>
// <CLOG>0.3.0: REFACTOR — extract coordinate selection helper.</CLOG>

use crate::{CoordinateSpace, ScopeEvalInput};

/// Select the coordinate pair for the requested scope coordinate space.
pub(crate) fn scope_coordinate(
    input: &ScopeEvalInput,
    coordinate_space: CoordinateSpace,
) -> (usize, usize) {
    match coordinate_space {
        CoordinateSpace::DestinationLocal => (input.destination_x, input.destination_y),
        CoordinateSpace::SampledSource => (input.sampled_source_x, input.sampled_source_y),
    }
}

// <FILE>crates/tui-vfx-contract/src/fnc_scope_coordinate.rs</FILE> - <DESC>Select scope coordinate from evaluation input</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
