// <FILE>crates/tui-vfx-player/src/cls_player_frame_diff_report.rs</FILE> - <DESC>Frame diff report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: expose deterministic cell-level frame diffs.</WCTX>
// <CLOG>0.1.0: INIT — add frame diff report schema.</CLOG>

use crate::{DescriptorPackReport, PlayerVisualFrame};

/// One changed cell in a frame diff.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrameDiffCell {
    /// Cell column.
    pub x: usize,
    /// Cell row.
    pub y: usize,
    /// Glyph in the from frame.
    pub from: String,
    /// Glyph in the to frame.
    pub to: String,
}

/// Deterministic diff between two sampled frames.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrameDiffReport {
    /// Stable frame diff schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// From-frame evidence.
    #[serde(rename = "from")]
    pub from_frame: PlayerVisualFrame,
    /// To-frame evidence.
    #[serde(rename = "to")]
    pub to_frame: PlayerVisualFrame,
    /// Whether render hashes differ.
    pub hash_changed: bool,
    /// Number of changed cells.
    pub changed_cell_count: usize,
    /// Difference in non-empty cell counts.
    pub non_empty_delta: isize,
    /// Sparse changed cell evidence.
    pub changed_cells: Vec<PlayerFrameDiffCell>,
}

impl PlayerFrameDiffReport {
    /// Build a frame diff report.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        from_frame: PlayerVisualFrame,
        to_frame: PlayerVisualFrame,
        changed_cells: Vec<PlayerFrameDiffCell>,
    ) -> Self {
        let hash_changed = from_frame.render_hash != to_frame.render_hash;
        let non_empty_delta =
            to_frame.non_empty_cells as isize - from_frame.non_empty_cells as isize;
        Self {
            schema_version: "v3.1.player.frameDiff.1",
            root,
            descriptor_packs,
            from_frame,
            to_frame,
            hash_changed,
            changed_cell_count: changed_cells.len(),
            non_empty_delta,
            changed_cells,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_frame_diff_report.rs</FILE> - <DESC>Frame diff report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
