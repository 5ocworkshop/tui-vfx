// <FILE>crates/tui-vfx-compost/src/validation/fnc_unsupported_input_error.rs</FILE> - <DESC>Build canonical unsupported input load errors</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validation helpers share one error builder while staying OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split unsupported-input error construction from direct input accessors.</CLOG>

use tui_vfx_contract::{NodeId, NodeSpec, Value};

use crate::LoadError;

pub(crate) fn unsupported_input_kind(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    expected: &str,
    value: &Value,
) -> LoadError {
    LoadError::UnsupportedInput {
        node_id: node_id.as_str().to_string(),
        effect: node.effect.as_str().to_string(),
        input: input.to_string(),
        reason: format!("expected {expected} input, found {:?}", value.kind()),
    }
}

// <FILE>crates/tui-vfx-compost/src/validation/fnc_unsupported_input_error.rs</FILE> - <DESC>Build canonical unsupported input load errors</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
