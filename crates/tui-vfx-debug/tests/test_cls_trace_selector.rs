// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_selector.rs</FILE> - <DESC>Tests for TraceSelector matching and opaque-id equality</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests that each TraceSelector variant matches the correct envelopes: Cell {x,y}, Rect, Role, Layer (opaque LayerId), Recipe (opaque RecipeId), All; mismatched IDs do not match.</WCTX>
// <CLOG>0.1.0: initial red-phase selector-match tests — per-variant positive + negative matches; opaque ID equality; Rect inclusion tests cover cell-center semantics; All matches everything.</CLOG>

use tui_vfx_debug::inspection::{TraceEnvelope, TraceEvent, TraceSelector};
use tui_vfx_types::{Cell, LayerId, RecipeId, Rect, RoleTag};

fn env_cell_rendered(x: u16, y: u16) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::CellRendered {
            x,
            y,
            final_cell: Cell::default(),
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: None,
        seq_in_frame: 0,
    }
}

fn env_layer_started(layer: &str) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::LayerStarted {
            layer_id: LayerId::from(layer),
            z: 0,
            source_kind: "scene".into(),
            target_rect: Rect::new(0, 0, 10, 10),
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: None,
        seq_in_frame: 0,
    }
}

fn env_layer_cell_painted(layer: &str, x: u16, y: u16, role: RoleTag) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::LayerCellPainted {
            layer_id: LayerId::from(layer),
            x,
            y,
            glyph: 'X',
            role,
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: None,
        seq_in_frame: 0,
    }
}

fn env_lifecycle(recipe: &str) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::LifecyclePhaseEntered {
            id: RecipeId::from(recipe),
            phase: "enter".into(),
            t_ms: 0,
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: Some(RecipeId::from(recipe)),
        seq_in_frame: 0,
    }
}

#[test]
fn all_selector_matches_every_envelope() {
    let s = TraceSelector::All;
    assert!(s.matches(&env_cell_rendered(0, 0)));
    assert!(s.matches(&env_layer_started("l")));
    assert!(s.matches(&env_lifecycle("r")));
}

#[test]
fn cell_selector_matches_exact_cell_events_only() {
    let s = TraceSelector::Cell { x: 3, y: 5 };
    assert!(s.matches(&env_cell_rendered(3, 5)));
    assert!(!s.matches(&env_cell_rendered(3, 6)));
    assert!(!s.matches(&env_cell_rendered(4, 5)));
    // Non-cell events should not match a Cell selector.
    assert!(!s.matches(&env_layer_started("l")));
}

#[test]
fn rect_selector_matches_cell_inside_rect_only() {
    let r = Rect::new(2, 2, 4, 4); // covers x in [2..6), y in [2..6)
    let s = TraceSelector::Rect(r);
    assert!(s.matches(&env_cell_rendered(2, 2)));
    assert!(s.matches(&env_cell_rendered(5, 5)));
    assert!(!s.matches(&env_cell_rendered(6, 5)));
    assert!(!s.matches(&env_cell_rendered(5, 6)));
    assert!(!s.matches(&env_cell_rendered(1, 2)));
}

#[test]
fn role_selector_matches_layer_cell_painted_with_role() {
    let s = TraceSelector::Role(RoleTag::Border);
    assert!(s.matches(&env_layer_cell_painted("l", 0, 0, RoleTag::Border)));
    assert!(!s.matches(&env_layer_cell_painted("l", 0, 0, RoleTag::Text)));
    // Events with no role field should not match a Role selector.
    assert!(!s.matches(&env_cell_rendered(0, 0)));
}

#[test]
fn layer_selector_matches_layer_events_with_matching_id() {
    let s = TraceSelector::Layer(LayerId::from("logo"));
    assert!(s.matches(&env_layer_started("logo")));
    assert!(s.matches(&env_layer_cell_painted("logo", 0, 0, RoleTag::Image)));
    assert!(!s.matches(&env_layer_started("card")));
    // Non-layer events should not match.
    assert!(!s.matches(&env_cell_rendered(0, 0)));
}

#[test]
fn recipe_selector_matches_envelope_recipe_id() {
    let s = TraceSelector::Recipe(RecipeId::from("splash.v2"));
    assert!(s.matches(&env_lifecycle("splash.v2")));
    // Different recipe id.
    assert!(!s.matches(&env_lifecycle("splash.v1")));
    // Envelope with no recipe id: does not match Recipe selector.
    assert!(!s.matches(&env_cell_rendered(0, 0)));
}

#[test]
fn opaque_ids_compare_by_content() {
    let a = LayerId::from("x");
    let b = LayerId::from("x".to_string());
    assert_eq!(a, b);
    let ra = RecipeId::from("r");
    let rb = RecipeId::from("r".to_string());
    assert_eq!(ra, rb);
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_selector.rs</FILE> - <DESC>Tests for TraceSelector matching</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
