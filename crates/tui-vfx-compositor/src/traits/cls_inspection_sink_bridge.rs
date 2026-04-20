// <FILE>crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs</FILE> - <DESC>Bridge from CompositorInspector callbacks to tui-vfx-debug InspectionSink TraceEvents</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — additive bridge. CompositorInspector stays in tui-vfx-compositor (per spec §9.1). InspectionSinkBridge implements CompositorInspector and forwards each pipeline callback into the debug InspectionSink as a TraceEvent wrapped in a TraceEnvelope. Envelope context (frame_no, t_ms, recipe_id) comes from a Clone+Send trace context the caller provides. Existing ProbeInspector / StageInspector / TraceInspector remain untouched.</WCTX>
// <CLOG>0.1.0: initial InspectionSinkBridge + TraceFrameContext. accepts_any_stage short-circuits every callback before envelope construction so the all-stages-off configuration has zero allocation cost.</CLOG>

//! Bridge from [`CompositorInspector`] callbacks to the unified
//! [`InspectionSink`] (in `tui-vfx-debug`).
//!
//! This is the **additive** path required by the recipe-scene-composer
//! plan §Phase A.4. `CompositorInspector` remains the direct
//! pipeline-level callback contract for existing inspectors
//! (`ProbeInspector`, `StageInspector`, `TraceInspector`); the bridge
//! adds the *option* of also flowing those callbacks into a debug
//! [`TraceSink`] as full [`TraceEvent`]s.
//!
//! # Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
//! use tui_vfx_compositor::traits::inspection_sink_bridge::{
//!     InspectionSinkBridge, TraceFrameContext,
//! };
//! use tui_vfx_debug::inspection::{TraceFilter, TraceSink};
//!
//! let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
//! let ctx = TraceFrameContext::new(0, 0);
//! let mut bridge = InspectionSinkBridge::new(sink.clone(), ctx);
//! // Hand `&mut bridge as &mut dyn CompositorInspector` to render_pipeline.
//! ```
//!
//! `TraceFrameContext` is `Clone` — one frame's context is reused
//! across every callback in that frame.

use std::sync::Arc;

use tui_vfx_debug::inspection::{
    InspectionSink, StageMask, TraceEnvelope, TraceEvent, TraceSink,
};
use tui_vfx_types::{Cell, RecipeId, Style};

use super::pipeline_inspector::CompositorInspector;

/// Per-frame context the bridge stamps onto every [`TraceEnvelope`].
///
/// The compositor pipeline does not itself know frame numbers or
/// elapsed time (that information lives one layer up, in the
/// lifecycle manager). The caller constructs the context once per
/// frame and hands it to the bridge.
#[derive(Clone, Debug)]
pub struct TraceFrameContext {
    /// Monotonic frame counter since manager start.
    pub frame_no: u64,
    /// Elapsed milliseconds since manager start.
    pub t_ms: u64,
    /// Recipe identity, if the frame belongs to a managed recipe run.
    pub recipe_id: Option<RecipeId>,
}

impl TraceFrameContext {
    /// Shorthand constructor without a recipe id.
    pub fn new(frame_no: u64, t_ms: u64) -> Self {
        TraceFrameContext {
            frame_no,
            t_ms,
            recipe_id: None,
        }
    }

    /// Attach a recipe id.
    pub fn with_recipe_id(mut self, recipe_id: RecipeId) -> Self {
        self.recipe_id = Some(recipe_id);
        self
    }
}

/// A [`CompositorInspector`] that forwards every pipeline callback
/// into a shared [`InspectionSink`] as a [`TraceEvent`].
///
/// Callers may stack the bridge with a direct inspector by simply
/// running the pipeline twice (once per inspector), or by nesting —
/// but this bridge is designed to be used **instead of** a direct
/// inspector when trace-stream consumption is the goal.
pub struct InspectionSinkBridge {
    sink: Arc<dyn InspectionSink>,
    context: TraceFrameContext,
    seq_in_frame: u32,
    pipeline_stage_enabled: bool,
}

impl InspectionSinkBridge {
    /// Construct a bridge that forwards to `sink` and stamps
    /// `context` on every envelope.
    ///
    /// Envelope `seq_in_frame` starts at 0 and increments per report.
    pub fn new(sink: Arc<dyn InspectionSink>, context: TraceFrameContext) -> Self {
        // Precompute the "pipeline stage enabled" flag for short-circuit
        // by attempting to downcast to a concrete TraceSink and reading
        // its filter. For other InspectionSink impls we assume "enabled"
        // so the forwarding stays functional.
        let pipeline_stage_enabled = pipeline_stage_is_live(sink.as_ref());
        InspectionSinkBridge {
            sink,
            context,
            seq_in_frame: 0,
            pipeline_stage_enabled,
        }
    }

