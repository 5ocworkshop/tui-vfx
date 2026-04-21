// <FILE>src/recipe_schema/scene/enum_ra_layer_placement.rs</FILE> - <DESC>Tagged placement enum for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema enum for anchored and absolute layer placement.</WCTX>
// <CLOG>0.1.0: add RaAnchoredPlacement, RaAbsolutePlacement, and adjacently-tagged RaLayerPlacement.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_geometry::types::Anchor;
use tui_vfx_types::Rect;

use super::{deserialize_anchor, serialize_anchor};

#[derive(
    Debug, Clone, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(default, deny_unknown_fields)]
pub struct RaAnchoredPlacement {
    #[serde(
        deserialize_with = "deserialize_anchor",
        serialize_with = "serialize_anchor"
    )]
    pub anchor: Anchor,
    #[config(opaque)]
    #[serde(default)]
    pub offset: (i16, i16),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(default, deny_unknown_fields)]
pub struct RaAbsolutePlacement {
    #[config(opaque)]
    pub rect: Rect,
}

#[non_exhaustive]
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec", rename_all = "snake_case")]
pub enum RaLayerPlacement {
    Anchor(RaAnchoredPlacement),
    Absolute(RaAbsolutePlacement),
}

// <FILE>src/recipe_schema/scene/enum_ra_layer_placement.rs</FILE> - <DESC>RaLayerPlacement</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
