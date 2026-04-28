// <FILE>crates/tui-vfx-contract/tests/test_effect_input_contract.rs</FILE> - <DESC>Typed effect input contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: lock typed value specs and descriptor input validation.</WCTX>
// <CLOG>0.1.0: INIT — prove ValueKind, ValueSpec, EffectInputSpec, and descriptor input validation behavior.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    CellAccess, CellChannel, CellWritePolicy, CoordinateSpace, DescriptorValidationError,
    EffectCompletion, EffectDescriptor, EffectDomain, EffectId, EffectInputId, EffectInputSpec,
    EffectLifecycle, NumericRange, RoleSpace, RoleWritePolicyKind, RuntimeMutability, ScopeKind,
    ScopeSpec, ScopeSupport, Value, ValueKind, ValueSpec, WriteSupport,
};
use tui_vfx_types::RoleTag;

fn visual_dim_descriptor() -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new("terminal.dim"),
        version: "0.1.0".to_string(),
        display_name: "Terminal dim".to_string(),
        category: Some("visual".to_string()),
        domain: EffectDomain::FrameFilter,
        cell_access: CellAccess {
            reads: vec![CellChannel::Foreground, CellChannel::Background],
            writes: vec![CellChannel::Foreground, CellChannel::Background],
        },
        scope_support: ScopeSupport {
            kinds: vec![
                ScopeKind::All,
                ScopeKind::Role,
                ScopeKind::Rect,
                ScopeKind::RowRange,
                ScopeKind::ColumnRange,
            ],
            coordinate_spaces: vec![
                CoordinateSpace::DestinationLocal,
                CoordinateSpace::SampledSource,
            ],
            role_spaces: vec![RoleSpace::SampledSource, RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![
                CellWritePolicy::WriteCell,
                CellWritePolicy::SkipTransparentEmpty,
            ],
            role_policies: vec![RoleWritePolicyKind::PreserveDestination],
        },
        inputs: BTreeMap::new(),
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: true,
            deterministic_with_seed: true,
        },
    }
}

fn number_value_spec(default: Value) -> ValueSpec {
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

fn runtime_factor_input(default: Value) -> EffectInputSpec {
    EffectInputSpec {
        display_name: Some("Factor".to_string()),
        description: Some("Opacity multiplier.".to_string()),
        value: number_value_spec(default),
        bindable: true,
        runtime_mutability: RuntimeMutability::Runtime,
    }
}

#[test]
fn number_value_matches_number_kind() {
    assert_eq!(Value::Number(0.5).kind(), ValueKind::Number);
    assert!(number_value_spec(Value::Number(0.5)).validate().is_ok());
}

#[test]
fn number_value_rejects_boolean_kind() {
    let error = number_value_spec(Value::Boolean(true))
        .validate()
        .expect_err("boolean default cannot satisfy number spec");

    assert!(matches!(
        error,
        DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Boolean
        }
    ));
}

#[test]
fn number_default_within_range_is_valid() {
    assert!(number_value_spec(Value::Number(0.25)).validate().is_ok());
}

#[test]
fn number_default_outside_range_is_invalid() {
    let error = number_value_spec(Value::Number(1.25))
        .validate()
        .expect_err("number default outside range must fail");

    assert!(matches!(
        error,
        DescriptorValidationError::NumericValueOutOfRange { value, .. } if value == 1.25
    ));
}

#[test]
fn integer_default_rejects_fractional_number() {
    let spec = ValueSpec {
        kind: ValueKind::Integer,
        default: Some(Value::Number(1.25)),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: None,
    };

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Integer,
            actual: ValueKind::Number
        })
    ));
}

#[test]
fn range_on_non_numeric_kind_is_invalid() {
    let spec = ValueSpec {
        kind: ValueKind::Text,
        default: Some(Value::Text("hello".to_string())),
        range: Some(NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        }),
        allowed_values: vec![],
        unit: None,
        semantic: None,
    };

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::RangeOnNonNumericKind {
            value_kind: ValueKind::Text
        })
    ));
}

