// <FILE>crates/tui-vfx-probe/src/fnc_row_text.rs</FILE> - <DESC>Build row text from a probe report for a specific widget-local Y</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>MINOR: Reconstruct rows against widget width so diagnostics preserve horizontal gaps instead of collapsing filtered or sparse cell sets</CLOG>

use crate::ProbeReport;

pub fn row_text(report: &ProbeReport, widget_y: u16) -> String {
    let width = report.widget.size.width as usize;
    if width == 0 {
        return String::new();
    }

    let mut row = vec![' '; width];
    for cell in report
        .cells
        .iter()
        .filter(|cell| cell.widget_local.y == widget_y)
    {
        let x = cell.widget_local.x as usize;
        if x < width {
            row[x] = cell.ch;
        }
    }

    row.into_iter().collect()
}

// <FILE>crates/tui-vfx-probe/src/fnc_row_text.rs</FILE> - <DESC>Build row text from a probe report for a specific widget-local Y</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
