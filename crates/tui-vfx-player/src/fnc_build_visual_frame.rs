// <FILE>crates/tui-vfx-player/src/fnc_build_visual_frame.rs</FILE> - <DESC>Build visual-frame entries from player frame reports</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.2 review: expose text-grid provenance in visual evidence.</WCTX>
// <CLOG>0.2.0: PATCH — carry loop_t and explicit substrate/style provenance.
// 0.1.0: INIT — add one-way frame-report to visual-frame adapter.</CLOG>

use crate::{
    PlayerFrameReport, PlayerVisualFrame,
    fnc_collect_unsupported_effect_ids::collect_unsupported_effect_ids,
    fnc_collect_visual_cells::collect_visual_cells,
};

/// Build one visual-frame entry from an existing player frame report.
pub(crate) fn build_visual_frame(report: PlayerFrameReport) -> PlayerVisualFrame {
    let cells = collect_visual_cells(&report.rows);
    let unsupported_effect_ids = collect_unsupported_effect_ids(&report.errors);
    PlayerVisualFrame {
        recipe_path: report.path,
        status: report.status,
        phase: report.phase,
        sample_t: report.phase_t,
        loop_t: report.loop_t,
        absolute_time_ms: 0,
        substrate: "textGrid".to_string(),
        cell_source: "rows".to_string(),
        style_known: false,
        width: report.width,
        height: report.height,
        render_hash: report.render_hash,
        non_empty_cells: report.non_empty_cells,
        rows: report.rows,
        cells,
        unsupported_effect_ids,
        errors: report.errors,
        warnings: report.warnings,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_build_visual_frame.rs</FILE> - <DESC>Build visual-frame entries from player frame reports</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
