// <FILE>crates/tui-vfx-contract/src/cls_effect_output_spec.rs</FILE> - <DESC>Effect descriptor output specification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: declare descriptor-local outputs nodes may publish.</WCTX>
// <CLOG>0.1.0: INIT — add output kind, shape, and description contract.</CLOG>

use crate::{GraphValueKind, GraphValueShape};

/// Descriptor-local specification for one effect-computed output.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectOutputSpec {
    /// Typed value kind produced by this output.
    pub kind: GraphValueKind,
    /// Cardinality of the produced value.
    pub shape: GraphValueShape,
    /// Optional human-facing description for docs and catalogs.
    pub description: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_output_spec.rs</FILE> - <DESC>Effect descriptor output specification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
