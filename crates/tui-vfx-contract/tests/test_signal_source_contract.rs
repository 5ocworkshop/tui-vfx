// <FILE>crates/tui-vfx-contract/tests/test_signal_source_contract.rs</FILE> - <DESC>Declarative signal value source contract tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase F2: lock signal source reference and fallback validation.</WCTX>
// <CLOG>0.2.0: MINOR — prove typed signal expressions and time-derived value sources.
// 0.1.0: INIT — prove SignalSpec defaults, references, unknown-id errors, and fallback kind checks.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    ClockValueSource, DescriptorValidationError, DurationSpec, LifecyclePhase, NumericRange,
    ParameterId, ParameterSpec, PreviewLoopbackSpec, SignalExpressionSpec, SignalId, SignalSpec,
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
        preview_loopback: None,
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

#[test]
fn signal_preview_loopback_literal_validates_against_signal_kind() {
    let mut spec = signal("demoProgress", ratio_value_spec(Value::Number(0.0)));
    spec.preview_loopback = Some(PreviewLoopbackSpec::Literal {
        value: Value::Number(0.75),
    });

    assert!(spec.validate().is_ok());
}

#[test]
fn signal_preview_loopback_rejects_literal_kind_mismatch() {
    let mut spec = signal("demoProgress", ratio_value_spec(Value::Number(0.0)));
    spec.preview_loopback = Some(PreviewLoopbackSpec::Literal {
        value: Value::Text("wrong".to_string()),
    });

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn numeric_preview_loopback_ramp_requires_numeric_signal_kind() {
    let mut spec = signal("demoTitle", text_value_spec("READY"));
    spec.preview_loopback = Some(PreviewLoopbackSpec::NumericRamp {
        start: 0.0,
        end: 1.0,
        duration: DurationSpec::Milliseconds { value: 800 },
        repeat: true,
    });

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Text,
            actual: ValueKind::Number
        })
    ));
}

#[test]
fn signal_expression_loopback_is_typed_not_structured_escape_hatch() {
    let mut spec = signal("demoFrequency", ratio_value_spec(Value::Number(1.0)));
    spec.preview_loopback = Some(PreviewLoopbackSpec::Expression {
        expression: SignalExpressionSpec::Mix {
            a: Box::new(SignalExpressionSpec::Triangle {
                frequency_hz: 0.6,
                amplitude: 0.6,
                offset: 1.0,
                phase: 0.0,
            }),
            b: Box::new(SignalExpressionSpec::Sine {
                frequency_hz: 0.3,
                amplitude: 0.6,
                offset: 1.0,
                phase: 0.0,
            }),
            mix: 0.5,
        },
        fallback: Some(Value::Number(1.0)),
    });

    assert!(spec.validate().is_ok());
    let json = serde_json::to_value(&spec).expect("signal serializes");
    assert_eq!(json["previewLoopback"]["kind"], "expression");
    assert_eq!(json["previewLoopback"]["expression"]["kind"], "mix");
    assert_eq!(
        json["previewLoopback"]["expression"]["a"]["kind"],
        "triangle"
    );
}

#[test]
fn value_source_can_use_authored_signal_expression_phase_progress_and_clock() {
    let expression = ValueSource::SignalExpression {
        expression: SignalExpressionSpec::Sine {
            frequency_hz: 0.5,
            amplitude: 0.5,
            offset: 0.5,
            phase: 0.0,
        },
        fallback: Some(Value::Number(0.5)),
    };
    assert_eq!(
        expression
            .infer_kind(&Default::default(), &Default::default())
            .unwrap(),
        ValueKind::Number
    );

    let phase = ValueSource::PhaseProgress {
        phase: LifecyclePhase::Dwell,
    };
    assert_eq!(
        phase
            .infer_kind(&Default::default(), &Default::default())
            .unwrap(),
        ValueKind::Number
    );

    let clock = ValueSource::Clock {
        clock: ClockValueSource::RecipeSeconds,
    };
    assert_eq!(
        clock
            .infer_kind(&Default::default(), &Default::default())
            .unwrap(),
        ValueKind::Number
    );
}

// <FILE>crates/tui-vfx-contract/tests/test_signal_source_contract.rs</FILE> - <DESC>Declarative signal value source contract tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
