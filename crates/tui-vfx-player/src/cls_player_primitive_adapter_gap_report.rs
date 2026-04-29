// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_report.rs</FILE> - <DESC>Primitive adapter gap report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: add focused adapter classification report.</WCTX>
// <CLOG>0.1.0: INIT — add v3.1.player.primitiveAdapterGap.1 aggregate shape.</CLOG>

use crate::{
    DescriptorPackReport, PlayerPrimitiveAdapterGapEntry, PlayerPrimitiveAdapterGapSummary,
};

/// Stable machine-readable primitive adapter gap report.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveAdapterGapReport {
    /// Stable primitive adapter gap report schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate adapter outcome counts.
    pub summary: PlayerPrimitiveAdapterGapSummary,
    /// Represented effect classifications.
    pub effects: Vec<PlayerPrimitiveAdapterGapEntry>,
}

impl PlayerPrimitiveAdapterGapReport {
    /// Build a primitive adapter gap report from classified effect entries.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        summary: PlayerPrimitiveAdapterGapSummary,
        effects: Vec<PlayerPrimitiveAdapterGapEntry>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.primitiveAdapterGap.1",
            root,
            descriptor_packs,
            summary,
            effects,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_report.rs</FILE> - <DESC>Primitive adapter gap report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
