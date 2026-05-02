// <FILE>crates/tui-vfx-contract/tests/test_transition_contract.rs</FILE> - <DESC>Native v3.1 transition contract tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>V3.1 native motion/compositing language: transitions are first-class schema objects, not effect-chain aliases.</WCTX>
// <CLOG>0.2.0: MINOR — cover content/style/path/blinds tracks and reduced-motion cycle validation.
// 0.1.0: INIT — assert canonical transition tracks, preset intent, and no generic type field.</CLOG>

use std::collections::BTreeMap;

use schemars::{Schema, schema_for};
use tui_vfx_contract::{
    DescriptorValidationError, DurationSpec, EasingSpec, GraphId, GraphSpec, LifecyclePhase,
    NamedEasing, RecipeDocument, RecipeId, RecipeMetadata, ReducedMotionKind, ReducedMotionPolicy,
    SceneId, ScopeSpec, SignalId, StyleColorSource, TransitionBlindsOrientation,
    TransitionCascadeOrder, TransitionCornerArcMode, TransitionDistanceMetric, TransitionEdge,
    TransitionFocal, TransitionIntent, TransitionInterruption, TransitionMaterializePattern,
    TransitionMotionPath, TransitionMotionSampling, TransitionPreset, TransitionSpec,
    TransitionSubjectRef, TransitionSubjects, TransitionTextCursor, TransitionTextCursorWake,
    TransitionTiming, TransitionTrack, TransitionTrackSubject, TransitionTravelDirection,
    TransitionVariant, TransitionVariantCondition, TransitionVisibilityGeometry, Value,
    ValueSource, VisibilityIrisShape,
};

#[test]
fn transition_spec_preserves_preset_intent_and_executes_tracks() {
    let transition = iris_transition();

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_json_object_has_no_key_named_type(&json);
    assert_eq!(json["intent"]["kind"], "preset");
    assert_eq!(json["intent"]["preset"], "iris");
    assert_eq!(json["tracks"][0]["kind"], "visibility.iris");
    assert_eq!(json["tracks"][0]["focal"]["x"]["kind"], "signal");
    assert_eq!(json["tracks"][0]["edge"]["featherCells"], 1);
    assert_eq!(json["activePhases"][0], "enter");
    assert_eq!(json["interruption"], "reverseFromCurrent");
    assert_eq!(json["reducedMotion"]["policy"], "substitute");
}

#[test]
fn transition_schema_contains_native_track_taxonomy() {
    let schema: Schema = schema_for!(TransitionSpec);
    let json = serde_json::to_string_pretty(&schema).expect("schema serializes");

    assert!(json.contains("Native v3.1 state-change composition interval"));
    assert!(json.contains("visibility.iris"));
    assert!(json.contains("opacity.fade"));
    assert!(json.contains("relation.crossfade"));
    assert!(json.contains("motion.slide"));
    assert!(json.contains("motion.path"));
    assert!(json.contains("visibility.blinds"));
    assert!(json.contains("visibility.materialize"));
    assert!(json.contains("content.splitFlap"));
    assert!(json.contains("content.typewriter"));
    assert!(json.contains("style.glistenBand"));
    assert!(json.contains("style.colorFade"));
    assert!(json.contains("steps"));
    assert!(json.contains("Required policy for superseded interactive transitions"));
    assert!(json.contains("Required accessibility behavior for reduced-motion contexts"));
    assert!(json.contains("Optional generic variants for reduced-motion, capability fallback"));
    assert!(json.contains("Preset intent preserved after author shorthand canonicalization"));
}

#[test]
fn transition_track_fields_use_qualified_names() {
    let mut transition = iris_transition();
    transition.tracks = vec![TransitionTrack::RelationPush {
        travel_direction: TransitionTravelDirection::Left,
        transition_progress: None,
    }];

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_eq!(json["tracks"][0]["kind"], "relation.push");
    assert_eq!(json["tracks"][0]["travelDirection"], "left");
    assert!(json["tracks"][0].get("direction").is_none());
}

