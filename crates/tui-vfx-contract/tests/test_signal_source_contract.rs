// <FILE>crates/tui-vfx-contract/tests/test_signal_source_contract.rs</FILE> - <DESC>Declarative signal value source contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: lock signal source reference and fallback validation.</WCTX>
// <CLOG>0.1.0: INIT — prove SignalSpec defaults, references, unknown-id errors, and fallback kind checks.</CLOG>

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
fn signal_spec_default_validates() {
    let spec = signal("audioLevel", ratio_value_spec(Value::Number(0.25)));

    assert!(spec.validate().is_ok());
}

#[test]
fn signal_reference_resolves_declared_signal() {
    let source = ValueSource::Signal {
        id: SignalId::new("audioLevel"),
        fallback: None,
    };

    assert_eq!(
        source
            .infer_kind(&parameter_map(), &signal_map())
            .expect("signal resolves"),
        ValueKind::Number
    );
}

#[test]
fn signal_reference_rejects_unknown_signal() {
    let source = ValueSource::Signal {
        id: SignalId::new("missing"),
        fallback: None,
    };

    assert!(matches!(
        source.infer_kind(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::UnknownSignal { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn signal_fallback_must_match_signal_kind() {
    let source = ValueSource::Signal {
        id: SignalId::new("audioLevel"),
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

// <FILE>crates/tui-vfx-contract/tests/test_signal_source_contract.rs</FILE> - <DESC>Declarative signal value source contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
