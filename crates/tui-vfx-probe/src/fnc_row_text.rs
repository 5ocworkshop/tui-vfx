// <FILE>crates/tui-vfx-probe/src/fnc_row_text.rs</FILE> - <DESC>Build row text from a probe report for a specific widget-local Y</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>NEW: Add a helper that reconstructs row text from structured cell output so diagnostics can evaluate text and border integrity directly</CLOG>

use crate::ProbeReport;

pub fn row_text(report: &ProbeReport, widget_y: u16) -> String {
    let mut cells = report
        .cells
        .iter()
        .filter(|cell| cell.widget_local.y == widget_y)
        .collect::<Vec<_>>();
    cells.sort_by_key(|cell| cell.widget_local.x);
    cells.iter().map(|cell| cell.ch).collect()
}

// <FILE>crates/tui-vfx-probe/src/fnc_row_text.rs</FILE> - <DESC>Build row text from a probe report for a specific widget-local Y</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