#[test]
fn transition_variants_are_generic_engine_conditions() {
    let mut transition = iris_transition();
    transition.variants = vec![TransitionVariant {
        when: TransitionVariantCondition::CapabilityUnavailable {
            capability: "glyphSet.braille".to_string(),
        },
        use_transition: tui_vfx_contract::TransitionId::new("stippledFallback"),
    }];

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_eq!(json["variants"][0]["when"]["kind"], "capabilityUnavailable");
    assert_eq!(json["variants"][0]["useTransition"], "stippledFallback");
}

#[test]
fn transition_tracks_cover_recipe_oracle_shapes_without_effect_chain_translation() {
    let transition = TransitionSpec {
        id: tui_vfx_contract::TransitionId::new("midcenturyEnter"),
        tracks: vec![
            TransitionTrack::MotionPath {
                subject: TransitionTrackSubject::To,
                path: TransitionMotionPath::Arc {
                    bulge: ValueSource::Literal {
                        value: Value::Number(0.22),
                    },
                },
                sampling: Some(TransitionMotionSampling::RoundToCell),
                timing: None,
                transition_progress: None,
            },
            TransitionTrack::VisibilityBlinds {
                subject: TransitionTrackSubject::To,
                orientation: TransitionBlindsOrientation::Horizontal,
                count: ValueSource::Literal {
                    value: Value::Integer(3),
                },
                timing: None,
                scope: None,
                transition_progress: None,
            },
            TransitionTrack::StyleGlistenBand {
                subject: TransitionTrackSubject::To,
                scope: Some(ScopeSpec::All),
                band_width: ValueSource::Literal {
                    value: Value::Integer(6),
                },
                sweep_rate: ValueSource::Literal {
                    value: Value::Number(0.35),
                },
                angle_degrees: ValueSource::Literal {
                    value: Value::Number(25.0),
                },
                head_color: None,
                tail_color: None,
                timing: None,
                transition_progress: None,
            },
            TransitionTrack::ContentSplitFlap {
                subject: TransitionTrackSubject::To,
                scope: Some(ScopeSpec::All),
                flap_rate: ValueSource::Literal {
                    value: Value::Number(1.0),
                },
                cycles: ValueSource::Literal {
                    value: Value::Number(0.5),
                },
                charset: Some("uppercase".to_string()),
                cascade: Some(ValueSource::Signal {
                    id: SignalId::new("flapCascade"),
                    fallback: Some(Value::Number(0.05)),
                }),
                order: Some(TransitionCascadeOrder::LeftToRight),
                timing: None,
                transition_progress: None,
            },
            TransitionTrack::ContentTypewriter {
                subject: TransitionTrackSubject::To,
                scope: Some(ScopeSpec::All),
                typing_rate_variance: Some(ValueSource::Signal {
                    id: SignalId::new("typingJitter"),
                    fallback: Some(Value::Number(0.0)),
                }),
                cursor: Some(TransitionTextCursor {
                    character: "▌".to_string(),
                    wake: TransitionTextCursorWake::Off,
                }),
                timing: None,
                transition_progress: None,
            },
            TransitionTrack::StyleColorFade {
                subject: TransitionTrackSubject::To,
                channel_target: Some("both".to_string()),
                from_color: StyleColorSource::Canvas,
                to_color: StyleColorSource::Current,
                timing: None,
                scope: None,
                transition_progress: None,
            },
        ],
        ..iris_transition()
    };

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_eq!(json["tracks"][0]["kind"], "motion.path");
    assert_eq!(json["tracks"][0]["sampling"], "roundToCell");
    assert_eq!(json["tracks"][1]["kind"], "visibility.blinds");
    assert_eq!(json["tracks"][1]["orientation"], "horizontal");
    assert_eq!(json["tracks"][2]["kind"], "style.glistenBand");
    assert_eq!(json["tracks"][2]["bandWidth"]["value"]["value"], 6);
    assert_eq!(json["tracks"][3]["kind"], "content.splitFlap");
    assert_eq!(json["tracks"][3]["order"], "leftToRight");
    assert_eq!(json["tracks"][4]["kind"], "content.typewriter");
    assert_eq!(json["tracks"][4]["cursor"]["wake"], "off");
    assert_eq!(json["tracks"][5]["kind"], "style.colorFade");
    assert_eq!(json["tracks"][5]["fromColor"]["kind"], "canvas");
    assert_json_object_has_no_key_named_type(&json);
}

