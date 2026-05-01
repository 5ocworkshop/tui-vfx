// <FILE>crates/tui-vfx-contract/tests/test_binding_contract.rs</FILE> - <DESC>Declarative parameter binding contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F2: lock parameter-target binding validation without node graph/runtime stores.</WCTX>
// <CLOG>0.1.0: INIT — prove parameter-only BindingSpec compatibility and F2 no-go guard.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    BindingMode, BindingSpec, BindingTarget, DescriptorValidationError, NumericRange, ParameterId,
    ParameterSpec, SignalId, SignalSpec, Value, ValueKind, ValueSource, ValueSpec,
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
fn binding_parameter_target_accepts_compatible_source() {
    let binding = BindingSpec {
        binding_target: BindingTarget::Parameter {
            id: ParameterId::new("opacity"),
        },
        value_source: ValueSource::Signal {
            id: SignalId::new("audioLevel"),
            fallback: None,
        },
        binding_mode: BindingMode::Replace,
    };

    assert!(binding.validate(&parameter_map(), &signal_map()).is_ok());
}

#[test]
fn binding_parameter_target_rejects_incompatible_source() {
    let binding = BindingSpec {
        binding_target: BindingTarget::Parameter {
            id: ParameterId::new("opacity"),
        },
        value_source: ValueSource::Parameter {
            id: ParameterId::new("title"),
            fallback: None,
        },
        binding_mode: BindingMode::Replace,
    };

    assert!(matches!(
        binding.validate(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn binding_parameter_target_rejects_non_bindable_parameter() {
    let mut parameters = parameter_map();
    parameters.insert(
        ParameterId::new("locked"),
        ParameterSpec {
            id: ParameterId::new("locked"),
            display_name: Some("Locked".to_string()),
            description: None,
            value: ratio_value_spec(Value::Number(0.5)),
            bindable: false,
        },
    );
    let binding = BindingSpec {
        binding_target: BindingTarget::Parameter {
            id: ParameterId::new("locked"),
        },
        value_source: ValueSource::Literal {
            value: Value::Number(0.5),
        },
        binding_mode: BindingMode::Replace,
    };

    assert!(matches!(
        binding.validate(&parameters, &signal_map()),
        Err(DescriptorValidationError::ParameterNotBindable { id }) if id.as_str() == "locked"
    ));
}

#[test]
fn binding_unknown_parameter_target_rejected() {
    let binding = BindingSpec {
        binding_target: BindingTarget::Parameter {
            id: ParameterId::new("missing"),
        },
        value_source: ValueSource::Literal {
            value: Value::Number(0.5),
        },
        binding_mode: BindingMode::Replace,
    };

    assert!(matches!(
        binding.validate(&parameter_map(), &signal_map()),
        Err(DescriptorValidationError::UnknownBindingParameterTarget { id })
            if id.as_str() == "missing"
    ));
}

#[test]
fn f2_does_not_add_runtime_store_or_recipe_execution() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut combined = String::new();
    for entry in std::fs::read_dir(src_dir).expect("contract src can be read") {
        let path = entry.expect("entry can be read").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            combined.push_str(&std::fs::read_to_string(path).expect("contract source can be read"));
        }
    }

    assert!(!combined.contains("ParameterStore"));
    assert!(!combined.contains("SignalStore"));
    assert!(!combined.contains("RuntimeBinding"));
    assert!(!combined.contains("RecipeCompiler"));
}

// <FILE>crates/tui-vfx-contract/tests/test_binding_contract.rs</FILE> - <DESC>Declarative parameter binding contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
