// <FILE>crates/tui-vfx-contract/tests/test_lifecycle_contract.rs</FILE> - <DESC>Recipe lifecycle, time, and trigger contract tests</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>New kernel Phase J2: keep lifecycle recipe fixture current with descriptor pack refs.</WCTX>
// <CLOG>0.1.1: PATCH — initialize empty descriptor pack refs in lifecycle recipe fixture.
// 0.1.0: INIT — lock clock, dwell, trigger, predicate, and RecipeDocument lifecycle validation.</CLOG>

mod support;

use std::collections::BTreeMap;

use support::{base_graph, literal_source, signal};
use tui_vfx_contract::{
    ClockMode, ClockSpec, DescriptorValidationError, DurationSpec, DwellPolicy, GraphValueId,
    LifecyclePhase, LifecycleSpec, ParameterId, PhaseSpec, PhaseTiming, RecipeDocument, RecipeId,
    RecipeMetadata, SignalId, TriggerAction, TriggerCondition, TriggerLatchPolicy,
    TriggerResetBoundary, TriggerSpec, Value, ValueKind, ValuePredicate, ValueSource, ValueSpec,
};

fn ms(value: u64) -> DurationSpec {
    DurationSpec::Milliseconds { value }
}

fn fixed_lifecycle() -> LifecycleSpec {
    LifecycleSpec {
        clock: ClockSpec {
            clock_mode: ClockMode::Monotonic,
            period: None,
        },
        phases: vec![
            PhaseSpec {
                phase: LifecyclePhase::Enter,
                timing: PhaseTiming::Fixed { duration: ms(120) },
            },
            PhaseSpec {
                phase: LifecyclePhase::Dwell,
                timing: PhaseTiming::Fixed {
                    duration: ms(5_000),
                },
            },
            PhaseSpec {
                phase: LifecyclePhase::Exit,
                timing: PhaseTiming::Fixed { duration: ms(180) },
            },
        ],
    }
}

fn bool_value_spec(default: bool) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Boolean,
        default: Some(Value::Boolean(default)),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: Some("lifecycle signal".to_string()),
    }
}

fn integer_value_spec(default: i64) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Integer,
        default: Some(Value::Integer(default)),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: Some("counter".to_string()),
    }
}

fn trigger_for(source: ValueSource, predicate: ValuePredicate) -> TriggerSpec {
    TriggerSpec {
        condition: TriggerCondition {
            predicate_source: source,
            predicate,
        },
        latch: TriggerLatchPolicy::UntilPhaseReset,
        reset: TriggerResetBoundary::PhaseStart,
        action: TriggerAction::AdvancePhase,
    }
}

fn lifecycle_until(trigger: TriggerSpec, max_duration: Option<DurationSpec>) -> LifecycleSpec {
    let mut lifecycle = fixed_lifecycle();
    lifecycle.phases[1].timing = PhaseTiming::Dwell {
        policy: DwellPolicy::Until {
            trigger,
            max_duration,
        },
    };
    lifecycle
}

fn graph_with_bool_signal() -> tui_vfx_contract::GraphSpec {
    let mut graph = base_graph(literal_source());
    graph.signals.insert(
        SignalId::new("userDismissed"),
        signal("userDismissed", bool_value_spec(false)),
    );
    graph
}

fn graph_with_integer_signal() -> tui_vfx_contract::GraphSpec {
    let mut graph = base_graph(literal_source());
    graph.signals.insert(
        SignalId::new("loopbackCount"),
        signal("loopbackCount", integer_value_spec(0)),
    );
    graph
}

#[test]
fn lifecycle_spec_accepts_fixed_enter_dwell_exit() {
    let graph = base_graph(literal_source());

    assert!(
        fixed_lifecycle()
            .validate(&graph.parameters, &graph.signals)
            .is_ok()
    );
}

#[test]
fn dwell_until_bool_signal_condition_is_representable() {
    let graph = graph_with_bool_signal();
    let trigger = trigger_for(
        ValueSource::Signal {
            id: SignalId::new("userDismissed"),
            fallback: Some(Value::Boolean(false)),
        },
        ValuePredicate::IsTrue,
    );

    assert!(
        lifecycle_until(trigger, None)
            .validate(&graph.parameters, &graph.signals)
            .is_ok()
    );
}

#[test]
fn truthy_loopback_case_maps_to_level_condition_not_edge_only() {
    let graph = graph_with_integer_signal();
    let trigger = trigger_for(
        ValueSource::Signal {
            id: SignalId::new("loopbackCount"),
            fallback: Some(Value::Integer(0)),
        },
        ValuePredicate::Truthy,
    );

    let lifecycle = lifecycle_until(trigger, Some(ms(2_500)));

    assert!(
        lifecycle
            .validate(&graph.parameters, &graph.signals)
            .is_ok()
    );
    let PhaseTiming::Dwell {
        policy: DwellPolicy::Until { trigger, .. },
    } = &lifecycle.phases[1].timing
    else {
        panic!("dwell phase should use trigger policy");
    };
    assert_eq!(trigger.latch, TriggerLatchPolicy::UntilPhaseReset);
    assert_eq!(trigger.reset, TriggerResetBoundary::PhaseStart);
    assert_eq!(trigger.condition.predicate, ValuePredicate::Truthy);
}

#[test]
fn dwell_policy_can_express_max_duration_cap() {
    let graph = graph_with_bool_signal();
    let lifecycle = lifecycle_until(
        trigger_for(
            ValueSource::Signal {
                id: SignalId::new("userDismissed"),
                fallback: Some(Value::Boolean(false)),
            },
            ValuePredicate::IsTrue,
        ),
        Some(ms(5_000)),
    );

    assert!(
        lifecycle
            .validate(&graph.parameters, &graph.signals)
            .is_ok()
    );
}

