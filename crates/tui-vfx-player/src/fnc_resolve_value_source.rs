// <FILE>crates/tui-vfx-player/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve value sources for player sampling</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player value resolution work: resolve graph-local values during topology execution.</WCTX>
// <CLOG>0.2.0: MINOR — let graph-value sources read from the player graph value bus.
// 0.1.0: INIT — add minimal resolver for source inputs and lifecycle triggers.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{GraphValueId, NumericRange, SignalId, Value, ValueSource};

/// Resolve a value source for K0 player needs.
pub fn resolve_value_source(
    source: &ValueSource,
    signals: &BTreeMap<SignalId, Value>,
) -> Option<Value> {
    resolve_value_source_with_graph_values(source, signals, &BTreeMap::new())
}

/// Resolve a value source with graph-local values available.
pub fn resolve_value_source_with_graph_values(
    source: &ValueSource,
    signals: &BTreeMap<SignalId, Value>,
    graph_values: &BTreeMap<GraphValueId, Value>,
) -> Option<Value> {
    match source {
        ValueSource::Literal { value } => Some(value.clone()),
        ValueSource::Signal { id, fallback } => {
            signals.get(id).cloned().or_else(|| fallback.clone())
        }
        ValueSource::Parameter { fallback, .. } => fallback.clone(),
        ValueSource::GraphValue { id, fallback } => {
            graph_values.get(id).cloned().or_else(|| fallback.clone())
        }
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => map_numeric(
            resolve_value_source_with_graph_values(from, signals, graph_values)?,
            *input,
            *output,
            *clamp,
        ),
        ValueSource::SampledField { fallback, .. } => fallback.clone().or(Some(Value::Number(0.0))),
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

// <FILE>crates/tui-vfx-player/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve value sources for player sampling</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
