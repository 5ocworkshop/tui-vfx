// <FILE>crates/tui-vfx-contract/src/cls_apply_outcome.rs</FILE> - <DESC>Surface operation outcome DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract ApplyOutcome from engine module.</CLOG>

use crate::SurfaceDiagnostic;

/// Result of one surface operation.
#[derive(
    Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApplyOutcome {
    /// Number of destination cells matched by scope and considered for writing.
    pub matched_cells: usize,
    /// Number of destination cells actually written after cell policy.
    pub written_cells: usize,
    /// Structured diagnostics emitted by the operation.
    pub diagnostics: Vec<SurfaceDiagnostic>,
}

// <FILE>crates/tui-vfx-contract/src/cls_apply_outcome.rs</FILE> - <DESC>Surface operation outcome DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