#[test]
fn visibility_tracks_can_use_structured_corner_and_materialize_geometry() {
    let mut transition = iris_transition();
    transition.tracks = vec![
        TransitionTrack::VisibilityWipe {
            subject: TransitionTrackSubject::To,
            reveal_direction: None,
            geometry: Some(TransitionVisibilityGeometry::CornerArc {
                corner_arc_mode: TransitionCornerArcMode::InToCorner,
                corner: tui_vfx_contract::SceneAnchor::BottomLeft,
                metric: TransitionDistanceMetric::Euclidean,
            }),
            angle_degrees: None,
            edge: None,
            timing: None,
            scope: None,
            transition_progress: None,
        },
        TransitionTrack::VisibilityMaterialize {
            subject: TransitionTrackSubject::To,
            from_anchor: Some(tui_vfx_contract::SceneAnchor::TopLeft),
            pattern: TransitionMaterializePattern::Noise,
            seed: Some(ValueSource::Literal {
                value: Value::Integer(1337),
            }),
            chunk_size: Some(ValueSource::Literal {
                value: Value::Integer(2),
            }),
            noise_amount: Some(ValueSource::Literal {
                value: Value::Number(0.3),
            }),
            edge: None,
            timing: None,
            scope: None,
            transition_progress: None,
        },
    ];

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_eq!(json["tracks"][0]["kind"], "visibility.wipe");
    assert_eq!(json["tracks"][0]["geometry"]["kind"], "cornerArc");
    assert_eq!(json["tracks"][0]["geometry"]["cornerArcMode"], "inToCorner");
    assert_eq!(json["tracks"][1]["kind"], "visibility.materialize");
    assert_eq!(json["tracks"][1]["fromAnchor"], "topLeft");
}

#[test]
fn transition_progress_is_a_normalized_track_value_source() {
    let mut transition = iris_transition();
    transition.tracks = vec![TransitionTrack::OpacityFade {
        subject: TransitionTrackSubject::To,
        opacity_from: Some(ValueSource::Literal {
            value: Value::Number(0.0),
        }),
        opacity_to: Some(ValueSource::Literal {
            value: Value::Number(1.0),
        }),
        timing: None,
        scope: None,
        transition_progress: Some(ValueSource::Signal {
            id: SignalId::new("transitionProgress"),
            fallback: Some(Value::Number(0.0)),
        }),
    }];

    let json = serde_json::to_value(&transition).expect("transition serializes");
    assert_eq!(json["tracks"][0]["kind"], "opacity.fade");
    assert_eq!(json["tracks"][0]["transitionProgress"]["kind"], "signal");
}

#[test]
fn reduced_motion_substitute_requires_terminal_acyclic_replacement() {
    let mut recipe = recipe_with_transitions(vec![
        transition_with_reduced_motion(
            "enter",
            ReducedMotionPolicy {
                policy: ReducedMotionKind::Substitute,
                transition: Some(tui_vfx_contract::TransitionId::new("enterReduced")),
            },
        ),
        transition_with_reduced_motion(
            "enterReduced",
            ReducedMotionPolicy {
                policy: ReducedMotionKind::None,
                transition: None,
            },
        ),
    ]);

    assert!(recipe.validate().is_ok());

    recipe
        .transitions
        .get_mut(&tui_vfx_contract::TransitionId::new("enterReduced"))
        .unwrap()
        .reduced_motion = ReducedMotionPolicy {
        policy: ReducedMotionKind::Substitute,
        transition: Some(tui_vfx_contract::TransitionId::new("enter")),
    };

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::ReducedMotionTransitionCycle { transition })
            if transition.as_str() == "enter"
    ));
}

#[test]
fn reduced_motion_policy_shape_matches_selected_kind() {
    let recipe = recipe_with_transitions(vec![transition_with_reduced_motion(
        "enter",
        ReducedMotionPolicy {
            policy: ReducedMotionKind::Substitute,
            transition: None,
        },
    )]);

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::MissingReducedMotionTransition { transition })
            if transition.as_str() == "enter"
    ));

    let recipe = recipe_with_transitions(vec![transition_with_reduced_motion(
        "enter",
        ReducedMotionPolicy {
            policy: ReducedMotionKind::None,
            transition: Some(tui_vfx_contract::TransitionId::new("unused")),
        },
    )]);

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnexpectedReducedMotionTransition { transition, referenced })
            if transition.as_str() == "enter" && referenced.as_str() == "unused"
    ));
}

