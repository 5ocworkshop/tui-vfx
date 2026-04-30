// <FILE>crates/tui-vfx-player/src/fnc_player_render_backend.rs</FILE> - <DESC>Player render backend seam implementations</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render backend seam: provide deterministic player-core backends without compositor or UI imports.</WCTX>
// <CLOG>0.1.0: INIT — add text-grid and styled-cell backends over PlayerRenderIrReport.</CLOG>

use crate::{
    PlayerRenderBackendOutput, PlayerRenderCell, PlayerRenderIrReport,
    cls_player_render_backend_output::PlayerRenderBackendDiagnostic,
};

/// Internal seam for deterministic player-owned render backends.
pub trait PlayerRenderBackend {
    /// Render backend output from a player render IR report.
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput;
}

/// Backend that forwards compact text-grid rows from render IR.
#[derive(Clone, Copy, Debug, Default)]
pub struct TextGridRenderBackend;

impl PlayerRenderBackend for TextGridRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput {
        PlayerRenderBackendOutput::new("textGrid", input.rows.clone(), vec![], vec![])
    }
}

/// Backend that forwards deterministic sparse styled cells from render IR.
#[derive(Clone, Copy, Debug, Default)]
pub struct StyledCellRenderBackend;

impl PlayerRenderBackend for StyledCellRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput {
        let mut styled_cells = input.styled_cells.clone();
        styled_cells.sort_by_key(styled_cell_sort_key);
        let diagnostics = if styled_cells.is_empty() {
            vec![PlayerRenderBackendDiagnostic {
                code: "styledCellsUnavailable".to_string(),
                path: "styledCells".to_string(),
                message: "Render IR did not contain non-default styled cells; rows were preserved."
                    .to_string(),
            }]
        } else {
            vec![]
        };
        PlayerRenderBackendOutput::new("styledCell", input.rows.clone(), styled_cells, diagnostics)
    }
}

fn styled_cell_sort_key(cell: &PlayerRenderCell) -> (usize, usize, String, String, String, String) {
    (
        cell.y,
        cell.x,
        cell.glyph.clone(),
        cell.foreground.clone(),
        cell.background.clone(),
        cell.role.clone().unwrap_or_default(),
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_player_render_backend.rs</FILE> - <DESC>Player render backend seam implementations</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
