// <FILE>crates/tui-vfx-next/src/fnc_read_proof_input.rs</FILE> - <DESC>Read typed values for proof adapters</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: read frame and cell-field proof inputs.</WCTX>
// <CLOG>0.2.0: MINOR — support ProofValue inputs and numeric cell fields.
// 0.1.0: INIT — extract proof input conversion helpers.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_types::{Color, RoleTag};

use crate::{
    EffectId, EffectInputId, GraphExecutionError, NumberCellField, ProofValue, Value, ValueKind,
};

pub(crate) enum ProofNumberInput<'a> {
    Frame(f64),
    Field(&'a NumberCellField),
}

pub(crate) fn input_char(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<char, GraphExecutionError> {
    match frame_input_value(effect, inputs, input)? {
        Value::Text(value) | Value::String(value) => Ok(value.chars().next().unwrap_or(' ')),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Text,
            value.kind(),
        )),
    }
}

pub(crate) fn input_number<'a>(
    effect: &EffectId,
    inputs: &'a BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<ProofNumberInput<'a>, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        ProofValue::Frame(value) => value
            .as_range_number()
            .map(ProofNumberInput::Frame)
            .ok_or_else(|| unsupported_input(effect, input, ValueKind::Number, value.kind())),
        ProofValue::NumberCellField(field) => Ok(ProofNumberInput::Field(field)),
    }
}

pub(crate) fn input_role(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<RoleTag, GraphExecutionError> {
    match frame_input_value(effect, inputs, input)? {
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
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<Color, GraphExecutionError> {
    match frame_input_value(effect, inputs, input)? {
        Value::Color(color) => Ok(*color),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Color,
            value.kind(),
        )),
    }
}

fn frame_input_value<'a>(
    effect: &EffectId,
    inputs: &'a BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<&'a Value, GraphExecutionError> {
    let value = input_value(effect, inputs, input)?;
    value
        .frame()
        .ok_or_else(|| GraphExecutionError::UnsupportedProofInput {
            effect: effect.clone(),
            input: EffectInputId::new(input),
            expected: ValueKind::Number,
            actual: value.kind(),
        })
}

fn input_value<'a>(
    effect: &EffectId,
    inputs: &'a BTreeMap<EffectInputId, ProofValue>,
    input: &str,
) -> Result<&'a ProofValue, GraphExecutionError> {
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
// <VERS>END OF VERSION: 0.2.0</VERS>
