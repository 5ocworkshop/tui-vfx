// <FILE>crates/tui-vfx-contract/tests/test_source_contract.rs</FILE> - <DESC>Source and asset contract validation tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: prove typed source inputs and structural asset refs.</WCTX>
// <CLOG>0.1.0: INIT — lock source descriptor, source spec, procedural asset, and validation behavior.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    AssetFormat, AssetId, AssetKind, AssetLocator, AssetRef, AssetRequirement, AssetSpec,
    DescriptorValidationError, GraphValueId, GraphValueKinds, NumericRange, ParameterId,
    ParameterSpec, RuntimeMutability, SignalId, SignalSpec, SourceDescriptor, SourceId,
    SourceInputId, SourceInputSpec, SourceKind, SourceLifecycle, SourceOutputSize,
    SourceOutputSpec, SourceRolePolicy, SourceSpec, Value, ValueKind, ValueSource, ValueSpec,
};
use tui_vfx_types::RoleTag;

fn ratio_spec(default: Option<Value>) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Number,
        default,
        range: Some(NumericRange {
            min: Some(0.0),
            max: Some(2.0),
        }),
        allowed_values: vec![],
        unit: Some("ratio".to_string()),
        semantic: Some("speed".to_string()),
    }
}

fn text_spec(default: Option<&str>) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Text,
        default: default.map(|value| Value::Text(value.to_string())),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: Some("sourceText".to_string()),
    }
}

fn source_input(value: ValueSpec, bindable: bool) -> SourceInputSpec {
    SourceInputSpec {
        display_name: None,
        description: Some("Source input under test.".to_string()),
        value,
        bindable,
        runtime_mutability: RuntimeMutability::Runtime,
    }
}

fn text_source_descriptor() -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new("source.text"),
        version: "0.1.0".to_string(),
        display_name: "Text source".to_string(),
        category: Some("source".to_string()),
        kind: SourceKind::Text,
        inputs: BTreeMap::from([(
            SourceInputId::new("text"),
            source_input(text_spec(None), true),
        )]),
        assets: BTreeMap::new(),
        output: SourceOutputSpec {
            size: SourceOutputSize::InputDriven,
            roles: SourceRolePolicy::DefaultRole {
                role: RoleTag::Text,
            },
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: false,
            resize_aware: true,
        },
    }
}

fn card_source_descriptor() -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new("source.card"),
        version: "0.1.0".to_string(),
        display_name: "Card source".to_string(),
        category: Some("source".to_string()),
        kind: SourceKind::Card,
        inputs: BTreeMap::from([(
            SourceInputId::new("title"),
            source_input(text_spec(Some("Processing")), true),
        )]),
        assets: BTreeMap::new(),
        output: SourceOutputSpec {
            size: SourceOutputSize::HostDriven,
            roles: SourceRolePolicy::Generated,
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: false,
            resize_aware: true,
        },
    }
}

fn procedural_descriptor() -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new("source.brailleFlagField"),
        version: "0.1.0".to_string(),
        display_name: "Braille flag field".to_string(),
        category: Some("source".to_string()),
        kind: SourceKind::Procedural,
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            source_input(ratio_spec(None), true),
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRequirement {
                kind: AssetKind::BrailleDotfield,
                format: AssetFormat::new("tui-vfx.braille_flag_asset.v1"),
                required: true,
                description: Some("Base flag dotfield.".to_string()),
            },
        )]),
        output: SourceOutputSpec {
            size: SourceOutputSize::InputDriven,
            roles: SourceRolePolicy::Explicit,
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: true,
            resize_aware: true,
        },
    }
}

fn asset_spec(kind: AssetKind, format: &str) -> AssetSpec {
    AssetSpec {
        id: AssetId::new("flagArt"),
        kind,
        format: AssetFormat::new(format),
        locator: AssetLocator::Path {
            path: "recipes/madeira_flag/assets/base_flag_dots.json".to_string(),
        },
        description: Some("Flag dotfield fixture.".to_string()),
    }
}

