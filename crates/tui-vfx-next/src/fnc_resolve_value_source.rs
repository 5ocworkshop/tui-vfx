// <FILE>crates/tui-vfx-next/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve declarative ValueSource values for proof graph execution</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>New kernel Phase G4: resolve graph-local values alongside parameters and signals.</WCTX>
// <CLOG>0.2.0: MINOR — return ProofValue and consume graph value bus entries.
// 0.1.0: INIT — resolve literals, parameters, signals, and numeric maps without runtime stores.</CLOG>

use std::collections::BTreeMap;

use crate::{
    GraphExecutionContext, GraphExecutionError, GraphSpec, GraphValueId, NumericRange, ParameterId,
    ProofValue, SignalId, Value, ValueKind, ValueSource,
};

/// Resolve a declarative value source against graph declarations and execution state.
pub fn resolve_value_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    graph_values: &BTreeMap<GraphValueId, ProofValue>,
    source: &ValueSource,
) -> Result<ProofValue, GraphExecutionError> {
    match source {
        ValueSource::Literal { value } => Ok(ProofValue::Frame(value.clone())),
        ValueSource::Parameter { id, fallback } => {
            resolve_parameter_source(graph, context, id, fallback.as_ref()).map(ProofValue::Frame)
        }
        ValueSource::Signal { id, fallback } => {
            resolve_signal_source(graph, context, id, fallback.as_ref()).map(ProofValue::Frame)
        }
        ValueSource::GraphValue { id, fallback } => graph_values
            .get(id)
            .cloned()
            .or_else(|| fallback.clone().map(ProofValue::Frame))
            .ok_or_else(|| GraphExecutionError::MissingGraphValue { id: id.clone() }),
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => resolve_map_source(graph, context, graph_values, from, *input, *output, *clamp),
        ValueSource::SampledField {
            field,
            x,
            y,
            fallback,
        } => resolve_sampled_field_source(
            graph,
            context,
            graph_values,
            field,
            x,
            y,
            fallback.as_ref(),
        ),
        ValueSource::SignalExpression { fallback, .. } => Ok(ProofValue::Frame(
            fallback.clone().unwrap_or(Value::Number(0.0)),
        )),
        ValueSource::PhaseProgress { .. } | ValueSource::Clock { .. } => {
            Ok(ProofValue::Frame(Value::Number(0.0)))
        }
    }
}

fn resolve_parameter_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    id: &ParameterId,
    fallback: Option<&Value>,
) -> Result<Value, GraphExecutionError> {
    let spec = graph
        .parameters
        .get(id)
        .ok_or_else(|| crate::DescriptorValidationError::UnknownParameter { id: id.clone() })?;
    let value = context
        .parameter_values
        .get(id)
        .or(fallback)
        .or(spec.value.default.as_ref())
        .ok_or_else(|| GraphExecutionError::MissingParameterValue { id: id.clone() })?;
    spec.value.validate_value(value)?;
    Ok(value.clone())
}

fn resolve_signal_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    id: &SignalId,
    fallback: Option<&Value>,
) -> Result<Value, GraphExecutionError> {
    let spec = graph
        .signals
        .get(id)
        .ok_or_else(|| crate::DescriptorValidationError::UnknownSignal { id: id.clone() })?;
    let value = context
        .signal_values
        .get(id)
        .or(fallback)
        .or(spec.value.default.as_ref())
        .ok_or_else(|| GraphExecutionError::MissingSignalValue { id: id.clone() })?;
    spec.value.validate_value(value)?;
    Ok(value.clone())
}

fn resolve_map_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    graph_values: &BTreeMap<GraphValueId, ProofValue>,
    from: &ValueSource,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
) -> Result<ProofValue, GraphExecutionError> {
    let value = resolve_value_source(graph, context, graph_values, from)?;
    match value {
        ProofValue::Frame(value) => map_frame_value(value, input, output, clamp),
        ProofValue::NumberCellField(field) => Ok(ProofValue::NumberCellField(
            field.map(|sample| map_number(sample, input, output, clamp)),
        )),
    }
}

fn resolve_sampled_field_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    graph_values: &BTreeMap<GraphValueId, ProofValue>,
    field: &str,
    x: &ValueSource,
    y: &ValueSource,
    fallback: Option<&Value>,
) -> Result<ProofValue, GraphExecutionError> {
    if field != "surfaceAngleFrom" {
        return Err(crate::DescriptorValidationError::UnknownSampledField {
            field: field.to_string(),
        }
        .into());
    }
    let _x = resolve_frame_number(graph, context, graph_values, x)?;
    let _y = resolve_frame_number(graph, context, graph_values, y)?;
    Ok(ProofValue::Frame(
        fallback.cloned().unwrap_or(Value::Number(0.0)),
    ))
}

fn resolve_frame_number(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    graph_values: &BTreeMap<GraphValueId, ProofValue>,
    source: &ValueSource,
) -> Result<f64, GraphExecutionError> {
    match resolve_value_source(graph, context, graph_values, source)? {
        ProofValue::Frame(value) => value.as_range_number().ok_or_else(|| {
            GraphExecutionError::NonNumericResolvedMapSource {
                actual: value.kind(),
            }
        }),
        ProofValue::NumberCellField(_) => Ok(0.0),
    }
}

fn map_frame_value(
    value: Value,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
) -> Result<ProofValue, GraphExecutionError> {
    let Some(source) = value.as_range_number() else {
        return Err(GraphExecutionError::NonNumericResolvedMapSource {
            actual: value.kind(),
        });
    };
    Ok(ProofValue::Frame(Value::Number(map_number(
        source, input, output, clamp,
    ))))
}

fn map_number(source: f64, input: NumericRange, output: NumericRange, clamp: bool) -> f64 {
    let input_min = input.min.expect("GraphSpec validation requires input min");
    let input_max = input.max.expect("GraphSpec validation requires input max");
    let output_min = output
        .min
        .expect("GraphSpec validation requires output min");
    let output_max = output
        .max
        .expect("GraphSpec validation requires output max");
    let source = if clamp {
        source.clamp(input_min, input_max)
    } else {
        source
    };
    let ratio = if input_min == input_max {
        0.0
    } else {
        (source - input_min) / (input_max - input_min)
    };
    output_min + ratio * (output_max - output_min)
}

/// Return true when a resolved value has the requested kind.
pub fn resolved_value_matches_kind(value: &ProofValue, expected: ValueKind) -> bool {
    value.kind() == expected
}

// <FILE>crates/tui-vfx-next/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve declarative ValueSource values for proof graph execution</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