fn iris_transition() -> TransitionSpec {
    TransitionSpec {
        id: tui_vfx_contract::TransitionId::new("modalOpen"),
        intent: Some(TransitionIntent::Preset {
            preset: TransitionPreset::Iris,
        }),
        subjects: TransitionSubjects {
            from: TransitionSubjectRef::Empty,
            to: TransitionSubjectRef::Scene {
                id: SceneId::new("modalScene"),
            },
            shared: vec![],
        },
        timing: TransitionTiming {
            duration: Some(DurationSpec::Milliseconds { value: 300 }),
            delay: None,
            easing: Some(EasingSpec::Named {
                value: NamedEasing::InOutCubic,
            }),
            stagger: None,
        },
        scope: Some(ScopeSpec::All),
        active_phases: vec![LifecyclePhase::Enter],
        tracks: vec![TransitionTrack::VisibilityIris {
            subject: TransitionTrackSubject::To,
            shape: VisibilityIrisShape::Circle,
            focal: Some(TransitionFocal {
                x: ValueSource::Signal {
                    id: SignalId::new("hoveredX"),
                    fallback: Some(Value::Number(0.5)),
                },
                y: ValueSource::Signal {
                    id: SignalId::new("hoveredY"),
                    fallback: Some(Value::Number(0.5)),
                },
            }),
            edge: Some(TransitionEdge::Soft { feather_cells: 1 }),
            timing: None,
            scope: None,
            transition_progress: None,
        }],
        interruption: TransitionInterruption::ReverseFromCurrent,
        reduced_motion: ReducedMotionPolicy {
            policy: ReducedMotionKind::Substitute,
            transition: Some(tui_vfx_contract::TransitionId::new("crossfadeFallback")),
        },
        variants: vec![TransitionVariant {
            when: TransitionVariantCondition::ReducedMotionRequested,
            use_transition: tui_vfx_contract::TransitionId::new("crossfadeFallback"),
        }],
    }
}

fn transition_with_reduced_motion(id: &str, reduced_motion: ReducedMotionPolicy) -> TransitionSpec {
    TransitionSpec {
        id: tui_vfx_contract::TransitionId::new(id),
        reduced_motion,
        variants: vec![],
        ..iris_transition()
    }
}

fn recipe_with_transitions(transitions: Vec<TransitionSpec>) -> RecipeDocument {
    RecipeDocument {
        id: RecipeId::new("transitionRecipe"),
        version: "3.1".to_string(),
        metadata: RecipeMetadata {
            title: None,
            description: None,
            authors: vec![],
            expected_visual: None,
            tags: vec![],
        },
        lifecycle: None,
        transitions: transitions
            .into_iter()
            .map(|transition| (transition.id.clone(), transition))
            .collect(),
        assets: BTreeMap::new(),
        descriptor_packs: vec![],
        source_descriptors: BTreeMap::new(),
        sources: BTreeMap::new(),
        graph: GraphSpec {
            id: GraphId::new("mainGraph"),
            version: "3.1".to_string(),
            parameters: BTreeMap::new(),
            signals: BTreeMap::new(),
            bindings: vec![],
            effects: BTreeMap::new(),
            nodes: BTreeMap::new(),
            order: vec![],
            topology: None,
        },
        scenes: vec![],
        intent: None,
    }
}

fn assert_json_object_has_no_key_named_type(value: &serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            assert!(
                !map.contains_key("type"),
                "canonical transition JSON must not use generic type keys"
            );
            for child in map.values() {
                assert_json_object_has_no_key_named_type(child);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                assert_json_object_has_no_key_named_type(child);
            }
        }
        _ => {}
    }
}

// <FILE>crates/tui-vfx-contract/tests/test_transition_contract.rs</FILE> - <DESC>Native v3.1 transition contract tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
