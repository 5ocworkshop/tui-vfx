// <FILE>crates/tui-vfx-contract/src/cls_node_output_spec.rs</FILE> - <DESC>Node graph value output declaration DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: let nodes declare graph-local value publications.</WCTX>
// <CLOG>0.1.0: INIT — add output-source wrapper for node output maps.</CLOG>

use crate::NodeOutputSource;

/// Declaration for one graph-local value published by a node.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeOutputSpec {
    /// Where the value is read from after node input resolution/effect execution.
    pub source: NodeOutputSource,
}

// <FILE>crates/tui-vfx-contract/src/cls_node_output_spec.rs</FILE> - <DESC>Node graph value output declaration DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
