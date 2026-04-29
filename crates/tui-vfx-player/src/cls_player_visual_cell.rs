// <FILE>crates/tui-vfx-player/src/cls_player_visual_cell.rs</FILE> - <DESC>Sparse visual-frame cell DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: expose stable sparse cell evidence from text-grid rows.</WCTX>
// <CLOG>0.1.0: INIT — add JSON-stable sparse visual cell shape.</CLOG>

/// Sparse non-default cell in a visual frame report.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerVisualCell {
    /// Cell x coordinate in terminal columns.
    pub x: usize,
    /// Cell y coordinate in terminal rows.
    pub y: usize,
    /// Rendered glyph at this cell.
    pub glyph: String,
    /// Foreground color label; row-derived frames use transparent.
    pub foreground: String,
    /// Background color label; row-derived frames use transparent.
    pub background: String,
    /// Text modifiers known for this cell.
    pub modifiers: Vec<String>,
    /// Semantic role when available from a future visual surface.
    pub role: Option<String>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_visual_cell.rs</FILE> - <DESC>Sparse visual-frame cell DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
