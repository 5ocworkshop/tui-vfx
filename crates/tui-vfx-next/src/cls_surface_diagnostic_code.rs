// <FILE>crates/tui-vfx-next/src/cls_surface_diagnostic_code.rs</FILE> - <DESC>Stable surface diagnostic codes</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract diagnostic code enum.</CLOG>

/// Stable diagnostic codes for the surface contract.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum SurfaceDiagnosticCode {
    /// A scope matched zero cells. The destination must not be mutated.
    ZeroCellScope,
    /// A source and destination surface pair had incompatible dimensions.
    SurfaceSizeMismatch,
}

// <FILE>crates/tui-vfx-next/src/cls_surface_diagnostic_code.rs</FILE> - <DESC>Stable surface diagnostic codes</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
