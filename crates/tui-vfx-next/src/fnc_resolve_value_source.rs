// <FILE>crates/tui-vfx-next/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve declarative ValueSource values for proof graph execution</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: convert graph ValueSource inputs into one-shot proof values.</WCTX>
// <CLOG>0.1.0: INIT — resolve literals, parameters, signals, and numeric maps without runtime stores.</CLOG>

use crate::{
    GraphExecutionContext, GraphExecutionError, GraphSpec, NumericRange, ParameterId, SignalId,
    Value, ValueKind, ValueSource,
};

/// Resolve a declarative value source against graph declarations and one execution snapshot.
pub fn resolve_value_source(
    graph: &GraphSpec,
    context: &GraphExecutionContext,
    source: &ValueSource,
) -> Result<Value, GraphExecutionError> {
    match source {
        ValueSource::Literal { value } => Ok(value.clone()),
        ValueSource::Parameter { id, fallback } => {
            resolve_parameter_source(graph, context, id, fallback.as_ref())
        }
        ValueSource::Signal { id, fallback } => {
            resolve_signal_source(graph, context, id, fallback.as_ref())
        }
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => resolve_map_source(graph, context, from, *input, *output, *clamp),
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
    from: &ValueSource,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
) -> Result<Value, GraphExecutionError> {
    let value = resolve_value_source(graph, context, from)?;
    let Some(source) = value.as_range_number() else {
        return Err(GraphExecutionError::NonNumericResolvedMapSource {
            actual: value.kind(),
        });
    };
    let input_min = input.min.expect("GraphSpec validation requires input min");
    let input_max = input.max.expect("GraphSpec validation requires input max");
    let output_min = output.output_min();
    let output_max = output.output_max();
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
    Ok(Value::Number(
        output_min + ratio * (output_max - output_min),
    ))
}

trait OutputBounds {
    fn output_min(self) -> f64;
    fn output_max(self) -> f64;
}

impl OutputBounds for NumericRange {
    fn output_min(self) -> f64 {
        self.min.expect("GraphSpec validation requires output min")
    }

    fn output_max(self) -> f64 {
        self.max.expect("GraphSpec validation requires output max")
    }
}

/// Return true when a resolved value has the requested kind.
pub fn resolved_value_matches_kind(value: &Value, expected: ValueKind) -> bool {
    value.kind() == expected
}

// <FILE>crates/tui-vfx-next/src/fnc_resolve_value_source.rs</FILE> - <DESC>Resolve declarative ValueSource values for proof graph execution</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
