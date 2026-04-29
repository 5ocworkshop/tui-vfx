// <FILE>crates/tui-vfx-player/src/cls_player_styled_grid.rs</FILE> - <DESC>Player-owned styled grid DTO</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Styled-cell substrate work: carry sparse visual evidence before report conversion.</WCTX>
// <CLOG>0.3.0: MINOR — add graph-adapter mutation helpers for glyph sync and styled evidence.
// 0.2.0: PATCH — keep row-derived grids style-unknown until real style is written.
// 0.1.1: PATCH — clarify row-to-grid local naming.
// 0.1.0: INIT — add styled-grid construction from rows plus style mutation seam.</CLOG>

use crate::PlayerStyledCell;

/// Dense styled-cell grid used as the player-owned visual substrate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerStyledGrid {
    width: usize,
    height: usize,
    style_known: bool,
    cells: Vec<PlayerStyledCell>,
}

impl PlayerStyledGrid {
    /// Build a styled grid from compact text rows without claiming style evidence.
    pub fn from_rows(rows: &[String]) -> Self {
        let width = rows
            .iter()
            .map(|row| row.chars().count())
            .max()
            .unwrap_or(0);
        let height = rows.len();
        let mut styled_grid = Self::blank(width, height, false);
        for (y, row) in rows.iter().enumerate() {
            for (x, glyph) in row.chars().enumerate() {
                if let Some(cell) = styled_grid.cell_mut(x, y) {
                    cell.glyph = glyph.to_string();
                }
            }
        }
        styled_grid
    }

    /// Build a blank grid with explicit style-knowledge provenance.
    pub fn blank(width: usize, height: usize, style_known: bool) -> Self {
        let cells = (0..height)
            .flat_map(|y| (0..width).map(move |x| PlayerStyledCell::default_at(x, y)))
            .collect();
        Self {
            width,
            height,
            style_known,
            cells,
        }
    }

    /// Set style evidence for one cell when it exists.
    pub fn set_cell_style(
        &mut self,
        x: usize,
        y: usize,
        foreground: &str,
        background: &str,
        modifiers: Vec<String>,
        role: Option<String>,
    ) {
        if let Some(cell) = self.cell_mut(x, y) {
            cell.foreground = foreground.to_string();
            cell.background = background.to_string();
            cell.modifiers = modifiers;
            cell.role = role;
            self.style_known = true;
        }
    }

    /// Sync glyph evidence after a text-grid adapter mutates compact rows.
    pub fn sync_glyphs_from_rows(&mut self, rows: &[String]) {
        for (y, row) in rows.iter().enumerate() {
            for (x, glyph) in row.chars().enumerate() {
                if let Some(cell) = self.cell_mut(x, y) {
                    cell.glyph = glyph.to_string();
                }
            }
        }
    }

    /// Frame width in terminal cells.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Frame height in terminal cells.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Whether style fields in this grid are known real evidence rather than row defaults.
    pub fn style_known(&self) -> bool {
        self.style_known
    }

    /// Dense cell storage in row-major order.
    pub fn cells(&self) -> &[PlayerStyledCell] {
        &self.cells
    }

    /// Whether a coordinate is inside this grid.
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut PlayerStyledCell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get_mut(y * self.width + x)
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_styled_grid.rs</FILE> - <DESC>Player-owned styled grid DTO</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
