// <FILE>crates/tui-vfx-contract/src/cls_node_output_source.rs</FILE> - <DESC>Node output source DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: describe how nodes publish graph values.</WCTX>
// <CLOG>0.1.0: INIT — support effect-output publication and input re-emission.</CLOG>

use crate::{EffectInputId, EffectOutputId};

/// Source used by a node to publish one graph-local value.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum NodeOutputSource {
    /// Publish an output computed by the proof/real effect adapter.
    EffectOutput {
        /// Descriptor-local output id.
        id: EffectOutputId,
    },
    /// Re-emit one resolved node input into the graph-local value bus.
    Input {
        /// Descriptor-local input id.
        id: EffectInputId,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_node_output_source.rs</FILE> - <DESC>Node output source DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
