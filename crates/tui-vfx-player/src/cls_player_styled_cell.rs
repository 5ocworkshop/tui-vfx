// <FILE>crates/tui-vfx-player/src/cls_player_styled_cell.rs</FILE> - <DESC>Player-owned styled cell DTO</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Styled-cell substrate work: carry glyph, style, modifier, and role evidence.</WCTX>
// <CLOG>0.1.1: PATCH — centralize default styled-cell field values for clarity.
// 0.1.0: INIT — add internal styled-cell value with explicit defaults.</CLOG>

const DEFAULT_GLYPH: &str = " ";
const DEFAULT_FOREGROUND: &str = "defaultForeground";
const DEFAULT_BACKGROUND: &str = "transparent";

/// Player-owned styled cell used before conversion to sparse visual-frame evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerStyledCell {
    /// Cell x coordinate in terminal columns.
    pub x: usize,
    /// Cell y coordinate in terminal rows.
    pub y: usize,
    /// Rendered glyph at this cell.
    pub glyph: String,
    /// Foreground color label.
    pub foreground: String,
    /// Background color label.
    pub background: String,
    /// Text modifiers known for this cell.
    pub modifiers: Vec<String>,
    /// Semantic role when available.
    pub role: Option<String>,
}

impl PlayerStyledCell {
    /// Build the canonical default styled cell at a coordinate.
    pub fn default_at(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            glyph: DEFAULT_GLYPH.to_string(),
            foreground: DEFAULT_FOREGROUND.to_string(),
            background: DEFAULT_BACKGROUND.to_string(),
            modifiers: vec![],
            role: None,
        }
    }

    /// Return true when no observable field differs from the styled-cell default.
    pub fn is_default(&self) -> bool {
        self.glyph == DEFAULT_GLYPH
            && self.foreground == DEFAULT_FOREGROUND
            && self.background == DEFAULT_BACKGROUND
            && self.modifiers.is_empty()
            && self.role.is_none()
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_styled_cell.rs</FILE> - <DESC>Player-owned styled cell DTO</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
