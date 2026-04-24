// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_emitter.rs</FILE> - <DESC>TDD coverage for TraceEmitter frame sequencing</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 RED — prove the shared TraceEmitter contract before implementation by asserting monotonic seq_in_frame behavior across borrowers and frame resets.</WCTX>
// <CLOG>0.1.0: initial RED tests for TraceEmitter monotonic sequencing and begin_frame reset semantics.</CLOG>

use std::sync::{Arc, Mutex};

use tui_vfx_debug::inspection::{
    InspectionSink, TraceEmitter, TraceEnvelope, TraceEvent, TraceFrameContext,
};

#[derive(Default)]
struct RecordingSink {
    envelopes: Mutex<Vec<TraceEnvelope>>,
}

impl RecordingSink {
    fn snapshot(&self) -> Vec<TraceEnvelope> {
        self.envelopes
            .lock()
            .expect("recording sink mutex poisoned")
            .clone()
    }
}

impl InspectionSink for RecordingSink {
    fn report(&self, envelope: TraceEnvelope) {
        self.envelopes
            .lock()
            .expect("recording sink mutex poisoned")
            .push(envelope);
    }
}

fn sample_event(mask: &str) -> TraceEvent {
    TraceEvent::MaskChecked {
        x: 0,
        y: 0,
        visible: true,
        mask: mask.to_string(),
    }
}

#[test]
fn shared_emitter_assigns_monotonic_seq_numbers_across_borrowers() {
    let sink = Arc::new(RecordingSink::default());
    let emitter = Arc::new(TraceEmitter::new(
        sink.clone() as Arc<dyn InspectionSink>,
        TraceFrameContext::new(7, 42),
    ));

    let borrower_a = emitter.clone();
    let borrower_b = emitter.clone();

    borrower_a.emit(sample_event("a"));
    borrower_b.emit(sample_event("b"));
    borrower_a.emit(sample_event("c"));

    let envelopes = sink.snapshot();
    let seqs: Vec<u32> = envelopes
        .iter()
        .map(|envelope| envelope.seq_in_frame)
        .collect();
    assert_eq!(seqs, vec![0, 1, 2]);
    assert!(envelopes.iter().all(|envelope| envelope.frame_no == 7));
    assert!(envelopes.iter().all(|envelope| envelope.t_ms == 42));
}

#[test]
fn begin_frame_resets_sequence_and_swaps_context() {
    let sink = Arc::new(RecordingSink::default());
    let emitter = TraceEmitter::new(
        sink.clone() as Arc<dyn InspectionSink>,
        TraceFrameContext::new(1, 10),
    );

    emitter.emit(sample_event("before"));
    emitter.begin_frame(TraceFrameContext::new(2, 99));
    emitter.emit(sample_event("after"));

    let envelopes = sink.snapshot();
    assert_eq!(envelopes.len(), 2);
    assert_eq!(envelopes[0].frame_no, 1);
    assert_eq!(envelopes[0].seq_in_frame, 0);
    assert_eq!(envelopes[1].frame_no, 2);
    assert_eq!(envelopes[1].t_ms, 99);
    assert_eq!(envelopes[1].seq_in_frame, 0);
}

// <FILE>crates/tui-vfx-debug/tests/test_cls_trace_emitter.rs</FILE> - <DESC>TraceEmitter sequencing tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
