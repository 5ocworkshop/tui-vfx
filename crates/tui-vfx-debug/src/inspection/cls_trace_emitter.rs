// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_emitter.rs</FILE> - <DESC>Shared TraceEmitter authority for frame context and seq_in_frame stamping</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — centralize trace-envelope stamping in tui-vfx-debug so the compositor bridge and future recipe-side emit sites share one per-frame seq_in_frame authority without duplicate counters.</WCTX>
// <CLOG>0.1.0: add TraceFrameContext and TraceEmitter with shared sink/context ownership, atomic seq_in_frame tracking, and begin_frame reset semantics.</CLOG>

//! Shared trace-envelope stamping authority.
//!
//! `TraceEmitter` owns the frame/time/recipe context plus the monotonic
//! `seq_in_frame` counter used to stamp [`TraceEnvelope`] values before
//! they are forwarded to an [`InspectionSink`]. The design intent is one
//! emitter per frame-cycle, borrowed by multiple emit sites.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

use tui_vfx_types::RecipeId;

use super::cls_inspection_sink::InspectionSink;
use super::cls_trace_envelope::TraceEnvelope;
use super::cls_trace_event::TraceEvent;

/// Per-frame context stamped onto emitted [`TraceEnvelope`] values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceFrameContext {
    /// Monotonic frame counter since the owning manager started.
    pub frame_no: u64,
    /// Elapsed milliseconds since the owning manager started.
    pub t_ms: u64,
    /// Recipe identity, if the frame belongs to a managed recipe run.
    pub recipe_id: Option<RecipeId>,
}

impl TraceFrameContext {
    /// Construct a frame context without a recipe id.
    pub fn new(frame_no: u64, t_ms: u64) -> Self {
        Self {
            frame_no,
            t_ms,
            recipe_id: None,
        }
    }

    /// Attach a recipe id to the context.
    pub fn with_recipe_id(mut self, recipe_id: RecipeId) -> Self {
        self.recipe_id = Some(recipe_id);
        self
    }
}

/// Shared stamping authority for trace envelopes.
pub struct TraceEmitter {
    sink: Arc<dyn InspectionSink>,
    frame: RwLock<TraceFrameContext>,
    seq: AtomicU32,
}

impl TraceEmitter {
    /// Create a new emitter for one logical frame stream.
    pub fn new(sink: Arc<dyn InspectionSink>, frame: TraceFrameContext) -> Self {
        Self {
            sink,
            frame: RwLock::new(frame),
            seq: AtomicU32::new(0),
        }
    }

    /// Replace the active frame context and reset the per-frame sequence.
    pub fn begin_frame(&self, frame: TraceFrameContext) {
        *self.frame.write().expect("TraceEmitter frame lock poisoned") = frame;
        self.seq.store(0, Ordering::Relaxed);
    }

    /// Emit one event through the shared sink, stamping the current frame context.
    pub fn emit(&self, event: TraceEvent) {
        let frame = self
            .frame
            .read()
            .expect("TraceEmitter frame lock poisoned")
            .clone();
        let seq_in_frame = self.seq.fetch_add(1, Ordering::Relaxed);
        self.sink.report(TraceEnvelope {
            event,
            frame_no: frame.frame_no,
            t_ms: frame.t_ms,
            recipe_id: frame.recipe_id,
            seq_in_frame,
        });
    }

    /// Borrow the wrapped sink.
    pub fn sink(&self) -> &Arc<dyn InspectionSink> {
        &self.sink
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_emitter.rs</FILE> - <DESC>TraceEmitter shared stamping authority</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
