// <FILE>crates/tui-vfx-player/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve K0 value sources for player sampling</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: read literal, signal fallback, and simple map values.</WCTX>
// <CLOG>0.1.0: INIT — add minimal resolver for source inputs and lifecycle triggers.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{NumericRange, SignalId, Value, ValueSource};

/// Resolve a value source for K0 player needs.
pub fn resolve_value_source(
    source: &ValueSource,
    signals: &BTreeMap<SignalId, Value>,
) -> Option<Value> {
    match source {
        ValueSource::Literal { value } => Some(value.clone()),
        ValueSource::Signal { id, fallback } => {
            signals.get(id).cloned().or_else(|| fallback.clone())
        }
        ValueSource::Parameter { fallback, .. } | ValueSource::GraphValue { fallback, .. } => {
            fallback.clone()
        }
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => map_numeric(
            resolve_value_source(from, signals)?,
            *input,
            *output,
            *clamp,
        ),
    }
}

/// Resolve an integer source input with a fallback.
pub fn resolve_integer(
    source: Option<&ValueSource>,
    signals: &BTreeMap<SignalId, Value>,
    fallback: i64,
) -> i64 {
    match source.and_then(|source| resolve_value_source(source, signals)) {
        Some(Value::Integer(value)) => value,
        Some(Value::Number(value)) => value.round() as i64,
        _ => fallback,
    }
}

/// Resolve a text source input with a fallback.
pub fn resolve_text(
    source: Option<&ValueSource>,
    signals: &BTreeMap<SignalId, Value>,
    fallback: &str,
) -> String {
    match source.and_then(|source| resolve_value_source(source, signals)) {
        Some(Value::Text(value) | Value::String(value) | Value::Enum(value)) => value,
        Some(Value::Integer(value)) => value.to_string(),
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Boolean(value)) => value.to_string(),
        _ => fallback.to_string(),
    }
}

fn map_numeric(
    value: Value,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
) -> Option<Value> {
    let number = match value {
        Value::Integer(value) => value as f64,
        Value::Number(value) | Value::Duration(value) => value,
        _ => return None,
    };
    let min_in = input.min?;
    let max_in = input.max?;
    let min_out = output.min?;
    let max_out = output.max?;
    let mut t = (number - min_in) / (max_in - min_in);
    if clamp {
        t = t.clamp(0.0, 1.0);
    }
    Some(Value::Number(min_out + t * (max_out - min_out)))
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve K0 value sources for player sampling</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