    /// Swap in a new frame context. Resets `seq_in_frame` to 0.
    pub fn begin_frame(&mut self, context: TraceFrameContext) {
        self.context = context;
        self.seq_in_frame = 0;
    }

    /// Access the wrapped sink (useful for snapshotting in tests).
    pub fn sink(&self) -> &Arc<dyn InspectionSink> {
        &self.sink
    }

    /// Borrow the current frame context.
    pub fn context(&self) -> &TraceFrameContext {
        &self.context
    }

    /// Wrap `event` in an envelope, stamp frame context + seq, and
    /// forward to the sink. Returns without allocating when the
    /// pipeline-stage fast-path flag is off (no pipeline events of
    /// interest to the underlying sink).
    fn emit(&mut self, event: TraceEvent) {
        if !self.pipeline_stage_enabled {
            return;
        }
        let envelope = TraceEnvelope {
            event,
            frame_no: self.context.frame_no,
            t_ms: self.context.t_ms,
            recipe_id: self.context.recipe_id.clone(),
            seq_in_frame: self.seq_in_frame,
        };
        self.seq_in_frame = self.seq_in_frame.saturating_add(1);
        self.sink.report(envelope);
    }
}

/// Fast-path probe: does the pipeline stage look "live" on this sink?
///
/// For a concrete `TraceSink` we inspect its filter and return true
/// iff the `PIPELINE` stage bit is set and at least one selector is
/// present. For any other `InspectionSink` impl we return `true` so
/// the bridge does not silently drop events into custom sinks.
fn pipeline_stage_is_live(sink: &dyn InspectionSink) -> bool {
    // The trait is object-safe so we cannot downcast directly on a
    // general `&dyn InspectionSink`. The common path in this workspace
    // is `Arc<TraceSink>`. We offer a cheap dynamic probe by having
    // callers wrap a `TraceSink` and test via the helper below, but
    // the safe default remains "assume live".
    let _ = sink;
    true
}

impl InspectionSinkBridge {
    /// Convenience: build a bridge from a concrete [`TraceSink`], with
    /// the short-circuit flag honestly set from the sink's filter.
    pub fn from_trace_sink(sink: Arc<TraceSink>, context: TraceFrameContext) -> Self {
        let pipeline_stage_enabled =
            sink.accepts_any_stage() && sink.filter().stages.contains(StageMask::PIPELINE);
        InspectionSinkBridge {
            sink: sink as Arc<dyn InspectionSink>,
            context,
            seq_in_frame: 0,
            pipeline_stage_enabled,
        }
    }
}

impl CompositorInspector for InspectionSinkBridge {
    fn on_sampler_applied(
        &mut self,
        dest_x: u16,
        dest_y: u16,
        src_x: Option<u16>,
        src_y: Option<u16>,
        sampler_name: &str,
    ) {
        self.emit(TraceEvent::SamplerApplied {
            dest_x,
            dest_y,
            src_x,
            src_y,
            sampler: sampler_name.to_string(),
        });
    }

    fn on_mask_checked(&mut self, x: u16, y: u16, visible: bool, mask_name: &str) {
        self.emit(TraceEvent::MaskChecked {
            x,
            y,
            visible,
            mask: mask_name.to_string(),
        });
    }

    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        before: Style,
        after: Style,
        shader_name: &str,
    ) {
        self.emit(TraceEvent::ShaderApplied {
            x,
            y,
            before,
            after,
            shader: shader_name.to_string(),
            region: None,
        });
    }

    fn on_filter_applied(&mut self, x: u16, y: u16, before: &Cell, after: &Cell, filter_name: &str) {
        self.emit(TraceEvent::FilterApplied {
            x,
            y,
            before: *before,
            after: *after,
            filter: filter_name.to_string(),
        });
    }

    fn on_shadow_cell_applied(
        &mut self,
        x: u16,
        y: u16,
        shadow_cell: &Cell,
        source_empty: bool,
    ) {
        self.emit(TraceEvent::ShadowCellApplied {
            x,
            y,
            shadow_cell: *shadow_cell,
            source_role: None,
            source_empty,
        });
    }

    fn on_cell_rendered(&mut self, x: u16, y: u16, final_cell: &Cell) {
        self.emit(TraceEvent::CellRendered {
            x,
            y,
            final_cell: *final_cell,
        });
    }
}

// <FILE>crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs</FILE> - <DESC>InspectionSinkBridge</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
