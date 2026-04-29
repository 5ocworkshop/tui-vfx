// <FILE>crates/tui-vfx-contract/src/cls_value_source.rs</FILE> - <DESC>Declarative value source DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: allow node inputs to consume graph-local values.</WCTX>
// <CLOG>0.2.0: MINOR — add graphValue source and context-aware validation.
// 0.1.0: INIT — add declarative ValueSource variants and reference/kind validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, GraphValueId, NumericRange, ParameterId, ParameterSpec, SignalId,
    SignalSpec, Value, ValueKind,
};

/// Value kind lookup for graph-local values available to a node input.
pub type GraphValueKinds = BTreeMap<GraphValueId, ValueKind>;

/// Declarative source for a typed value.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ValueSource {
    /// Source value is a typed literal.
    Literal {
        /// Literal typed value.
        value: Value,
    },
    /// Source value comes from a declared public parameter.
    Parameter {
        /// Referenced parameter id.
        id: ParameterId,
        /// Optional fallback literal when the parameter is unavailable.
        fallback: Option<Value>,
    },
    /// Source value comes from a declared host/runtime signal.
    Signal {
        /// Referenced signal id.
        id: SignalId,
        /// Optional fallback literal when the signal is unavailable.
        fallback: Option<Value>,
    },
    /// Source value comes from an earlier graph-local node output.
    GraphValue {
        /// Referenced graph-local value id.
        id: GraphValueId,
        /// Optional fallback literal when the value is unavailable at execution time.
        fallback: Option<Value>,
    },
    /// Source value maps a numeric source from one range to another.
    Map {
        /// Nested numeric-compatible source to map.
        from: Box<ValueSource>,
        /// Inclusive input range for the nested source.
        input: NumericRange,
        /// Inclusive output range produced by the map.
        output: NumericRange,
        /// Whether values outside the input range should clamp before mapping.
        clamp: bool,
    },
}

impl ValueSource {
    /// Infer the value kind produced by this source using declared parameters and signals.
    pub fn infer_kind(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<ValueKind, DescriptorValidationError> {
        self.infer_kind_with_graph_values(parameters, signals, None)
    }

    /// Infer the source kind with optional graph-local value declarations.
    pub fn infer_kind_with_graph_values(
        &self,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
        graph_values: Option<&GraphValueKinds>,
    ) -> Result<ValueKind, DescriptorValidationError> {
        match self {
            Self::Literal { value } => Ok(value.kind()),
            Self::Parameter { id, fallback } => {
                let spec = parameters.get(id).ok_or_else(|| {
                    DescriptorValidationError::UnknownParameter { id: id.clone() }
                })?;
                spec.validate()?;
                if let Some(value) = fallback {
                    spec.value.validate_value(value)?;
                }
                Ok(spec.value.kind)
            }
            Self::Signal { id, fallback } => {
                let spec = signals
                    .get(id)
                    .ok_or_else(|| DescriptorValidationError::UnknownSignal { id: id.clone() })?;
                spec.validate()?;
                if let Some(value) = fallback {
                    spec.value.validate_value(value)?;
                }
                Ok(spec.value.kind)
            }
            Self::GraphValue { id, fallback } => {
                let Some(values) = graph_values else {
                    return Err(DescriptorValidationError::GraphValueSourceNotAllowed {
                        id: id.clone(),
                    });
                };
                let kind = *values.get(id).ok_or_else(|| {
                    DescriptorValidationError::UnknownGraphValue { id: id.clone() }
                })?;
                if let Some(value) = fallback
                    && value.kind() != kind
                {
                    return Err(DescriptorValidationError::SourceKindMismatch {
                        expected: kind,
                        actual: value.kind(),
                    });
                }
                Ok(kind)
            }
            Self::Map {
                from,
                input,
                output,
                ..
            } => {
                validate_map_range(*input, "input")?;
                validate_map_range(*output, "output")?;
                let source_kind =
                    from.infer_kind_with_graph_values(parameters, signals, graph_values)?;
                if !is_numeric_kind(source_kind) {
                    return Err(DescriptorValidationError::NonNumericMapSource {
                        actual: source_kind,
                    });
                }
                Ok(ValueKind::Number)
            }
        }
    }

    /// Validate this source against an expected target value kind.
    pub fn validate_kind(
        &self,
        expected: ValueKind,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
    ) -> Result<(), DescriptorValidationError> {
        self.validate_kind_with_graph_values(expected, parameters, signals, None)
    }

    /// Validate this source against an expected kind with graph-local values allowed.
    pub fn validate_kind_with_graph_values(
        &self,
        expected: ValueKind,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
        graph_values: Option<&GraphValueKinds>,
    ) -> Result<(), DescriptorValidationError> {
        let actual = self.infer_kind_with_graph_values(parameters, signals, graph_values)?;
        if actual == expected {
            Ok(())
        } else {
            Err(DescriptorValidationError::SourceKindMismatch { expected, actual })
        }
    }
}

fn is_numeric_kind(kind: ValueKind) -> bool {
    matches!(
        kind,
        ValueKind::Integer | ValueKind::Number | ValueKind::Duration
    )
}

fn validate_map_range(
    range: NumericRange,
    range_name: &str,
) -> Result<(), DescriptorValidationError> {
    if range.min.is_none() || range.max.is_none() {
        return Err(DescriptorValidationError::IncompleteMapRange {
            range: range_name.to_string(),
        });
    }
    if let Some(value) = range.min.filter(|value| !value.is_finite()) {
        return Err(DescriptorValidationError::NonFiniteNumericRangeBound { value });
    }
    if let Some(value) = range.max.filter(|value| !value.is_finite()) {
        return Err(DescriptorValidationError::NonFiniteNumericRangeBound { value });
    }
    if !range.is_ordered() {
        return Err(DescriptorValidationError::InvalidNumericRange {
            min: range.min,
            max: range.max,
        });
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/cls_value_source.rs</FILE> - <DESC>Declarative value source DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
