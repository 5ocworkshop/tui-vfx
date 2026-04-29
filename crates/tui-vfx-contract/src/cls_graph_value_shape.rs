// <FILE>crates/tui-vfx-contract/src/cls_graph_value_shape.rs</FILE> - <DESC>Graph value bus cardinality vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: distinguish frame values from spatial fields.</WCTX>
// <CLOG>0.1.0: INIT — add frameValue and cellField graph value shapes.</CLOG>

/// Cardinality or sampling shape for a graph-local value.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum GraphValueShape {
    /// One value applies to the entire frame/node execution.
    FrameValue,
    /// Value may vary per destination/sample cell.
    CellField,
}

// <FILE>crates/tui-vfx-contract/src/cls_graph_value_shape.rs</FILE> - <DESC>Graph value bus cardinality vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
