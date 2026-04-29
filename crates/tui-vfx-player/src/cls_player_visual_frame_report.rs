// <FILE>crates/tui-vfx-player/src/cls_player_visual_frame_report.rs</FILE> - <DESC>Visual-frame report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: add stable visual-frame evidence report schema.</WCTX>
// <CLOG>0.1.0: INIT — add v3.1.player.visualFrameReport.1 aggregate shape.</CLOG>

use crate::{DescriptorPackReport, PlayerSummary, PlayerVisualFrame};

/// Stable machine-readable visual-frame report for one player invocation.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerVisualFrameReport {
    /// Stable visual-frame report schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this visual-frame invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate render status counts.
    pub summary: PlayerSummary,
    /// Per-recipe visual frame entries.
    pub frames: Vec<PlayerVisualFrame>,
}

impl PlayerVisualFrameReport {
    /// Build a visual-frame report from already-rendered visual frame entries.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        summary: PlayerSummary,
        frames: Vec<PlayerVisualFrame>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.visualFrameReport.1",
            root,
            descriptor_packs,
            summary,
            frames,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_visual_frame_report.rs</FILE> - <DESC>Visual-frame report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
