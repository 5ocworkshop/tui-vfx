// <FILE>crates/tui-vfx-player/src/cls_player_frame_timeline_report.rs</FILE> - <DESC>Frame timeline report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: expose deterministic multi-sample frame timelines.</WCTX>
// <CLOG>0.1.0: INIT — add frame timeline report schema.</CLOG>

use crate::{DescriptorPackReport, PlayerVisualFrame};

/// Deterministic timeline report for one recipe sampled across normalized time.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrameTimelineReport {
    /// Stable frame timeline schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Timeline frame evidence.
    pub frames: Vec<PlayerVisualFrame>,
}

impl PlayerFrameTimelineReport {
    /// Build a frame timeline report.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        frames: Vec<PlayerVisualFrame>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.frameTimeline.1",
            root,
            descriptor_packs,
            frames,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_frame_timeline_report.rs</FILE> - <DESC>Frame timeline report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
