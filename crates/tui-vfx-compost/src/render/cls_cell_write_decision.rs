// <FILE>crates/tui-vfx-compost/src/render/cls_cell_write_decision.rs</FILE> - <DESC>Cell write decision for native compost surface merging</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Cell write decisions separate policy decisions from destination mutation.</WCTX>
// <CLOG>0.1.0: INIT — add write/skip decision for cell policy handling.</CLOG>

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CellWriteDecision {
    Write,
    Skip,
}

// <FILE>crates/tui-vfx-compost/src/render/cls_cell_write_decision.rs</FILE> - <DESC>Cell write decision for native compost surface merging</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
