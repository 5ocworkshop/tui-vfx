// <FILE>crates/tui-vfx-next/src/cls_surface_delta.rs</FILE> - <DESC>Proof-only channel-aware surface delta</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: accumulate branch deltas before parallel merge.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic last-writer delta map.</CLOG>

use std::collections::BTreeMap;

use crate::{CellChannel, CellDelta, NodeId};

/// Channel-aware proof delta produced by a graph step.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SurfaceDelta {
    writes: BTreeMap<(usize, usize, CellChannel), CellDelta>,
}

impl SurfaceDelta {
    /// Create an empty delta.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace one channel write.
    pub fn set(&mut self, delta: CellDelta) {
        self.writes.insert((delta.x, delta.y, delta.channel), delta);
    }

    /// Overlay another delta using deterministic last-writer semantics.
    pub fn overlay(&mut self, other: SurfaceDelta) {
        for delta in other.into_writes() {
            self.set(delta);
        }
    }

    /// Return the previous writer for one cell channel.
    pub fn writer(&self, x: usize, y: usize, channel: CellChannel) -> Option<&NodeId> {
        self.writes.get(&(x, y, channel)).map(|delta| &delta.node)
    }

    /// Iterate channel writes in deterministic coordinate/channel order.
    pub fn writes(&self) -> impl Iterator<Item = &CellDelta> {
        self.writes.values()
    }

    /// Consume this delta into deterministic channel writes.
    pub fn into_writes(self) -> impl Iterator<Item = CellDelta> {
        self.writes.into_values()
    }
}

// <FILE>crates/tui-vfx-next/src/cls_surface_delta.rs</FILE> - <DESC>Proof-only channel-aware surface delta</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
