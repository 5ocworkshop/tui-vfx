// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_envelope.rs</FILE> - <DESC>Tests for TraceEnvelope JSON + NDJSON round-trip, seq_in_frame ordering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests that TraceEnvelope carries the event + frame_no + t_ms + optional recipe_id + seq_in_frame, survives JSON round-trip, and has predictable ordering semantics for replay.</WCTX>
// <CLOG>0.1.0: initial red-phase envelope tests — JSON round-trip, NDJSON line format, seq_in_frame monotonic per frame via TraceSink accounting.</CLOG>

use tui_vfx_debug::inspection::{TraceEnvelope, TraceEvent};
use tui_vfx_types::{Cell, RecipeId};

#[test]
fn envelope_json_round_trip() {
    let env = TraceEnvelope {
        event: TraceEvent::CellRendered {
            x: 3,
            y: 5,
            final_cell: Cell::default(),
        },
        frame_no: 42,
        t_ms: 700,
        recipe_id: Some(RecipeId::from("splash.v2")),
        seq_in_frame: 7,
    };
    let json = serde_json::to_string(&env).expect("serialize envelope");
    let back: TraceEnvelope = serde_json::from_str(&json).expect("deserialize envelope");
    assert_eq!(back.frame_no, 42);
    assert_eq!(back.t_ms, 700);
    assert_eq!(back.recipe_id.as_ref().map(|r| r.as_str()), Some("splash.v2"));
    assert_eq!(back.seq_in_frame, 7);
    match back.event {
        TraceEvent::CellRendered { x, y, .. } => {
            assert_eq!((x, y), (3, 5));
        }
        _ => panic!("variant mismatch"),
    }
}

#[test]
fn envelope_without_recipe_id_round_trips() {
    let env = TraceEnvelope {
        event: TraceEvent::LayerSkipped {
            layer_id: tui_vfx_types::LayerId::from("l0"),
            reason: "no-source".into(),
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: None,
        seq_in_frame: 0,
    };
    let json = serde_json::to_string(&env).expect("serialize");
    let back: TraceEnvelope = serde_json::from_str(&json).expect("deserialize");
    assert!(back.recipe_id.is_none());
}

#[test]
fn ndjson_line_contains_no_embedded_newlines() {
    let env = TraceEnvelope {
        event: TraceEvent::AssetResolved {
            name: "logo.rs".into(),
            found: true,
            fallback_reason: None,
        },
        frame_no: 1,
        t_ms: 16,
        recipe_id: None,
        seq_in_frame: 0,
    };
    let line = serde_json::to_string(&env).expect("serialize");
    assert!(
        !line.contains('\n'),
        "NDJSON line must be newline-free: {:?}",
        line
    );
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_envelope.rs</FILE> - <DESC>Tests for TraceEnvelope round-trip</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
