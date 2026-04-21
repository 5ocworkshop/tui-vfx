// <FILE>src/recipe_schema/scene/enum_ra_scene_fit_policy.rs</FILE> - <DESC>Fit policy for scene-bearing recipes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema enum for how composed scenes fit the destination area.</WCTX>
// <CLOG>0.1.0: add clip/shrink/scroll scene fit policy enum.</CLOG>

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaSceneFitPolicy {
    #[default]
    Clip,
    Shrink,
    Scroll,
}

// <FILE>src/recipe_schema/scene/enum_ra_scene_fit_policy.rs</FILE> - <DESC>RaSceneFitPolicy</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