#[test]
fn trigger_latch_policy_is_explicit() {
    let trigger = trigger_for(
        ValueSource::Literal {
            value: Value::Boolean(true),
        },
        ValuePredicate::IsTrue,
    );

    assert_eq!(trigger.latch, TriggerLatchPolicy::UntilPhaseReset);
}

#[test]
fn trigger_reset_boundary_is_explicit() {
    let trigger = trigger_for(
        ValueSource::Literal {
            value: Value::Boolean(true),
        },
        ValuePredicate::IsTrue,
    );

    assert_eq!(trigger.reset, TriggerResetBoundary::PhaseStart);
}

#[test]
fn typed_predicates_reject_wrong_value_kinds() {
    let graph = graph_with_bool_signal();
    let trigger = trigger_for(
        ValueSource::Signal {
            id: SignalId::new("userDismissed"),
            fallback: Some(Value::Boolean(false)),
        },
        ValuePredicate::NonEmpty,
    );

    assert!(matches!(
        lifecycle_until(trigger, None).validate(&graph.parameters, &graph.signals),
        Err(DescriptorValidationError::PredicateKindMismatch { predicate, actual })
            if predicate == "nonEmpty" && actual == ValueKind::Boolean
    ));
}

#[test]
fn recipe_document_can_include_lifecycle() {
    let mut graph = graph_with_bool_signal();
    graph.parameters.remove(&ParameterId::new("locked"));
    let recipe = RecipeDocument {
        id: RecipeId::new("lifecycleRecipe"),
        version: "3.1".to_string(),
        metadata: RecipeMetadata {
            title: Some("Lifecycle Recipe".to_string()),
            description: Some("I0 recipe lifecycle proof.".to_string()),
            authors: vec!["new-kernel".to_string()],
            expected_visual: None,
            tags: vec!["lifecycle".to_string()],
        },
        lifecycle: Some(lifecycle_until(
            trigger_for(
                ValueSource::Signal {
                    id: SignalId::new("userDismissed"),
                    fallback: Some(Value::Boolean(false)),
                },
                ValuePredicate::IsTrue,
            ),
            Some(ms(5_000)),
        )),
        transitions: BTreeMap::new(),
        assets: BTreeMap::new(),
        descriptor_packs: vec![],
        source_descriptors: BTreeMap::new(),
        sources: BTreeMap::new(),
        graph,
        scenes: vec![],
        intent: None,
    };

    assert!(recipe.validate().is_ok());
}

#[test]
fn lifecycle_trigger_rejects_graph_value_sources() {
    let graph = base_graph(literal_source());
    let trigger = trigger_for(
        ValueSource::GraphValue {
            id: GraphValueId::new("runtimeOutput"),
            fallback: Some(Value::Number(0.0)),
        },
        ValuePredicate::NonZero,
    );

    assert!(matches!(
        lifecycle_until(trigger, None).validate(&graph.parameters, &graph.signals),
        Err(DescriptorValidationError::RecipeLifecycleGraphValueSourceNotAllowed { id })
            if id.as_str() == "runtimeOutput"
    ));
}

#[test]
fn truthy_predicate_documents_allowed_value_kinds() {
    for kind in [
        ValueKind::Boolean,
        ValueKind::Integer,
        ValueKind::Number,
        ValueKind::String,
        ValueKind::Text,
        ValueKind::Color,
        ValueKind::Duration,
    ] {
        assert!(
            ValuePredicate::Truthy.validate_for_kind(kind).is_ok(),
            "truthy should be valid for {kind:?}"
        );
    }
}

#[test]
fn truthy_predicate_rejects_kinds_without_truth_rules() {
    for kind in [
        ValueKind::Null,
        ValueKind::Enum,
        ValueKind::Role,
        ValueKind::Scope,
        ValueKind::Rect,
    ] {
        assert!(matches!(
            ValuePredicate::Truthy.validate_for_kind(kind),
            Err(DescriptorValidationError::PredicateKindMismatch { predicate, actual })
                if predicate == "truthy" && actual == kind
        ));
    }
}

#[test]
fn dwell_policy_serializes_max_duration_as_camel_case() {
    let policy = DwellPolicy::Until {
        trigger: trigger_for(
            ValueSource::Literal {
                value: Value::Boolean(true),
            },
            ValuePredicate::IsTrue,
        ),
        max_duration: Some(ms(5000)),
    };

    let json = serde_json::to_value(policy).expect("dwell policy serializes");

    assert!(json.get("maxDuration").is_some());
    assert!(json.get("max_duration").is_none());
}

#[test]
fn dwell_policy_schema_uses_max_duration_wire_name() {
    let schema = schemars::schema_for!(DwellPolicy);
    let json = serde_json::to_string(&schema).expect("schema serializes");

    assert!(json.contains("maxDuration"));
    assert!(!json.contains("max_duration"));
}

#[test]
fn vocabulary_mentions_trigger_gate_loopback_and_effect_schedule_distinctions() {
    let vocabulary = std::fs::read_to_string("../../docs/VOCABULARY.md")
        .expect("vocabulary document should be readable from contract crate");

    assert!(vocabulary.contains("Trigger"));
    assert!(vocabulary.contains("Gate"));
    assert!(vocabulary.contains("Loopback"));
    assert!(vocabulary.contains("Effect-local schedule"));
    assert!(vocabulary.contains("Trigger ≠ Gate"));
    assert!(vocabulary.contains("Trigger ≠ Binding"));
    assert!(vocabulary.contains("Trigger ≠ Loopback"));
}

// <FILE>crates/tui-vfx-contract/tests/test_lifecycle_contract.rs</FILE> - <DESC>Recipe lifecycle, time, and trigger contract tests</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
