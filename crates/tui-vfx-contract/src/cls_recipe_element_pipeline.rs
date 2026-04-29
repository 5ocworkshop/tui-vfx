// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_pipeline.rs</FILE> - <DESC>Recipe element-local graph pipeline reference DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: preserve source-local pipeline integration point without runtime execution.</WCTX>
// <CLOG>0.1.0: INIT — add element-local graph/topology reference for future lowering.</CLOG>

use crate::{GraphId, GraphStep};

/// Optional element-local pipeline reference into the canonical graph.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeElementPipeline {
    /// Graph that owns the nodes used by this element-local pipeline.
    pub graph: GraphId,
    /// Optional pipeline topology subset for this element.
    pub topology: Option<GraphStep>,
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_element_pipeline.rs</FILE> - <DESC>Recipe element-local graph pipeline reference DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
