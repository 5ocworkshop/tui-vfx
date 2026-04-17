// <FILE>crates/tui-vfx-probe/src/fnc_collect_basic_diagnostics.rs</FILE> - <DESC>Collect basic border and underline diagnostics from a probe report</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>NEW: Add a probe-level diagnostics pass that detects alphabetic text leaking onto border rows and underline glyphs contaminating the bottom border</CLOG>

use crate::ProbeReport;
use crate::cls_probe_diagnostic::{ProbeDiagnostic, ProbeDiagnosticSeverity};
use crate::fnc_has_ascii_alpha::has_ascii_alpha;
use crate::fnc_max_widget_y::max_widget_y;
use crate::fnc_row_text::row_text;

pub fn collect_basic_diagnostics(report: &ProbeReport) -> Vec<ProbeDiagnostic> {
    let mut diagnostics = Vec::new();
    let top = row_text(report, 0);
    let bottom_y = max_widget_y(report);
    let bottom = row_text(report, bottom_y);

    if has_ascii_alpha(&top) {
        diagnostics.push(ProbeDiagnostic {
            code: "alpha_on_top_border".to_string(),
            severity: ProbeDiagnosticSeverity::Error,
            message: "Detected alphabetic characters on the top border row".to_string(),
            widget_y: Some(0),
        });
    }

    if has_ascii_alpha(&bottom) {
        diagnostics.push(ProbeDiagnostic {
            code: "alpha_on_bottom_border".to_string(),
            severity: ProbeDiagnosticSeverity::Error,
            message: "Detected alphabetic characters on the bottom border row".to_string(),
            widget_y: Some(bottom_y),
        });
    }

    if bottom.contains('▁') {
        diagnostics.push(ProbeDiagnostic {
            code: "underline_on_bottom_border".to_string(),
            severity: ProbeDiagnosticSeverity::Error,
            message: "Detected underline glyphs on the bottom border row".to_string(),
            widget_y: Some(bottom_y),
        });
    }

    diagnostics
}

// <FILE>crates/tui-vfx-probe/src/fnc_collect_basic_diagnostics.rs</FILE> - <DESC>Collect basic border and underline diagnostics from a probe report</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
