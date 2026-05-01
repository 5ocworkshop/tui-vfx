// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_cell_write_policy.rs</FILE> - <DESC>Apply canonical cell write policy before destination mutation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Cell write policy execution adapts mature preserve-unfilled behavior to canonical cellWritePolicy values.</WCTX>
// <CLOG>0.1.0: INIT — support writeCell and skipTransparentEmpty decisions.</CLOG>

use tui_vfx_contract::CellWritePolicy;
use tui_vfx_types::Cell;

use crate::render::CellWriteDecision;

pub(crate) fn apply_cell_write_policy(
    policy: CellWritePolicy,
    sampled_cell: &Cell,
) -> CellWriteDecision {
    match policy {
        CellWritePolicy::WriteCell => CellWriteDecision::Write,
        CellWritePolicy::SkipTransparentEmpty if sampled_cell.is_empty() => CellWriteDecision::Skip,
        CellWritePolicy::SkipTransparentEmpty => CellWriteDecision::Write,
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_cell_write_policy.rs</FILE> - <DESC>Apply canonical cell write policy before destination mutation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
