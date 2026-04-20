// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_sink.rs</FILE> - <DESC>Tests for TraceSink ordering, bounded mode, concurrent emits, short-circuit</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — TDD red tests for the inspection sink: preserves order; bounded mode drops oldest with explicit dropped counter; concurrent emits from multiple threads serialize correctly; StageMask::NONE short-circuits before allocation.</WCTX>
// <CLOG>0.1.0: initial red-phase sink tests — ordering, bounded drop-oldest accounting, concurrent thread safety, NONE short-circuit.</CLOG>

use std::sync::Arc;
use std::thread;
use tui_vfx_debug::inspection::{
    InspectionSink, StageMask, TraceEnvelope, TraceEvent, TraceFilter, TraceSelector, TraceSink,
};
use tui_vfx_types::{Cell, RecipeId};

fn env(x: u16, y: u16, frame: u64, t_ms: u64) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::CellRendered {
            x,
            y,
            final_cell: Cell::default(),
        },
        frame_no: frame,
        t_ms,
        recipe_id: Some(RecipeId::from("r")),
        seq_in_frame: 0,
    }
}

#[test]
fn sink_preserves_insertion_order() {
    let sink = TraceSink::new(TraceFilter::accept_all());
    for i in 0..10u16 {
        sink.report(env(i, 0, 0, i as u64));
    }
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 10);
    for (i, envelope) in report.envelopes.iter().enumerate() {
        assert_eq!(envelope.t_ms, i as u64);
    }
    assert_eq!(report.dropped, 0);
}

#[test]
fn sink_rejects_filtered_events() {
    // Only accept (Cell {x:0,y:0}) — all others dropped by the filter.
    let filter = TraceFilter {
        selectors: vec![TraceSelector::Cell { x: 0, y: 0 }],
        stages: StageMask::ALL,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    let sink = TraceSink::new(filter);
    sink.report(env(0, 0, 0, 0));
    sink.report(env(1, 0, 0, 0));
    sink.report(env(2, 0, 0, 0));
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 1);
    assert_eq!(report.dropped, 0); // filtered != dropped
}

#[test]
fn bounded_mode_drops_oldest_with_dropped_counter() {
    let sink = TraceSink::with_capacity(TraceFilter::accept_all(), 3);
    sink.report(env(0, 0, 0, 0));
    sink.report(env(0, 0, 0, 1));
    sink.report(env(0, 0, 0, 2));
    sink.report(env(0, 0, 0, 3));
    sink.report(env(0, 0, 0, 4));
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 3);
    assert_eq!(report.dropped, 2);
    // Oldest-first drop policy — t_ms 0, 1 are gone; we see 2, 3, 4.
    assert_eq!(report.envelopes[0].t_ms, 2);
    assert_eq!(report.envelopes[1].t_ms, 3);
    assert_eq!(report.envelopes[2].t_ms, 4);
}

#[test]
fn stage_mask_none_short_circuits_without_allocation() {
    // With STAGE mask NONE, accepts_any_stage returns false — emit is a fast no-op.
    let filter = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::NONE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    let sink = TraceSink::new(filter);
    for i in 0..1000u16 {
        sink.report(env(i, 0, 0, i as u64));
    }
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 0);
    assert_eq!(report.dropped, 0);
}

#[test]
fn concurrent_emits_serialize_correctly() {
    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let mut handles = Vec::new();
    for tid in 0..4u16 {
        let s = Arc::clone(&sink);
        handles.push(thread::spawn(move || {
            for i in 0..250u16 {
                s.report(env(tid, i, 0, (tid as u64) * 1000 + i as u64));
            }
        }));
    }
    for h in handles {
        h.join().expect("thread");
    }
    let report = sink.snapshot();
    assert_eq!(report.envelopes.len(), 4 * 250);
    assert_eq!(report.dropped, 0);
}

#[test]
fn sink_is_inspection_sink_trait_object_friendly() {
    // The sink must be usable as &dyn InspectionSink without issue.
    let sink: Arc<dyn InspectionSink> = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    sink.report(env(0, 0, 0, 0));
    // We can only observe via the concrete type; this test exists to prove the Arc<dyn _>
    // form compiles and the blanket Send+Sync bound on the trait is satisfied.
    drop(sink);
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_sink.rs</FILE> - <DESC>Tests for TraceSink</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
