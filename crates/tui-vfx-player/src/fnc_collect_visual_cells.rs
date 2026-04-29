// <FILE>crates/tui-vfx-player/src/fnc_collect_visual_cells.rs</FILE> - <DESC>Collect sparse cells from text-grid frame rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: derive sparse visual cells from existing text rows.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic non-space row-to-cell extraction.</CLOG>

use crate::PlayerVisualCell;

/// Convert compact text rows into sparse non-space visual cell entries.
pub(crate) fn collect_visual_cells(rows: &[String]) -> Vec<PlayerVisualCell> {
    rows.iter()
        .enumerate()
        .flat_map(|(y, row)| row_cells(y, row))
        .collect()
}

fn row_cells(y: usize, row: &str) -> Vec<PlayerVisualCell> {
    row.chars()
        .enumerate()
        .filter(|(_, glyph)| *glyph != ' ')
        .map(|(x, glyph)| visual_cell(x, y, glyph))
        .collect()
}

fn visual_cell(x: usize, y: usize, glyph: char) -> PlayerVisualCell {
    PlayerVisualCell {
        x,
        y,
        glyph: glyph.to_string(),
        foreground: "transparent".to_string(),
        background: "transparent".to_string(),
        modifiers: vec![],
        role: None,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_visual_cells.rs</FILE> - <DESC>Collect sparse cells from text-grid frame rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
