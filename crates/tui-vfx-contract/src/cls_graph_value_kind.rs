// <FILE>crates/tui-vfx-contract/src/cls_graph_value_kind.rs</FILE> - <DESC>Graph value bus value kind vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: type node-published graph values.</WCTX>
// <CLOG>0.1.0: INIT — add initial scalar number graph value kind.</CLOG>

use crate::ValueKind;

/// Closed vocabulary for values published on the graph-local value bus.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum GraphValueKind {
    /// Floating-point scalar value. Shape decides whether it is frame-wide or per-cell.
    Number,
}

impl GraphValueKind {
    /// Return the compatible effect-input value kind for this graph value kind.
    pub const fn value_kind(self) -> ValueKind {
        match self {
            Self::Number => ValueKind::Number,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_graph_value_kind.rs</FILE> - <DESC>Graph value bus value kind vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
