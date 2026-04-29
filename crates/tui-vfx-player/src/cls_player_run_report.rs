// <FILE>crates/tui-vfx-player/src/cls_player_run_report.rs</FILE> - <DESC>Recursive player run report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: report recursive render-recipe smoke results.</WCTX>
// <CLOG>0.1.0: INIT — add v3.1.player.run.1 report shape.</CLOG>

use crate::{PlayerFrameReport, PlayerStatus, PlayerSummary};

/// Stable machine-readable report for a recursive player smoke run.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRunReport {
    /// Stable run report schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Aggregate render status counts.
    pub summary: PlayerSummary,
    /// Per-recipe frame reports.
    pub frames: Vec<PlayerFrameReport>,
}

impl PlayerRunReport {
    /// Build a run report from already-rendered frame reports.
    pub fn new(root: String, frames: Vec<PlayerFrameReport>) -> Self {
        let mut summary = PlayerSummary {
            total: frames.len(),
            ..PlayerSummary::default()
        };
        for frame in &frames {
            match frame.status {
                PlayerStatus::Rendered => summary.rendered += 1,
                PlayerStatus::Unsupported => summary.unsupported += 1,
                PlayerStatus::Error => summary.errors += 1,
            }
        }
        Self {
            schema_version: "v3.1.player.run.1",
            root,
            summary,
            frames,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_run_report.rs</FILE> - <DESC>Recursive player run report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
