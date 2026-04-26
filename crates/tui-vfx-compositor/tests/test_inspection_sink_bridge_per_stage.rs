// <FILE>crates/tui-vfx-compositor/tests/test_inspection_sink_bridge_per_stage.rs</FILE> - <DESC>Bridge round-trip test for the four new per-stage CompositorInspector callbacks landing in Pipeline observability Unit A</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — confirm InspectionSinkBridge forwards on_stage_entered/on_stage_finished/on_stage_skipped/on_scope_evaluated to the matching TraceEvent variants without losing fields, before the compositor pipeline starts emitting them.</WCTX>
// <CLOG>0.1.0: drive each new bridge callback by hand and assert the underlying TraceSink receives matching TraceEvent::Stage*/ScopeEvaluated envelopes with intact payload.</CLOG>

//! Round-trip test for the four new per-stage `CompositorInspector` callbacks
//! that landed with Pipeline observability Unit A.
//!
//! Drives each callback on `InspectionSinkBridge` directly (no full pipeline
//! run yet — the pipeline emit sites land in US-006) and asserts the
//! underlying `TraceSink` receives matching `TraceEvent` envelopes with
//! intact payload.

use std::sync::Arc;

use tui_vfx_compositor::traits::cls_inspection_sink_bridge::{
    InspectionSinkBridge, TraceFrameContext,
};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_debug::inspection::{
    PipelineSkipReason, PipelineStageKind, RoleHistogram, TraceEvent, TraceFilter, TraceSink,
};

fn install_bridge() -> (InspectionSinkBridge, Arc<TraceSink>) {
    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx = TraceFrameContext::new(0, 0);
    let bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);
    (bridge, sink)
}

fn captured_events(sink: &Arc<TraceSink>) -> Vec<TraceEvent> {
    sink.snapshot()
        .envelopes
        .into_iter()
        .map(|env| env.event)
        .collect()
}

#[test]
fn bridge_forwards_on_stage_entered_to_trace_event() {
    let (mut bridge, sink) = install_bridge();
    bridge.on_stage_entered(
        PipelineStageKind::Shader,
        1,
        "FocusedRowGradient",
        "Role(Text)",
    );

    let events = captured_events(&sink);
    assert_eq!(events.len(), 1, "expected exactly one envelope");
    match &events[0] {
        TraceEvent::StageEntered {
            kind,
            step_id,
            name,
            scope_summary,
        } => {
            assert_eq!(*kind, PipelineStageKind::Shader);
            assert_eq!(*step_id, 1);
            assert_eq!(name, "FocusedRowGradient");
            assert_eq!(scope_summary, "Role(Text)");
        }
        other => panic!("expected StageEntered, got {other:?}"),
    }
}

#[test]
fn bridge_forwards_on_stage_finished_to_trace_event() {
    let (mut bridge, sink) = install_bridge();
    bridge.on_stage_finished(PipelineStageKind::Filter, 2, 17, 4_200);

    let events = captured_events(&sink);
    assert_eq!(events.len(), 1);
    match &events[0] {
        TraceEvent::StageFinished {
            kind,
            step_id,
            cells_modified,
            elapsed_ns,
        } => {
            assert_eq!(*kind, PipelineStageKind::Filter);
            assert_eq!(*step_id, 2);
            assert_eq!(*cells_modified, 17);
            assert_eq!(*elapsed_ns, 4_200);
        }
        other => panic!("expected StageFinished, got {other:?}"),
    }
}

#[test]
fn bridge_forwards_on_stage_skipped_to_trace_event_with_reason() {
    let (mut bridge, sink) = install_bridge();
    let reason = PipelineSkipReason::ScopeMatchedZeroCells {
        predicate: "Role(Text)".to_string(),
        role_histogram: RoleHistogram {
            background: 320,
            ..RoleHistogram::EMPTY
        },
    };
    bridge.on_stage_skipped(PipelineStageKind::Shader, 3, reason.clone());

    let events = captured_events(&sink);
    assert_eq!(events.len(), 1);
    match &events[0] {
        TraceEvent::StageSkipped {
            kind,
            step_id,
            reason: got_reason,
        } => {
            assert_eq!(*kind, PipelineStageKind::Shader);
            assert_eq!(*step_id, 3);
            assert_eq!(*got_reason, reason);
        }
        other => panic!("expected StageSkipped, got {other:?}"),
    }
}

#[test]
fn bridge_forwards_on_scope_evaluated_to_trace_event() {
    let (mut bridge, sink) = install_bridge();
    let histogram = RoleHistogram {
        background: 320,
        ..RoleHistogram::EMPTY
    };
    bridge.on_scope_evaluated(1, 0, 320, histogram);

    let events = captured_events(&sink);
    assert_eq!(events.len(), 1);
    match &events[0] {
        TraceEvent::ScopeEvaluated {
            step_id,
            matched,
            skipped,
            role_histogram,
        } => {
            assert_eq!(*step_id, 1);
            assert_eq!(*matched, 0);
            assert_eq!(*skipped, 320);
            assert_eq!(*role_histogram, histogram);
        }
        other => panic!("expected ScopeEvaluated, got {other:?}"),
    }
}

#[test]
fn bridge_forwards_full_stage_pair_in_order() {
    let (mut bridge, sink) = install_bridge();
    bridge.on_stage_entered(PipelineStageKind::Sampler, 1, "SineWave", "");
    bridge.on_scope_evaluated(
        1,
        100,
        220,
        RoleHistogram {
            background: 220,
            text: 100,
            ..RoleHistogram::EMPTY
        },
    );
    bridge.on_stage_finished(PipelineStageKind::Sampler, 1, 100, 1_500);

    let events = captured_events(&sink);
    assert_eq!(events.len(), 3, "expected three envelopes in order");
    assert!(matches!(events[0], TraceEvent::StageEntered { .. }));
    assert!(matches!(events[1], TraceEvent::ScopeEvaluated { .. }));
    assert!(matches!(events[2], TraceEvent::StageFinished { .. }));
}

// <FILE>crates/tui-vfx-compositor/tests/test_inspection_sink_bridge_per_stage.rs</FILE> - <DESC>Bridge round-trip test for new per-stage callbacks</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
