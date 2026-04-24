// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_filter.rs</FILE> - <DESC>Tests for TraceFilter OR-selectors, stage mask, frame range, time range</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests that TraceFilter combines multiple selectors with OR semantics, gates on StageMask, frame range (u64) and time_ms range (u64). A filter with ALL stages + All selector + unbounded ranges is a pass-all sentinel.</WCTX>
// <CLOG>0.1.0: initial red-phase filter tests covering OR across selectors, stage-mask gating, frame range, and time range.</CLOG>

use tui_vfx_debug::inspection::{StageMask, TraceEnvelope, TraceEvent, TraceFilter, TraceSelector};
use tui_vfx_types::{Cell, LayerId, RecipeId, RoleTag};

fn env(event: TraceEvent, frame_no: u64, t_ms: u64, recipe: Option<&str>) -> TraceEnvelope {
    TraceEnvelope {
        event,
        frame_no,
        t_ms,
        recipe_id: recipe.map(RecipeId::from),
        seq_in_frame: 0,
    }
}

fn cell(x: u16, y: u16) -> TraceEvent {
    TraceEvent::CellRendered {
        x,
        y,
        final_cell: Cell::default(),
    }
}

fn layer_painted(layer: &str, role: RoleTag) -> TraceEvent {
    TraceEvent::LayerCellPainted {
        layer_id: LayerId::from(layer),
        x: 0,
        y: 0,
        glyph: '.',
        role,
    }
}

#[test]
fn accepts_all_is_pass_all() {
    let f = TraceFilter::accept_all();
    assert!(f.accepts(&env(cell(0, 0), 0, 0, None)));
    assert!(f.accepts(&env(
        layer_painted("l", RoleTag::Border),
        10,
        1000,
        Some("r")
    )));
}

#[test]
fn or_semantics_across_selectors() {
    // Match if (Cell 3,5) OR (Role Border) — but stage mask must also pass.
    let f = TraceFilter {
        selectors: vec![
            TraceSelector::Cell { x: 3, y: 5 },
            TraceSelector::Role(RoleTag::Border),
        ],
        stages: StageMask::ALL,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    assert!(f.accepts(&env(cell(3, 5), 0, 0, None)));
    assert!(f.accepts(&env(layer_painted("l", RoleTag::Border), 0, 0, None)));
    assert!(!f.accepts(&env(cell(4, 5), 0, 0, None)));
    assert!(!f.accepts(&env(layer_painted("l", RoleTag::Text), 0, 0, None)));
}

#[test]
fn stage_mask_gating() {
    let f = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::PIPELINE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    // CellRendered is a pipeline event: passes.
    assert!(f.accepts(&env(cell(0, 0), 0, 0, None)));
    // LayerStarted is composition: does not pass.
    let layer_ev = TraceEvent::LayerStarted {
        layer_id: LayerId::from("l"),
        z: 0,
        source_kind: "scene".into(),
        target_rect: tui_vfx_types::Rect::new(0, 0, 1, 1),
    };
    assert!(!f.accepts(&env(layer_ev, 0, 0, None)));
}

#[test]
fn frame_range_gates() {
    let f = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::ALL,
        frames: 5..10,
        time_ms: 0..u64::MAX,
    };
    assert!(!f.accepts(&env(cell(0, 0), 4, 0, None)));
    assert!(f.accepts(&env(cell(0, 0), 5, 0, None)));
    assert!(f.accepts(&env(cell(0, 0), 9, 0, None)));
    assert!(!f.accepts(&env(cell(0, 0), 10, 0, None)));
}

#[test]
fn time_range_gates() {
    let f = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::ALL,
        frames: 0..u64::MAX,
        time_ms: 100..500,
    };
    assert!(!f.accepts(&env(cell(0, 0), 0, 99, None)));
    assert!(f.accepts(&env(cell(0, 0), 0, 100, None)));
    assert!(f.accepts(&env(cell(0, 0), 0, 499, None)));
    assert!(!f.accepts(&env(cell(0, 0), 0, 500, None)));
}

#[test]
fn stage_mask_none_rejects_everything() {
    let f = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::NONE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    assert!(!f.accepts(&env(cell(0, 0), 0, 0, None)));
    assert!(!f.accepts(&env(layer_painted("l", RoleTag::Border), 0, 0, None)));
}

#[test]
fn empty_selectors_reject_everything() {
    let f = TraceFilter {
        selectors: vec![],
        stages: StageMask::ALL,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    assert!(!f.accepts(&env(cell(0, 0), 0, 0, None)));
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_filter.rs</FILE> - <DESC>Tests for TraceFilter semantics</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
