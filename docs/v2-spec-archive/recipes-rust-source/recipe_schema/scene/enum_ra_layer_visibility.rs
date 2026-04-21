// <FILE>src/recipe_schema/scene/enum_ra_layer_visibility.rs</FILE> - <DESC>Visibility selector enum for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema enum for always/phase/predicate layer visibility.</WCTX>
// <CLOG>0.1.0: add RaLayerVisibility.</CLOG>

use serde::{Deserialize, Serialize};

use crate::AnimationPhase;

#[non_exhaustive]
#[derive(
    Debug, Clone, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaLayerVisibility {
    #[default]
    Always,
    #[serde(rename = "phase")]
    Phase(#[config(opaque)] Vec<AnimationPhase>),
    Predicate(String),
}

// <FILE>src/recipe_schema/scene/enum_ra_layer_visibility.rs</FILE> - <DESC>RaLayerVisibility</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