#[test]
fn enum_default_must_be_allowed_value() {
    let spec = ValueSpec {
        kind: ValueKind::Enum,
        default: Some(Value::Enum("spring".to_string())),
        range: None,
        allowed_values: vec!["linear".to_string(), "easeOut".to_string()],
        unit: None,
        semantic: Some("curve".to_string()),
    };

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::EnumValueNotAllowed { value }) if value == "spring"
    ));
}

#[test]
fn enum_allowed_values_must_not_be_empty() {
    let spec = ValueSpec {
        kind: ValueKind::Enum,
        default: None,
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: None,
    };

    assert!(matches!(
        spec.validate(),
        Err(DescriptorValidationError::EmptyEnumAllowedValues)
    ));
}

#[test]
fn role_value_round_trips() {
    let value = Value::Role(RoleTag::Text);
    let json = serde_json::to_string(&value).expect("role value serializes");
    let decoded: Value = serde_json::from_str(&json).expect("role value deserializes");

    assert_eq!(decoded, value);
}

#[test]
fn scope_value_round_trips() {
    let value = Value::Scope(ScopeSpec::Rect {
        rect: tui_vfx_types::Rect::new(1, 2, 3, 4),
    });
    let json = serde_json::to_string(&value).expect("scope value serializes");
    let decoded: Value = serde_json::from_str(&json).expect("scope value deserializes");

    assert_eq!(decoded, value);
}

#[test]
fn color_value_round_trips() {
    let value = Value::Color(tui_vfx_types::Color::new(1, 2, 3, 4));
    let json = serde_json::to_string(&value).expect("color value serializes");
    let decoded: Value = serde_json::from_str(&json).expect("color value deserializes");

    assert_eq!(decoded, value);
}

#[test]
fn rect_value_round_trips() {
    let value = Value::Rect(tui_vfx_types::Rect::new(1, 2, 3, 4));
    let json = serde_json::to_string(&value).expect("rect value serializes");
    let decoded: Value = serde_json::from_str(&json).expect("rect value deserializes");

    assert_eq!(decoded, value);
}

#[test]
fn effect_descriptor_accepts_valid_input_spec() {
    let mut descriptor = visual_dim_descriptor();
    descriptor.inputs.insert(
        EffectInputId::new("factor"),
        runtime_factor_input(Value::Number(0.5)),
    );

    assert!(descriptor.validate_inputs().is_ok());
}

#[test]
fn effect_descriptor_rejects_input_default_type_mismatch() {
    let mut descriptor = visual_dim_descriptor();
    descriptor.inputs.insert(
        EffectInputId::new("factor"),
        runtime_factor_input(Value::Boolean(true)),
    );

    assert!(matches!(
        descriptor.validate_inputs(),
        Err(DescriptorValidationError::ValueKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Boolean
        })
    ));
}

#[test]
fn effect_descriptor_rejects_input_default_out_of_range() {
    let mut descriptor = visual_dim_descriptor();
    descriptor.inputs.insert(
        EffectInputId::new("factor"),
        runtime_factor_input(Value::Number(-0.1)),
    );

    assert!(matches!(
        descriptor.validate_inputs(),
        Err(DescriptorValidationError::NumericValueOutOfRange { value, .. }) if value == -0.1
    ));
}

#[test]
fn effect_descriptor_rejects_invalid_input_id() {
    let mut descriptor = visual_dim_descriptor();
    descriptor.inputs.insert(
        EffectInputId::new("1factor"),
        runtime_factor_input(Value::Number(0.5)),
    );

    assert!(matches!(
        descriptor.validate_inputs(),
        Err(DescriptorValidationError::InvalidInputId { id }) if id.as_str() == "1factor"
    ));
}

#[test]
fn phase_f1_does_not_add_value_source_or_parameters() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut combined = String::new();
    for entry in std::fs::read_dir(src_dir).expect("contract src can be read") {
        let path = entry.expect("entry can be read").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            combined.push_str(&std::fs::read_to_string(path).expect("contract source can be read"));
        }
    }

    assert!(!combined.contains("ValueSource"));
    assert!(!combined.contains("ParameterSpec"));
    assert!(!combined.contains("SignalSpec"));
    assert!(!combined.contains("BindingSpec"));
}

// <FILE>crates/tui-vfx-contract/tests/test_effect_input_contract.rs</FILE> - <DESC>Typed effect input contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
