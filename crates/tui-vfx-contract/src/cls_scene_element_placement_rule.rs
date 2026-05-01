// <FILE>crates/tui-vfx-contract/src/cls_scene_element_placement_rule.rs</FILE> - <DESC>Scene element placement rule DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 scene parity: preserve anchor, absolute-rect, sibling-relative, and motion placement semantics.</WCTX>
// <CLOG>0.1.0: INIT — add optional declarative placement rule layered over resolved placement fallback.</CLOG>

use tui_vfx_types::Rect;

use crate::{LayerId, SceneAnchor, StructuredValue};

/// Declarative placement rule for a recipe scene element.
///
/// The existing `placement` field remains the resolved absolute fallback.
/// `placementRule` carries richer scene authoring intent so players and validators
/// can reproduce anchor, sibling-relative, and motion-aware scenes without
/// flattening those semantics out of the recipe tree.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum SceneElementPlacementRule {
    /// Place by an absolute target rectangle in scene coordinates.
    Absolute {
        /// Absolute target rectangle.
        rect: Rect,
        /// Optional placement-motion payload preserved from scene authoring.
        placement_motion: Option<StructuredValue>,
    },
    /// Place relative to a scene anchor or an already-declared sibling layer.
    Anchor {
        /// Anchor point used for placement.
        anchor: SceneAnchor,
        /// Row offset applied after resolving the anchor.
        #[serde(rename = "offsetRows")]
        offset_rows: i32,
        /// Column offset applied after resolving the anchor.
        #[serde(rename = "offsetColumns")]
        offset_columns: i32,
        /// Optional sibling layer used as the anchor frame instead of the scene.
        #[serde(rename = "siblingLayer")]
        sibling_layer: Option<LayerId>,
        /// Optional placement-motion payload preserved from scene authoring.
        placement_motion: Option<StructuredValue>,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_element_placement_rule.rs</FILE> - <DESC>SceneElementPlacementRule</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
