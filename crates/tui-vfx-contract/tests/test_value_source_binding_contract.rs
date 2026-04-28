// <FILE>crates/tui-vfx-contract/tests/test_value_source_binding_contract.rs</FILE> - <DESC>Declarative value source contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: lock parameter/signal/source validation without runtime stores.</WCTX>
// <CLOG>0.1.0: INIT — prove declarative ValueSource, ParameterSpec, and SignalSpec behavior.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    DescriptorValidationError, NumericRange, ParameterId, ParameterSpec, SignalId, SignalSpec,
    Value, ValueKind, ValueSource, ValueSpec,
};

fn ratio_value_spec(default: Value) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Number,
        default: Some(default),
        range: Some(NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        }),
        allowed_values: vec![],
        unit: Some("ratio".to_string()),
        semantic: Some("opacity".to_string()),
    }
}

fn text_value_spec(default: &str) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Text,
        default: Some(Value::Text(default.to_string())),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: Some("label".to_string()),
    }
}

fn parameter(id: &str, value: ValueSpec) -> ParameterSpec {
    ParameterSpec {
        id: ParameterId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value,
        bindable: true,
    }
}

fn signal(id: &str, value: ValueSpec) -> SignalSpec {
    SignalSpec {
        id: SignalId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value,
        required: false,
    }
}

fn parameter_map() -> BTreeMap<ParameterId, ParameterSpec> {
    BTreeMap::from([
        (
            ParameterId::new("opacity"),
            parameter("opacity", ratio_value_spec(Value::Number(0.5))),
        ),
        (
            ParameterId::new("title"),
            parameter("title", text_value_spec("READY")),
        ),
    ])
}

fn signal_map() -> BTreeMap<SignalId, SignalSpec> {
    BTreeMap::from([(
        SignalId::new("audioLevel"),
        signal("audioLevel", ratio_value_spec(Value::Number(0.0))),
    )])
}

#[test]
fn literal_value_source_validates_kind() {
    let source = ValueSource::Literal {
        value: Value::Number(0.5),
    };

    assert!(
        source
            .validate_kind(ValueKind::Number, &parameter_map(), &signal_map())
            .is_ok()
    );
}

#[test]
fn literal_value_source_rejects_wrong_target_kind() {
    let source = ValueSource::Literal {
        value: Value::Boolean(true),
    };

    assert!(matches!(
        source.validate_kind(ValueKind::Number, &parameter_map(), &signal_map()),
        Err(DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Boolean
        })
    ));
}

#[test]
fn parameter_spec_default_validates() {
    let spec = parameter("opacity", ratio_value_spec(Value::Number(0.25)));

    assert!(spec.validate().is_ok());
}

#[test]
fn parameter_reference_resolves_declared_parameter() {
    let source = ValueSource::Parameter {
        id: ParameterId::new("opacity"),
        fallback: None,
    };

    assert_eq!(
        source
            .infer_kind(&parameter_map(), &signal_map())
            .expect("parameter resolves"),
        ValueKind::Number
    );
}

#[test]
fn parameter_reference_rejects_unknown_parameter() {
    let source = ValueSource::Parameter {
        id: ParameterId::new("missing"),
        fallback: None,
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::UnknownParameter { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn parameter_fallback_must_match_parameter_kind() {
    let source = ValueSource::Parameter {
        id: ParameterId::new("opacity"),
        fallback: Some(Value::Text("bad".to_string())),
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn map_source_accepts_numeric_source() {
    let source = ValueSource::Map {
        from: Box::new(ValueSource::Signal {
            id: SignalId::new("audioLevel"),
            fallback: None,
        }),
        input: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        output: NumericRange {
            min: Some(0.2),
            max: Some(0.8),
        },
        clamp: true,
    };

    assert_eq!(
        source
            .infer_kind(&parameter_map(), &signal_map())
            .expect("numeric source maps"),
        ValueKind::Number
    );
}

#[test]
fn map_source_rejects_output_range_missing_bound() {
    let source = ValueSource::Map {
        from: Box::new(ValueSource::Literal {
            value: Value::Number(0.5),
        }),
        input: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        output: NumericRange {
            min: None,
            max: Some(1.0),
        },
        clamp: true,
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::IncompleteMapRange { range }) if range == "output"
    ));
}

#[test]
fn map_source_rejects_non_numeric_source() {
    let source = ValueSource::Map {
        from: Box::new(ValueSource::Parameter {
            id: ParameterId::new("title"),
            fallback: None,
        }),
        input: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        output: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        clamp: true,
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::NonNumericMapSource {
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn map_source_rejects_incomplete_range() {
    let source = ValueSource::Map {
        from: Box::new(ValueSource::Literal {
            value: Value::Number(0.5),
        }),
        input: NumericRange {
            min: Some(0.0),
            max: None,
        },
        output: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        clamp: true,
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::IncompleteMapRange { range }) if range == "input"
    ));
}

#[test]
fn map_source_output_kind_is_number() {
    let source = ValueSource::Map {
        from: Box::new(ValueSource::Literal {
            value: Value::Number(0.5),
        }),
        input: NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        },
        output: NumericRange {
            min: Some(0.0),
            max: Some(100.0),
        },
        clamp: false,
    };

    assert_eq!(
        source.infer_kind(&parameter_map(), &signal_map()).unwrap(),
        ValueKind::Number
    );
}

// <FILE>crates/tui-vfx-contract/tests/test_value_source_binding_contract.rs</FILE> - <DESC>Declarative value source contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
