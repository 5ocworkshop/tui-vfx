// <FILE>crates/tui-vfx-contract/src/cls_recipe_scene.rs</FILE> - <DESC>Canonical recipe scene declaration DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: declare scene dimensions and source-backed elements.</WCTX>
// <CLOG>0.1.0: INIT — add canonical recipe scene shape without embedding concrete surfaces.</CLOG>

use crate::{RecipeSceneElement, SceneId};

/// Canonical recipe scene declaration before source rendering/composition.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeScene {
    /// Stable recipe-local scene id.
    pub id: SceneId,
    /// Width of the final composed scene in cells.
    pub width: usize,
    /// Height of the final composed scene in cells.
    pub height: usize,
    /// Source-backed elements composed into the scene.
    pub elements: Vec<RecipeSceneElement>,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_scene.rs</FILE> - <DESC>Canonical recipe scene declaration DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
