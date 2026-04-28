// <FILE>crates/tui-vfx-contract/src/cls_effect_domain.rs</FILE> - <DESC>Coarse effect domain enum</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase E1: lock initial descriptor domain vocabulary.</WCTX>
// <CLOG>0.5.0: MINOR — replace proof-only visual/procedural domains with stable descriptor domain vocabulary.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract EffectDomain enum.</CLOG>

/// Broad execution role declared by an effect descriptor.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum EffectDomain {
    /// Generates content cells without requiring existing source content.
    ContentGenerator,
    /// Transforms existing content while preserving its broad semantic role.
    ContentTransform,
    /// Shades individual cell visual channels.
    CellShader,
    /// Filters a complete frame or surface after content exists.
    FrameFilter,
    /// Changes which source coordinate a destination cell samples.
    CoordinateSampler,
    /// Produces or applies a mask for later stages.
    Mask,
    /// Produces shadow-like support content or role tagging.
    Shadow,
    /// Runs after primary composition as a post-process.
    PostProcess,
    /// Emits diagnostics or analysis without being a visual effect.
    DiagnosticTooling,
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_domain.rs</FILE> - <DESC>Coarse effect domain enum</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
