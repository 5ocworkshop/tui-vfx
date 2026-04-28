// <FILE>crates/tui-vfx-next/src/cls_coordinate_space.rs</FILE> - <DESC>Geometry scope coordinate-space enum</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract CoordinateSpace enum.</CLOG>

/// Coordinate space used by geometry scopes.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum CoordinateSpace {
    /// Destination-local coordinates. This is the Phase A/B default.
    #[default]
    DestinationLocal,
    /// Sampled-source local coordinates. Present to make the contract explicit.
    SampledSource,
}

// <FILE>crates/tui-vfx-next/src/cls_coordinate_space.rs</FILE> - <DESC>Geometry scope coordinate-space enum</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
