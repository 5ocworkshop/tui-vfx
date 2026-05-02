// <FILE>crates/tui-vfx-compost/src/validation/col_direct_input.rs</FILE> - <DESC>Shared native v3.1 input validation helpers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Direct rendering resolves node inputs through the native runtime value resolver.</WCTX>
// <CLOG>0.3.0: MINOR — validate resolved borrowed-or-owned values instead of literal-only sources.</CLOG>
use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::LoadError;
use crate::runtime::{ResolvedValue, RuntimeContext, resolve_value_source};
use crate::validation::unsupported_input_kind;

pub(crate) fn require_declared_inputs_resolvable(
    node_id: &NodeId,
    node: &NodeSpec,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    for (input, source) in &node.inputs {
        resolve_value_source(source, context).map_err(|error| LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.as_str().to_string(),
            reason: error.reason(),
        })?;
    }
    Ok(())
}

pub(crate) fn require_resolved_input<'a>(
    node_id: &NodeId,
    node: &'a NodeSpec,
    input: &str,
    context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, LoadError> {
    match node.inputs.get(&EffectInputId::new(input)) {
        Some(source) => {
            resolve_value_source(source, context).map_err(|error| LoadError::UnsupportedInput {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                input: input.to_string(),
                reason: error.reason(),
            })
        }
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
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    let value = require_resolved_input(node_id, node, input, context)?;
    if value.value().as_range_number().is_some() {
        Ok(())
    } else {
        Err(unsupported_input_kind(
            node_id,
            node,
            input,
            "numeric",
            value.value(),
        ))
    }
}

pub(crate) fn require_enum_value(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    allowed: &[&str],
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    let value = require_resolved_input(node_id, node, input, context)?;
    let Some(actual) = value.value().as_enum_value() else {
        return Err(unsupported_input_kind(
            node_id,
            node,
            input,
            "enum",
            value.value(),
        ));
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
// <FILE>crates/tui-vfx-compost/src/validation/col_direct_input.rs</FILE> - <DESC>Shared native v3.1 input validation helpers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
