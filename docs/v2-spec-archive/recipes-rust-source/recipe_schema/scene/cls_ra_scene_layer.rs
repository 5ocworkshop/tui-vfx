// <FILE>src/recipe_schema/scene/cls_ra_scene_layer.rs</FILE> - <DESC>One authored semantic layer inside a scene-bearing recipe</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema type describing one layer's identity, placement, content source, semantic role, overflow, and visibility.</WCTX>
// <CLOG>0.1.0: add RaSceneLayer.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::{LayerId, RoleTag};

use super::{
    RaContentSource, RaLayerOverflow, RaLayerPlacement, RaLayerVisibility, deserialize_layer_id,
    deserialize_role_tag, serialize_layer_id, serialize_role_tag,
};

#[non_exhaustive]
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaSceneLayer {
    #[config(opaque)]
    #[serde(
        deserialize_with = "deserialize_layer_id",
        serialize_with = "serialize_layer_id"
    )]
    pub id: LayerId,
    #[serde(default)]
    pub z: i16,
    pub placement: RaLayerPlacement,
    pub source: RaContentSource,
    #[config(opaque)]
    #[serde(
        deserialize_with = "deserialize_role_tag",
        serialize_with = "serialize_role_tag"
    )]
    pub role_tag: RoleTag,
    #[serde(default)]
    pub overflow: RaLayerOverflow,
    #[serde(default)]
    pub visibility: RaLayerVisibility,
}

impl Default for RaSceneLayer {
    fn default() -> Self {
        Self {
            id: LayerId::from("layer"),
            z: 0,
            placement: RaLayerPlacement::Anchor(Default::default()),
            source: RaContentSource::Text(Default::default()),
            role_tag: RoleTag::Background,
            overflow: RaLayerOverflow::Clip,
            visibility: RaLayerVisibility::Always,
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_scene_layer.rs</FILE> - <DESC>RaSceneLayer</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
