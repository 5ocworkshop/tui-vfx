// <FILE>crates/tui-vfx-contract/src/cls_recipe_scene_element.rs</FILE> - <DESC>Canonical recipe scene element source reference DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>v3.1 pre-release scene vocabulary: reserve per-element scroll response metadata.</WCTX>
// <CLOG>0.2.0: MINOR — add optional scrollFactor metadata for future scene-scroll/parallax backends.
// 0.1.0: INIT — add source-backed scene element shape for canonical recipes.</CLOG>

use crate::{
    CellWritePolicy, ClipPolicy, ElementId, ElementPlacement, LayerId, RecipeElementGraphBinding,
    RoleWritePolicy, SceneElementOverflowPolicy, SceneElementPlacementRule, SceneElementSurface,
    SceneElementVisibility, ScrollFactor, SourceInstanceId, StructuredValue,
};

/// Scene element whose surface is produced by a declared source instance.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeSceneElement {
    /// Stable element identity, distinct from roles and source instances.
    pub id: ElementId,
    /// Optional lightweight layer grouping identity.
    pub layer: Option<LayerId>,
    /// Z order; higher values compose later and appear above lower values.
    pub z_index: i32,
    /// Placement of the source-produced surface in scene coordinates.
    pub placement: ElementPlacement,
    /// Source instance that produces this element-local semantic surface.
    pub source_instance: SourceInstanceId,
    /// Optional element-local graph binding.
    pub graph_binding: Option<RecipeElementGraphBinding>,
    /// Optional declarative placement rule preserving anchor, sibling-relative, and motion placement semantics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_rule: Option<SceneElementPlacementRule>,
    /// Optional typed visibility policy for phase-aware or binding-backed layer visibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<SceneElementVisibility>,
    /// Optional layer-local surface style and shadow envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<SceneElementSurface>,
    /// Optional overflow policy beyond simple scene-bound clipping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overflow: Option<SceneElementOverflowPolicy>,
    /// Optional structured placement-motion payload preserved from scene authoring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_motion: Option<StructuredValue>,
    /// Optional per-element scene scroll/camera response factor reserved for future backends.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_factor: Option<ScrollFactor>,
    /// Policy for local cells that land outside the final scene bounds.
    pub clip_policy: ClipPolicy,
    /// Policy for whether transparent empty local cells write or skip.
    pub cell_write_policy: CellWritePolicy,
    /// Policy for how a written cell updates the final scene role channel.
    pub role_write_policy: RoleWritePolicy,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_scene_element.rs</FILE> - <DESC>Canonical recipe scene element source reference DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
