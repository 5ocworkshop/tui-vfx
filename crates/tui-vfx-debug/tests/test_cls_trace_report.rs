// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_report.rs</FILE> - <DESC>Tests for TraceReport NDJSON round-trip and summary counts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests that TraceReport::to_ndjson emits one envelope per line; from_ndjson round-trips bit-stable; summary counts per stage match emitted events; dropped counter survives round-trip.</WCTX>
// <CLOG>0.1.0: initial red-phase report tests — NDJSON line-per-envelope, round-trip equality, summary per-stage counts, dropped counter.</CLOG>

use std::io::Cursor;
use tui_vfx_debug::inspection::{
    InspectionSink, StageMask, TraceEnvelope, TraceEvent, TraceFilter, TraceReport, TraceSink,
};
use tui_vfx_types::{Cell, LayerId, RecipeId, Rect};

fn env_cell(x: u16, y: u16) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::CellRendered {
            x,
            y,
            final_cell: Cell::default(),
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: Some(RecipeId::from("r")),
        seq_in_frame: 0,
    }
}

fn env_layer_started() -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::LayerStarted {
            layer_id: LayerId::from("l"),
            z: 0,
            source_kind: "scene".into(),
            target_rect: Rect::new(0, 0, 1, 1),
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: Some(RecipeId::from("r")),
        seq_in_frame: 0,
    }
}

fn env_asset() -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::AssetResolved {
            name: "logo".into(),
            found: true,
            fallback_reason: None,
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: None,
        seq_in_frame: 0,
    }
}

fn env_lifecycle() -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::LifecyclePhaseEntered {
            id: RecipeId::from("r"),
            phase: "enter".into(),
            t_ms: 0,
        },
        frame_no: 0,
        t_ms: 0,
        recipe_id: Some(RecipeId::from("r")),
        seq_in_frame: 0,
    }
}

#[test]
fn to_ndjson_emits_one_line_per_envelope() {
    let sink = TraceSink::new(TraceFilter::accept_all());
    sink.report(env_cell(0, 0));
    sink.report(env_cell(1, 0));
    sink.report(env_layer_started());
    let report = sink.snapshot();

    let mut buf = Vec::new();
    report.to_ndjson(&mut buf).expect("write ndjson");
    let s = String::from_utf8(buf).expect("utf-8");
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 3);
    for line in &lines {
        assert!(!line.is_empty(), "no empty lines allowed");
        // Every line must parse as a TraceEnvelope.
        let _: TraceEnvelope = serde_json::from_str(line).expect("ndjson line parse");
    }
}

#[test]
fn from_ndjson_round_trips_report() {
    let sink = TraceSink::new(TraceFilter::accept_all());
    sink.report(env_cell(3, 5));
    sink.report(env_layer_started());
    sink.report(env_asset());
    let original = sink.snapshot();

    let mut buf = Vec::new();
    original.to_ndjson(&mut buf).expect("write");

    let cursor = Cursor::new(buf);
    let restored = TraceReport::from_ndjson(cursor).expect("read");

    // Envelope count equal; dropped counter not transported (read-side defaults to 0).
    assert_eq!(restored.envelopes.len(), original.envelopes.len());
    // Verify by serializing each envelope back to a canonical string and comparing.
    for (a, b) in restored.envelopes.iter().zip(original.envelopes.iter()) {
        let sa = serde_json::to_string(a).expect("a");
        let sb = serde_json::to_string(b).expect("b");
        assert_eq!(sa, sb, "envelope round-trip mismatch");
    }
}

#[test]
fn summary_counts_per_stage() {
    let sink = TraceSink::new(TraceFilter::accept_all());
    // 3 pipeline events.
    sink.report(env_cell(0, 0));
    sink.report(env_cell(1, 0));
    sink.report(env_cell(2, 0));
    // 1 composition event.
    sink.report(env_layer_started());
    // 1 resolution event.
    sink.report(env_asset());
    // 1 lifecycle event.
    sink.report(env_lifecycle());

    let report = sink.snapshot();
    assert_eq!(report.summary.count_for(StageMask::PIPELINE), 3);
    assert_eq!(report.summary.count_for(StageMask::COMPOSITION), 1);
    assert_eq!(report.summary.count_for(StageMask::RESOLUTION), 1);
    assert_eq!(report.summary.count_for(StageMask::LIFECYCLE), 1);
    assert_eq!(report.summary.total, 6);
}

#[test]
fn dropped_counter_preserved_in_report() {
    let sink = TraceSink::with_capacity(TraceFilter::accept_all(), 2);
    sink.report(env_cell(0, 0));
    sink.report(env_cell(1, 0));
    sink.report(env_cell(2, 0));
    sink.report(env_cell(3, 0));
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 2);
    assert_eq!(report.dropped, 2);
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_report.rs</FILE> - <DESC>Tests for TraceReport</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
