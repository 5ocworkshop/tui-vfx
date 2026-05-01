// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_graph_binding.rs</FILE> - <DESC>Recipe element-local graph binding DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: preserve element-local graph binding without runtime execution.</WCTX>
// <CLOG>0.1.0: INIT — add element-local graph/topology reference for future graph execution.</CLOG>

use crate::{GraphId, GraphStep, RecipeElementGraphTiming};

/// Optional element-local graph binding into the canonical graph.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeElementGraphBinding {
    /// Graph that owns the nodes used by this element-local graph binding.
    pub graph: GraphId,
    /// Optional element-local enter/exit timing envelope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timing: Option<RecipeElementGraphTiming>,
    /// Optional graph-binding topology subset for this element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<GraphStep>,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_graph_binding.rs</FILE> - <DESC>Recipe element-local graph binding DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
