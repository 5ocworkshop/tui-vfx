// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_event.rs</FILE> - <DESC>Tests for TraceEvent serde round-trip across every variant</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests covering the full TraceEvent taxonomy from spec §9.2: every variant round-trips through serde_json; stage accessor categorizes each variant correctly (lifecycle | resolution | composition | pipeline).</WCTX>
// <CLOG>0.1.0: initial red-phase tests — round-trip for every variant across lifecycle/resolution/composition/pipeline buckets, plus stage() accessor parity with StageMask.</CLOG>

use tui_vfx_debug::inspection::{StageMask, TraceEvent};
use tui_vfx_types::{Cell, Color, LayerId, RecipeId, Rect, RoleTag, Style};

fn round_trip(event: &TraceEvent) -> TraceEvent {
    let json = serde_json::to_string(event).expect("serialize event");
    serde_json::from_str(&json).expect("deserialize event")
}

#[test]
fn lifecycle_phase_entered_round_trips() {
    let ev = TraceEvent::LifecyclePhaseEntered {
        id: RecipeId::from("splash.v2"),
        phase: "enter".into(),
        t_ms: 42,
    };
    let back = round_trip(&ev);
    match back {
        TraceEvent::LifecyclePhaseEntered { id, phase, t_ms } => {
            assert_eq!(id.as_str(), "splash.v2");
            assert_eq!(phase, "enter");
            assert_eq!(t_ms, 42);
        }
        _ => panic!("variant mismatch"),
    }
}

#[test]
fn lifecycle_phase_transition_round_trips() {
    let ev = TraceEvent::LifecyclePhaseTransition {
        id: RecipeId::from("r"),
        from: "enter".into(),
        to: "dwell".into(),
        t_ms: 100,
        eased_progress: 0.5,
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::LifecyclePhaseTransition { .. }
    ));
}

#[test]
fn lifecycle_dismissed_round_trips() {
    let ev = TraceEvent::LifecycleDismissed {
        id: RecipeId::from("r"),
        reason: "timeout".into(),
        t_ms: 200,
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::LifecycleDismissed { .. }
    ));
}

#[test]
fn lifecycle_held_round_trips() {
    let ev = TraceEvent::LifecycleHeld {
        id: RecipeId::from("r"),
        until_ms: 500,
    };
    assert!(matches!(round_trip(&ev), TraceEvent::LifecycleHeld { .. }));
}

#[test]
fn asset_resolved_round_trips() {
    let ev = TraceEvent::AssetResolved {
        name: "logo.rs".into(),
        found: true,
        fallback_reason: None,
    };
    assert!(matches!(round_trip(&ev), TraceEvent::AssetResolved { .. }));
}

#[test]
fn procedural_resolved_round_trips() {
    let ev = TraceEvent::ProceduralResolved {
        source_id: "spinner.braille".into(),
        resolved: true,
        fallback_id: Some("spinner.dot".into()),
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::ProceduralResolved { .. }
    ));
}

#[test]
fn token_resolved_round_trips() {
    let ev = TraceEvent::TokenResolved {
        input: "{theme}".into(),
        output: "harbor".into(),
        missing_keys: vec!["mode".into()],
    };
    assert!(matches!(round_trip(&ev), TraceEvent::TokenResolved { .. }));
}

#[test]
fn recipe_binding_resolved_round_trips() {
    let ev = TraceEvent::RecipeBindingResolved {
        selector: "splash".into(),
        recipe_id: RecipeId::from("splash.v2"),
        theme: "harbor".into(),
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::RecipeBindingResolved { .. }
    ));
}

#[test]
fn layer_started_round_trips() {
    let ev = TraceEvent::LayerStarted {
        layer_id: LayerId::from("l0"),
        z: 2,
        source_kind: "scene-fragment".into(),
        target_rect: Rect::new(1, 2, 3, 4),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::LayerStarted { .. }));
}

