// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_element_surface.rs</FILE> - <DESC>Merge one element cell into the destination scene</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Element surface merging centralizes final destination cell and role mutation.</WCTX>
// <CLOG>0.1.1: PATCH — apply skipTransparentEmpty to the final write payload.</CLOG>

use tui_vfx_contract::{CellWritePolicy, RoleWritePolicy};
use tui_vfx_types::{Cell, Grid, SemanticScene};

use crate::render::{CellWriteDecision, apply_cell_write_policy, apply_role_write_policy};

pub(crate) fn merge_element_surface(
    destination: &mut SemanticScene,
    dest_x: usize,
    dest_y: usize,
    _sampled_cell: &Cell,
    final_cell: Cell,
    cell_policy: CellWritePolicy,
    role_policy: &RoleWritePolicy,
) {
    match apply_cell_write_policy(cell_policy, &final_cell) {
        CellWriteDecision::Skip => {}
        CellWriteDecision::Write => {
            destination.grid_mut().set(dest_x, dest_y, final_cell);
            apply_role_write_policy(destination, dest_x, dest_y, role_policy);
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_element_surface.rs</FILE> - <DESC>Merge one element cell into the destination scene</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
