// <FILE>crates/tui-vfx-player/src/fnc_build_visual_frame.rs</FILE> - <DESC>Build visual-frame entries from player frame reports</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Styled-cell substrate work: build visual frames from player-owned styled grids.</WCTX>
// <CLOG>0.3.1: PATCH — clarify default visual-frame substrate construction.
// 0.3.0: MINOR — emit styled-cell substrate metadata and sparse styled cells.
// 0.2.0: PATCH — carry loop_t and explicit substrate/style provenance.
// 0.1.0: INIT — add one-way frame-report to visual-frame adapter.</CLOG>

use crate::{
    PlayerFrameReport, PlayerStyledGrid, PlayerVisualFrame,
    fnc_collect_styled_visual_cells::collect_styled_visual_cells,
    fnc_collect_unsupported_effect_ids::collect_unsupported_effect_ids,
};

/// Build one visual-frame entry from a player frame report using styled-cell defaults.
pub(crate) fn build_visual_frame(report: PlayerFrameReport) -> PlayerVisualFrame {
    let styled_grid = PlayerStyledGrid::from_rows(&report.rows);
    build_visual_frame_from_styled_grid(report, styled_grid)
}

/// Build a visual-frame entry from explicit styled-cell evidence.
pub fn build_visual_frame_from_styled_grid(
    report: PlayerFrameReport,
    styled_grid: PlayerStyledGrid,
) -> PlayerVisualFrame {
    let cells = collect_styled_visual_cells(&styled_grid);
    let unsupported_effect_ids = collect_unsupported_effect_ids(&report.errors);
    let (substrate, cell_source) = visual_frame_provenance(&styled_grid);
    PlayerVisualFrame {
        recipe_path: report.path,
        status: report.status,
        phase: report.phase,
        sample_t: report.phase_t,
        loop_t: report.loop_t,
        absolute_time_ms: 0,
        substrate: substrate.to_string(),
        cell_source: cell_source.to_string(),
        style_known: styled_grid.style_known(),
        width: styled_grid.width(),
        height: styled_grid.height(),
        render_hash: report.render_hash,
        non_empty_cells: report.non_empty_cells,
        rows: report.rows,
        cells,
        unsupported_effect_ids,
        errors: report.errors,
        warnings: report.warnings,
    }
}

fn visual_frame_provenance(styled_grid: &PlayerStyledGrid) -> (&'static str, &'static str) {
    if styled_grid.style_known() {
        ("styledCell", "styledCells")
    } else {
        ("textGrid", "rows")
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_visual_frame.rs</FILE> - <DESC>Build visual-frame entries from player frame reports</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
