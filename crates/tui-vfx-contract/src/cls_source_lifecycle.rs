// <FILE>crates/tui-vfx-contract/src/cls_source_lifecycle.rs</FILE> - <DESC>Source lifecycle metadata DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: describe procedural source lifecycle properties.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic seed, time, and resize awareness metadata.</CLOG>

/// Minimal planning metadata for a source descriptor.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceLifecycle {
    /// Whether this source is deterministic when supplied the same seed and inputs.
    pub deterministic_with_seed: bool,
    /// Whether this source can vary with time/clock input.
    pub time_aware: bool,
    /// Whether this source may produce different output when the host size changes.
    pub resize_aware: bool,
}

// <FILE>crates/tui-vfx-contract/src/cls_source_lifecycle.rs</FILE> - <DESC>Source lifecycle metadata DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