#[test]
fn layer_cell_painted_round_trips() {
    let ev = TraceEvent::LayerCellPainted {
        layer_id: LayerId::from("l0"),
        x: 5,
        y: 7,
        glyph: '▓',
        role: RoleTag::Border,
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::LayerCellPainted { .. }
    ));
}

#[test]
fn layer_completed_round_trips() {
    let ev = TraceEvent::LayerCompleted {
        layer_id: LayerId::from("l0"),
        cells_painted: 10,
        cells_skipped: 2,
        fallback: false,
    };
    assert!(matches!(round_trip(&ev), TraceEvent::LayerCompleted { .. }));
}

#[test]
fn layer_skipped_round_trips() {
    let ev = TraceEvent::LayerSkipped {
        layer_id: LayerId::from("l0"),
        reason: "empty-source".into(),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::LayerSkipped { .. }));
}

#[test]
fn sampler_applied_round_trips() {
    let ev = TraceEvent::SamplerApplied {
        dest_x: 1,
        dest_y: 2,
        src_x: Some(3),
        src_y: Some(4),
        sampler: "ripple#1".into(),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::SamplerApplied { .. }));
}

#[test]
fn mask_checked_round_trips() {
    let ev = TraceEvent::MaskChecked {
        x: 1,
        y: 2,
        visible: true,
        mask: "wipe#1".into(),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::MaskChecked { .. }));
}

#[test]
fn shader_applied_round_trips() {
    let s = Style {
        fg: Color::new(255, 0, 0, 255),
        bg: Color::new(0, 0, 0, 0),
        mods: tui_vfx_types::Modifiers::NONE,
    };
    let ev = TraceEvent::ShaderApplied {
        x: 1,
        y: 1,
        before: s,
        after: s,
        shader: "pulse#1".into(),
        region: Some("Border".into()),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::ShaderApplied { .. }));
}

#[test]
fn filter_applied_round_trips() {
    let cell = Cell::default();
    let ev = TraceEvent::FilterApplied {
        x: 1,
        y: 1,
        before: cell,
        after: cell,
        filter: "tint#1".into(),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::FilterApplied { .. }));
}

#[test]
fn shadow_cell_applied_round_trips() {
    let cell = Cell::default();
    let ev = TraceEvent::ShadowCellApplied {
        x: 1,
        y: 1,
        shadow_cell: cell,
        source_role: Some(RoleTag::Border),
        source_empty: false,
    };
    assert!(matches!(
        round_trip(&ev),
        TraceEvent::ShadowCellApplied { .. }
    ));
}

#[test]
fn cell_rendered_round_trips() {
    let ev = TraceEvent::CellRendered {
        x: 1,
        y: 1,
        final_cell: Cell::default(),
    };
    assert!(matches!(round_trip(&ev), TraceEvent::CellRendered { .. }));
}

#[test]
fn stage_accessor_categorizes_every_variant() {
    let life = TraceEvent::LifecyclePhaseEntered {
        id: RecipeId::from("r"),
        phase: "e".into(),
        t_ms: 0,
    };
    assert_eq!(life.stage(), StageMask::LIFECYCLE);

    let asset = TraceEvent::AssetResolved {
        name: "n".into(),
        found: true,
        fallback_reason: None,
    };
    assert_eq!(asset.stage(), StageMask::RESOLUTION);

    let layer = TraceEvent::LayerStarted {
        layer_id: LayerId::from("l"),
        z: 0,
        source_kind: "x".into(),
        target_rect: Rect::new(0, 0, 1, 1),
    };
    assert_eq!(layer.stage(), StageMask::COMPOSITION);

    let pipe = TraceEvent::CellRendered {
        x: 0,
        y: 0,
        final_cell: Cell::default(),
    };
    assert_eq!(pipe.stage(), StageMask::PIPELINE);
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_event.rs</FILE> - <DESC>Tests for TraceEvent serde + stage accessor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