fn parameter(id: &str, value: ValueSpec) -> ParameterSpec {
    ParameterSpec {
        id: ParameterId::new(id),
        display_name: None,
        description: None,
        value,
        bindable: true,
    }
}

fn signal(id: &str, value: ValueSpec) -> SignalSpec {
    SignalSpec {
        id: SignalId::new(id),
        display_name: None,
        description: None,
        value,
        required: false,
    }
}

#[test]
fn source_descriptor_declares_typed_inputs() {
    let descriptor = procedural_descriptor();

    assert!(descriptor.validate_contract().is_ok());
    assert_eq!(
        descriptor.inputs[&SourceInputId::new("wave.speed")]
            .value
            .kind,
        ValueKind::Number
    );
}

#[test]
fn source_descriptor_declares_required_asset_slot() {
    let descriptor = procedural_descriptor();

    let requirement = &descriptor.assets[&AssetId::new("flagArt")];
    assert!(requirement.required);
    assert_eq!(requirement.kind, AssetKind::BrailleDotfield);
}

#[test]
fn source_spec_rejects_unknown_source_id() {
    let spec = SourceSpec {
        source: SourceId::new("source.missing"),
        inputs: BTreeMap::new(),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(SourceId::new("source.text"), text_source_descriptor())]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::UnknownSource { id }) if id.as_str() == "source.missing"
    ));
}

#[test]
fn source_spec_rejects_unknown_input() {
    let spec = SourceSpec {
        source: SourceId::new("source.text"),
        inputs: BTreeMap::from([(
            SourceInputId::new("unknown"),
            ValueSource::Literal {
                value: Value::Text("FOLLOW".to_string()),
            },
        )]),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(SourceId::new("source.text"), text_source_descriptor())]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::UnknownSourceInput { input, .. })
            if input.as_str() == "unknown"
    ));
}

#[test]
fn source_spec_rejects_missing_required_input() {
    let spec = SourceSpec {
        source: SourceId::new("source.text"),
        inputs: BTreeMap::new(),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(SourceId::new("source.text"), text_source_descriptor())]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::MissingRequiredSourceInput { input, .. })
            if input.as_str() == "text"
    ));
}

#[test]
fn source_spec_rejects_external_source_for_non_bindable_input() {
    let mut descriptor = text_source_descriptor();
    descriptor
        .inputs
        .get_mut(&SourceInputId::new("text"))
        .unwrap()
        .bindable = false;
    let spec = SourceSpec {
        source: SourceId::new("source.text"),
        inputs: BTreeMap::from([(
            SourceInputId::new("text"),
            ValueSource::Parameter {
                id: ParameterId::new("title"),
                fallback: Some(Value::Text("FOLLOW".to_string())),
            },
        )]),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(SourceId::new("source.text"), descriptor)]),
            &BTreeMap::new(),
            &BTreeMap::from([(
                ParameterId::new("title"),
                parameter("title", text_spec(Some("FOLLOW"))),
            )]),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::SourceInputNotBindable { input, .. })
            if input.as_str() == "text"
    ));
}

#[test]
fn source_spec_rejects_input_kind_mismatch() {
    let spec = SourceSpec {
        source: SourceId::new("source.text"),
        inputs: BTreeMap::from([(
            SourceInputId::new("text"),
            ValueSource::Literal {
                value: Value::Number(0.5),
            },
        )]),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(SourceId::new("source.text"), text_source_descriptor())]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Text,
            actual: ValueKind::Number
        })
    ));
}

#[test]
fn source_spec_accepts_parameter_value_source_for_bindable_input() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Parameter {
                id: ParameterId::new("waveSpeed"),
                fallback: Some(Value::Number(0.8)),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::from([(
                ParameterId::new("waveSpeed"),
                parameter("waveSpeed", ratio_spec(Some(Value::Number(0.8)))),
            )]),
            &BTreeMap::new(),
            None,
        )
        .is_ok()
    );
}

#[test]
fn source_spec_rejects_graph_value_source_outside_graph_context_if_not_allowed() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::GraphValue {
                id: GraphValueId::new("waveOut"),
                fallback: None,
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::GraphValueSourceNotAllowed { id })
            if id.as_str() == "waveOut"
    ));
}

