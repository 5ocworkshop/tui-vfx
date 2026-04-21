// <FILE>src/recipe_schema/scene/cls_ra_scene_config.rs</FILE> - <DESC>Top-level scene block for scene-bearing recipes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema root carrying authored layers, fit policy, and a default semantic role.</WCTX>
// <CLOG>0.1.0: add RaSceneConfig.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::RoleTag;

use super::{
    RaSceneFitPolicy, RaSceneLayer, default_background_role, deserialize_role_tag,
    serialize_role_tag,
};

#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaSceneConfig {
    #[serde(default)]
    pub layers: Vec<RaSceneLayer>,
    #[serde(default)]
    pub fit_policy: RaSceneFitPolicy,
    #[config(opaque)]
    #[serde(
        default = "default_background_role",
        deserialize_with = "deserialize_role_tag",
        serialize_with = "serialize_role_tag"
    )]
    pub default_role: RoleTag,
}

impl Default for RaSceneConfig {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            fit_policy: RaSceneFitPolicy::Clip,
            default_role: default_background_role(),
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_scene_config.rs</FILE> - <DESC>RaSceneConfig</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
