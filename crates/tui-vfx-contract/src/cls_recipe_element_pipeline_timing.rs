// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_pipeline_timing.rs</FILE> - <DESC>Scene element-local pipeline timing DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Scene parity work: preserve layer-local VFX timing offsets and durations from authored scene recipes.</WCTX>
// <CLOG>0.1.0: INIT — add element pipeline timing fields for staggered layer-local enter/exit effects.</CLOG>

/// Element-local timing for scene layer pipeline effects.
///
/// These values are relative to the parent recipe timeline. They do not choose
/// a clock policy; the recipe lifecycle clock still owns sample-space
/// interpretation. Renderers use this envelope only to derive a local
/// `phase_t` for element-local pipeline steps.
#[derive(
    Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeElementPipelineTiming {
    /// Element-local enter duration in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_ms: Option<u64>,
    /// Element-local exit duration in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_ms: Option<u64>,
    /// Element-local enter easing label preserved for backend/tooling parity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_ease: Option<String>,
    /// Element-local exit easing label preserved for backend/tooling parity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_ease: Option<String>,
    /// Offset from parent timeline start before the element enter effect begins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_offset_ms: Option<u64>,
    /// Offset from parent exit timeline start before the element exit effect begins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_offset_ms: Option<u64>,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_pipeline_timing.rs</FILE> - <DESC>Scene element-local pipeline timing DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
