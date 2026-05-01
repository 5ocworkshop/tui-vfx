// <FILE>crates/tui-vfx-compost/src/validation/col_direct_input.rs</FILE> - <DESC>Shared native v3.1 literal input validation helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct rendering supports canonical literal inputs only until a runtime binding model is deliberately added.</WCTX>
// <CLOG>0.1.0: INIT — add literal input validators.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec, Value, ValueSource};

use crate::LoadError;

pub(crate) fn require_declared_inputs_literal(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), LoadError> {
    for (input, source) in &node.inputs {
        if !matches!(source, ValueSource::Literal { .. }) {
            return Err(LoadError::UnsupportedInput {
                node_id: node_id.as_str().to_string(),
                effect: node.effect.as_str().to_string(),
                input: input.as_str().to_string(),
                reason: "native compost rendering currently accepts literal inputs only"
                    .to_string(),
            });
        }
    }
    Ok(())
}

pub(crate) fn require_literal_input<'a>(
    node_id: &NodeId,
    node: &'a NodeSpec,
    input: &str,
) -> Result<&'a Value, LoadError> {
    match node.inputs.get(&EffectInputId::new(input)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(LoadError::UnsupportedInput {
            node_id: node_id.as_str().to_string(),
            effect: node.effect.as_str().to_string(),
            input: input.to_string(),
            reason: "native compost rendering currently accepts literal inputs only".to_string(),
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
// <VERS>END OF VERSION: 0.1.0</VERS>
