// <FILE>crates/tui-vfx-compost/src/validation/col_direct_input.rs</FILE> - <DESC>Shared native v3.1 literal input validation helpers</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Direct rendering resolves node inputs through the native runtime value resolver.</WCTX>
// <CLOG>0.2.0: MINOR — route ValueSource handling through the runtime value resolver.
// 0.1.0: INIT — add literal input validators.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec, Value};

use crate::LoadError;
use crate::runtime::{RuntimeContext, resolve_value_source};

pub(crate) fn require_declared_inputs_literal(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), LoadError> {
    let context = RuntimeContext::load_time();
    for (input, source) in &node.inputs {
        resolve_value_source(source, &context).map_err(|error| LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.as_str().to_string(),
            reason: error.reason(),
        })?;
    }
    Ok(())
}

pub(crate) fn require_literal_input<'a>(
    node_id: &NodeId,
    node: &'a NodeSpec,
    input: &str,
) -> Result<&'a Value, LoadError> {
    let context = RuntimeContext::load_time();
    match node.inputs.get(&EffectInputId::new(input)) {
        Some(source) => resolve_value_source(source, &context)
            .map(|resolved| resolved.value())
            .map_err(|error| LoadError::UnsupportedInput {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                input: input.to_string(),
                reason: error.reason(),
            }),
        None => Err(LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: "required input is missing".to_string(),
        }),
    }
}

pub(crate) fn require_number_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), LoadError> {
    let value = require_literal_input(node_id, node, input)?;
    if value.as_range_number().is_some() {
        Ok(())
    } else {
        Err(LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: format!("expected numeric input, found {:?}", value.kind()),
        })
    }
}

pub(crate) fn require_enum_value(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    allowed: &[&str],
) -> Result<(), LoadError> {
    let value = require_literal_input(node_id, node, input)?;
    let Some(actual) = value.as_enum_value() else {
        return Err(LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: format!("expected enum input, found {:?}", value.kind()),
        });
    };
    if allowed.contains(&actual) {
        Ok(())
    } else {
        Err(LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: format!("unsupported enum value `{actual}`"),
        })
    }
}

// <FILE>crates/tui-vfx-compost/src/validation/col_direct_input.rs</FILE> - <DESC>Shared native v3.1 literal input validation helpers</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
