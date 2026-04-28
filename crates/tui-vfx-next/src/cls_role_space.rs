// <FILE>crates/tui-vfx-next/src/cls_role_space.rs</FILE> - <DESC>Role scope source enum</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract RoleSpace enum.</CLOG>

/// Role source used by role scopes.
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
pub enum RoleSpace {
    /// Role scopes inspect the sampled source role. This is the Phase A/B default.
    #[default]
    SampledSource,
    /// Role scopes inspect the destination role before the write.
    Destination,
}

// <FILE>crates/tui-vfx-next/src/cls_role_space.rs</FILE> - <DESC>Role scope source enum</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
