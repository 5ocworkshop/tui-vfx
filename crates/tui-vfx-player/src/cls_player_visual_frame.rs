// <FILE>crates/tui-vfx-player/src/cls_player_visual_frame.rs</FILE> - <DESC>Visual-frame entry DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.2 review: make text-grid sample provenance explicit.</WCTX>
// <CLOG>0.2.0: PATCH — add loop/provenance/style-known fields for safer schema v1.</CLOG>

use tui_vfx_contract::LifecyclePhase;

use crate::{PlayerError, PlayerStatus, PlayerVisualCell, PlayerWarning};

/// One visual-frame entry derived from the existing player render path.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerVisualFrame {
    /// Recipe file path for this frame when rendered from disk.
    pub recipe_path: Option<String>,
    /// Current player render status.
    pub status: PlayerStatus,
    /// Requested lifecycle phase.
    pub phase: LifecyclePhase,
    /// Requested normalized phase progress.
    pub sample_t: f64,
    /// Optional loop-local normalized progress from the sample request.
    pub loop_t: Option<f64>,
    /// Absolute sample timestamp in milliseconds when known; text-grid samples use zero.
    pub absolute_time_ms: u64,
    /// Visual substrate used to construct this frame; current samples are text-grid backed.
    pub substrate: String,
    /// Source used to derive sparse cells; current samples derive cells from rows.
    pub cell_source: String,
    /// Whether style/color/modifier fields are known real visual style data.
    pub style_known: bool,
    /// Frame width in terminal cells.
    pub width: usize,
    /// Frame height in terminal cells.
    pub height: usize,
    /// Deterministic hash over recipe/sample/output.
    pub render_hash: u64,
    /// Number of non-space cells in rows.
    pub non_empty_cells: usize,
    /// Compact glyph rows, preserving current human-readable text-grid output.
    pub rows: Vec<String>,
    /// Sparse non-default cell evidence derived from rows.
    pub cells: Vec<PlayerVisualCell>,
    /// Distinct unsupported effect ids reported for this frame.
    pub unsupported_effect_ids: Vec<String>,
    /// Hard errors or explicit unsupported adapter diagnostics.
    pub errors: Vec<PlayerError>,
    /// Non-fatal player warnings.
    pub warnings: Vec<PlayerWarning>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_visual_frame.rs</FILE> - <DESC>Visual-frame entry DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
