// <FILE>crates/tui-vfx-contract/src/cls_surface_diagnostic_code.rs</FILE> - <DESC>Stable surface diagnostic codes</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase D1: add element-aware scene clipping diagnostics.</WCTX>
// <CLOG>0.5.0: MINOR — add scene element clipping diagnostic code for D1 composition.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
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
    /// A scene element had local cells clipped by the final scene bounds.
    SceneElementClipped,
}

// <FILE>crates/tui-vfx-contract/src/cls_surface_diagnostic_code.rs</FILE> - <DESC>Stable surface diagnostic codes</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
