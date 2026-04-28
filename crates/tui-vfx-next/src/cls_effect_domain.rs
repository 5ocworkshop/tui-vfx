// <FILE>crates/tui-vfx-next/src/cls_effect_domain.rs</FILE> - <DESC>Coarse effect domain enum</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract EffectDomain enum.</CLOG>

/// Coarse effect domain declaration.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum EffectDomain {
    /// Visual-only effects may change cell visual channels but not roles.
    Visual,
    /// Procedural effects may generate cells and explicit roles.
    Procedural,
}

// <FILE>crates/tui-vfx-next/src/cls_effect_domain.rs</FILE> - <DESC>Coarse effect domain enum</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
