// <FILE>crates/tui-vfx-next/src/cls_scene_element.rs</FILE> - <DESC>Placed semantic surface element DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: represent one local semantic surface inside a composed scene.</WCTX>
// <CLOG>0.1.0: ADD — introduce schema-ready scene elements with placement, z order, clipping, and write policies.</CLOG>

use crate::{
    CellWritePolicy, ClipPolicy, ElementId, ElementPlacement, LayerId, RoleWritePolicy, Surface,
};

/// One placed semantic surface inside a scene.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SceneElement {
    /// Stable element identity, distinct from semantic cell roles.
    pub id: ElementId,
    /// Optional lightweight layer grouping identity.
    pub layer: Option<LayerId>,
    /// Z order; higher values compose later and appear above lower values.
    pub z_index: i32,
    /// Placement of the element-local surface in scene coordinates.
    pub placement: ElementPlacement,
    /// Element-local semantic surface.
    pub surface: Surface,
    /// Policy for local cells that land outside the final scene bounds.
    pub clip_policy: ClipPolicy,
    /// Policy for whether transparent empty local cells write or skip.
    pub cell_write_policy: CellWritePolicy,
    /// Policy for how a written cell updates the final scene role channel.
    pub role_write_policy: RoleWritePolicy,
}

impl SceneElement {
    /// Create a scene element with default D1 composition policies.
    pub fn new(id: ElementId, surface: Surface, placement: ElementPlacement) -> Self {
        Self {
            id,
            layer: None,
            z_index: 0,
            placement,
            surface,
            clip_policy: ClipPolicy::Clip,
            cell_write_policy: CellWritePolicy::WriteCell,
            role_write_policy: RoleWritePolicy::CopySampledSource,
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_scene_element.rs</FILE> - <DESC>Placed semantic surface element DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
