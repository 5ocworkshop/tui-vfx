// <FILE>crates/tui-vfx-next/src/cls_cell_delta.rs</FILE> - <DESC>Proof-only channel delta for one surface cell</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: identify cell channel writes by producing node.</WCTX>
// <CLOG>0.1.0: INIT — add channel delta record for merge conflict detection.</CLOG>

use crate::{CellChannel, CellChannelWrite, NodeId};

/// One channel write produced by a proof graph node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellDelta {
    /// Destination x coordinate.
    pub x: usize,
    /// Destination y coordinate.
    pub y: usize,
    /// Cell channel written.
    pub channel: CellChannel,
    /// Node that produced this channel value.
    pub node: NodeId,
    /// Channel payload to apply at merge.
    pub write: CellChannelWrite,
}

// <FILE>crates/tui-vfx-next/src/cls_cell_delta.rs</FILE> - <DESC>Proof-only channel delta for one surface cell</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
