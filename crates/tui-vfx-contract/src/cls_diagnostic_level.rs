// <FILE>crates/tui-vfx-contract/src/cls_diagnostic_level.rs</FILE> - <DESC>Surface diagnostic severity enum</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract diagnostic level enum.</CLOG>

/// Severity of a surface diagnostic.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticLevel {
    /// Informational note that does not indicate an invalid operation.
    Info,
    /// Warning that records a suspicious but recoverable operation.
    Warning,
    /// Error that records an invalid contract operation.
    Error,
}

// <FILE>crates/tui-vfx-contract/src/cls_diagnostic_level.rs</FILE> - <DESC>Surface diagnostic severity enum</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
