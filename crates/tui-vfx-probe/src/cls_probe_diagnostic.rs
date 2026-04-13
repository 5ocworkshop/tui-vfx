// <FILE>crates/tui-vfx-probe/src/cls_probe_diagnostic.rs</FILE> - <DESC>Diagnostic DTOs for probe quality checks</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>NEW: Add typed diagnostics so probe consumers can reason about border contamination and underline placement without ad hoc SQL or regexes</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeDiagnosticSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeDiagnostic {
    pub code: String,
    pub severity: ProbeDiagnosticSeverity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_y: Option<u16>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_diagnostic.rs</FILE> - <DESC>Diagnostic DTOs for probe quality checks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
