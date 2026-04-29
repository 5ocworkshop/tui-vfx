// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_checkers.rs</FILE> - <DESC>Apply text-grid checkers mask</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add field-aware checkers mask evidence.</WCTX>
// <CLOG>0.1.0: INIT — add cell-size aware checkers mask.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{PlayerSampleRequest, fnc_resolve_effect_input::resolve_effect_integer};

/// Apply a checkers mask to text-grid rows.
pub(crate) fn apply_mask_checkers(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) {
    if request.phase_t >= 1.0 {
        return;
    }
    let cell_size = resolve_effect_integer(node, request, "cellSize", 1).max(1) as usize;
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, ch)| {
                if ((x / cell_size) + (y / cell_size)).is_multiple_of(2) {
                    ch
                } else {
                    ' '
                }
            })
            .collect();
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_mask_checkers.rs</FILE> - <DESC>Apply text-grid checkers mask</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
