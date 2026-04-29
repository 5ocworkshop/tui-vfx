// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_visual_cells.rs</FILE> - <DESC>Collect sparse visual cells from styled grids</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Styled-cell substrate work: preserve real style evidence in visual frames.</WCTX>
// <CLOG>0.1.1: PATCH — name conversion helper by its source cell type.
// 0.1.0: INIT — convert non-default styled cells into report cells.</CLOG>

use crate::{PlayerStyledCell, PlayerStyledGrid, PlayerVisualCell};

/// Convert a styled grid into sparse visual-frame cells.
pub(crate) fn collect_styled_visual_cells(grid: &PlayerStyledGrid) -> Vec<PlayerVisualCell> {
    grid.cells()
        .iter()
        .filter(|cell| !cell.is_default())
        .map(visual_cell_from_styled_cell)
        .collect()
}

fn visual_cell_from_styled_cell(cell: &PlayerStyledCell) -> PlayerVisualCell {
    PlayerVisualCell {
        x: cell.x,
        y: cell.y,
        glyph: cell.glyph.clone(),
        foreground: cell.foreground.clone(),
        background: cell.background.clone(),
        modifiers: cell.modifiers.clone(),
        role: cell.role.clone(),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_visual_cells.rs</FILE> - <DESC>Collect sparse visual cells from styled grids</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
