// <FILE>crates/tui-vfx-player/src/cls_player_frame.rs</FILE> - <DESC>Sampled skeleton player frame DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player frame work: carry deterministic text-grid frame output.</WCTX>
// <CLOG>0.2.0: MINOR — carry non-serialized styled-grid evidence for visual-frame reports.
// 0.1.0: INIT — add rows, dimensions, hash, and non-empty count.</CLOG>

use crate::PlayerStyledGrid;

/// Minimal semantic text-grid frame produced by the contract-native skeleton player.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrame {
    /// Frame width in terminal cells.
    pub width: usize,
    /// Frame height in terminal cells.
    pub height: usize,
    /// Deterministic hash over recipe/sample/adapter-visible output.
    pub render_hash: u64,
    /// Number of non-space cells in the sampled rows.
    pub non_empty_cells: usize,
    /// Text rows clipped/padded to the frame dimensions.
    pub rows: Vec<String>,
    /// Styled-cell evidence for visual-frame output when a styled adapter wrote style data.
    #[serde(skip)]
    pub styled_grid: Option<PlayerStyledGrid>,
}

impl PlayerFrame {
    /// Count non-empty cells in frame rows.
    pub fn count_non_empty(rows: &[String]) -> usize {
        rows.iter()
            .flat_map(|row| row.chars())
            .filter(|ch| *ch != ' ')
            .count()
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_frame.rs</FILE> - <DESC>Sampled skeleton player frame DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
