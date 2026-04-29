// <FILE>crates/tui-vfx-player/src/cls_player_frame_report.rs</FILE> - <DESC>Stable JSON player frame report DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player reporting work: expose machine-readable sampled frame evidence.</WCTX>
// <CLOG>0.2.0: MINOR — carry non-serialized styled-grid evidence into visual-frame reports.
// 0.1.0: INIT — add v3.1.player.frame.1 report shape.</CLOG>

use tui_vfx_contract::LifecyclePhase;

use crate::{PlayerError, PlayerFrame, PlayerStatus, PlayerStyledGrid, PlayerWarning};

/// Stable machine-readable report for one sampled player frame.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrameReport {
    /// Stable frame report schema label.
    pub schema_version: &'static str,
    /// Canonical recipe id.
    pub recipe_id: String,
    /// Optional recipe file path when the CLI rendered from disk.
    pub path: Option<String>,
    /// Render status.
    pub status: PlayerStatus,
    /// Requested lifecycle phase.
    pub phase: LifecyclePhase,
    /// Requested normalized phase progress.
    pub phase_t: f64,
    /// Optional requested loop progress.
    pub loop_t: Option<f64>,
    /// Frame width in terminal cells.
    pub width: usize,
    /// Frame height in terminal cells.
    pub height: usize,
    /// Deterministic hash over recipe/sample/output.
    pub render_hash: u64,
    /// Number of non-space cells in rows.
    pub non_empty_cells: usize,
    /// Text rows for compact smoke-render inspection.
    pub rows: Vec<String>,
    /// Styled-cell evidence for downstream visual-frame output.
    #[serde(skip)]
    pub styled_grid: Option<PlayerStyledGrid>,
    /// True when a trigger-terminated dwell policy has fired in this session.
    pub dwell_terminated: bool,
    /// Hard errors or explicit unsupported adapter diagnostics.
    pub errors: Vec<PlayerError>,
    /// Non-fatal player warnings.
    pub warnings: Vec<PlayerWarning>,
}

impl PlayerFrameReport {
    /// Build a frame report from a frame and diagnostic lists.
    pub fn from_frame(
        recipe_id: String,
        frame: PlayerFrame,
        status: PlayerStatus,
        request: &crate::PlayerSampleRequest,
        dwell_terminated: bool,
        errors: Vec<PlayerError>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.frame.1",
            recipe_id,
            path: None,
            status,
            phase: request.phase,
            phase_t: request.phase_t,
            loop_t: request.loop_t,
            width: frame.width,
            height: frame.height,
            render_hash: frame.render_hash,
            non_empty_cells: frame.non_empty_cells,
            styled_grid: frame.styled_grid,
            rows: frame.rows,
            dwell_terminated,
            errors,
            warnings: vec![],
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_frame_report.rs</FILE> - <DESC>Stable JSON player frame report DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