#[test]
fn asset_requirement_schema_is_strict_and_described() {
    let schema = serde_json::to_string_pretty(&schemars::schema_for!(AssetRequirement))
        .expect("asset requirement schema serializes");

    assert!(schema.contains("additionalProperties"));
    assert!(schema.contains("Whether a source spec must supply this asset slot"));
}

#[test]
fn source_spec_rejects_unknown_asset_slot() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("extraArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::UnknownSourceAssetSlot { asset, .. })
            if asset.as_str() == "extraArt"
    ));
}

#[test]
fn asset_ref_rejects_wrong_asset_kind() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::Image, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::AssetKindMismatch { asset, .. })
            if asset.as_str() == "flagArt"
    ));
}

#[test]
fn source_spec_rejects_missing_required_asset() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::new(),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::MissingRequiredAsset { asset, .. })
            if asset.as_str() == "flagArt"
    ));
}

#[test]
fn asset_ref_rejects_unknown_asset_id() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("missing"),
            },
        )]),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::UnknownAssetRef { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn asset_ref_rejects_wrong_asset_format() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(matches!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "other.format"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        ),
        Err(DescriptorValidationError::AssetFormatMismatch { asset, .. })
            if asset.as_str() == "flagArt"
    ));
}

#[test]
fn procedural_source_can_reference_asset_slot() {
    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Literal {
                value: Value::Number(0.8),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::new(),
            None,
        )
        .is_ok()
    );
}

#[test]
fn procedural_source_can_accept_runtime_parameter_input() {
    let mut values = GraphValueKinds::new();
    values.insert(GraphValueId::new("waveOut"), ValueKind::Number);
    assert_eq!(values[&GraphValueId::new("waveOut")], ValueKind::Number);

    let spec = SourceSpec {
        source: SourceId::new("source.brailleFlagField"),
        inputs: BTreeMap::from([(
            SourceInputId::new("wave.speed"),
            ValueSource::Signal {
                id: SignalId::new("clockSpeed"),
                fallback: Some(Value::Number(1.0)),
            },
        )]),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRef {
                id: AssetId::new("flagArt"),
            },
        )]),
    };

    assert!(
        spec.validate(
            &BTreeMap::from([(
                SourceId::new("source.brailleFlagField"),
                procedural_descriptor(),
            )]),
            &BTreeMap::from([(
                AssetId::new("flagArt"),
                asset_spec(AssetKind::BrailleDotfield, "tui-vfx.braille_flag_asset.v1"),
            )]),
            &BTreeMap::new(),
            &BTreeMap::from([(
                SignalId::new("clockSpeed"),
                signal("clockSpeed", ratio_spec(Some(Value::Number(1.0)))),
            )]),
            Some(&values),
        )
        .is_ok()
    );
}

#[test]
fn text_source_descriptor_can_default_text_role() {
    assert!(matches!(
        text_source_descriptor().output.roles,
        SourceRolePolicy::DefaultRole {
            role: RoleTag::Text
        }
    ));
}

#[test]
fn card_source_descriptor_can_declare_generated_roles() {
    assert!(matches!(
        card_source_descriptor().output.roles,
        SourceRolePolicy::Generated
    ));
}

#[test]
fn asset_locator_rejects_legacy_interpolation() {
    let asset = AssetSpec {
        id: AssetId::new("flagArt"),
        kind: AssetKind::BrailleDotfield,
        format: AssetFormat::new("tui-vfx.braille_flag_asset.v1"),
        locator: AssetLocator::Path {
            path: "{{ flag_art }}".to_string(),
        },
        description: None,
    };

    assert!(matches!(
        asset.validate(),
        Err(DescriptorValidationError::InterpolatedAssetLocator { locator })
            if locator == "{{ flag_art }}"
    ));
}

// <FILE>crates/tui-vfx-contract/tests/test_source_contract.rs</FILE> - <DESC>Source and asset contract validation tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
