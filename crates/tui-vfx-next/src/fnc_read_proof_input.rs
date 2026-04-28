// <FILE>crates/tui-vfx-next/src/fnc_read_proof_input.rs</FILE> - <DESC>Read typed values for proof adapters</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: keep proof adapter application below OFPF limits.</WCTX>
// <CLOG>0.1.0: INIT — extract proof input conversion helpers.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_types::{Color, RoleTag};

use crate::{EffectId, EffectInputId, GraphExecutionError, Value, ValueKind};

pub(crate) fn input_char(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<char, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        Value::Text(value) | Value::String(value) => Ok(value.chars().next().unwrap_or(' ')),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Text,
            value.kind(),
        )),
    }
}

pub(crate) fn input_number(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<f64, GraphExecutionError> {
    let value = input_value(effect, inputs, input)?;
    value
        .as_range_number()
        .ok_or_else(|| unsupported_input(effect, input, ValueKind::Number, value.kind()))
}

pub(crate) fn input_role(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<RoleTag, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        Value::Role(role) => Ok(role.clone()),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Role,
            value.kind(),
        )),
    }
}

pub(crate) fn input_color(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<Color, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        Value::Color(color) => Ok(*color),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Color,
            value.kind(),
        )),
    }
}

fn input_value<'a>(
    effect: &EffectId,
    inputs: &'a BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<&'a Value, GraphExecutionError> {
    let input_id = EffectInputId::new(input);
    inputs
        .get(&input_id)
        .ok_or_else(|| GraphExecutionError::MissingProofInput {
            effect: effect.clone(),
            input: input_id,
        })
}

fn unsupported_input(
    effect: &EffectId,
    input: &str,
    expected: ValueKind,
    actual: ValueKind,
) -> GraphExecutionError {
    GraphExecutionError::UnsupportedProofInput {
        effect: effect.clone(),
        input: EffectInputId::new(input),
        expected,
        actual,
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_read_proof_input.rs</FILE> - <DESC>Read typed values for proof adapters</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
