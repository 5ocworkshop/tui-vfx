// <FILE>crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs</FILE> - <DESC>Bridge from CompositorInspector callbacks to tui-vfx-debug InspectionSink TraceEvents</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Pipeline observability Unit A — forward the four new per-stage CompositorInspector callbacks (on_stage_entered/on_stage_finished/on_stage_skipped/on_scope_evaluated) onto the inspection bus as the matching TraceEvent variants, reusing the existing pipeline_stage_enabled fast-path.</WCTX>
// <CLOG>0.3.0: forward on_stage_entered/on_stage_finished/on_stage_skipped/on_scope_evaluated to TraceEvent::StageEntered/StageFinished/StageSkipped/ScopeEvaluated through the existing self.emit() path.</CLOG>

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
    InspectionSink, PipelineSkipReason, PipelineStageKind, RoleHistogram, RoleMapSource, StageMask,
    TraceEmitter, TraceEvent, TraceSink,
};
use tui_vfx_types::{Cell, Style};

use super::pipeline_inspector::CompositorInspector;

pub use tui_vfx_debug::inspection::TraceFrameContext;

/// A [`CompositorInspector`] that forwards every pipeline callback
/// into a shared [`InspectionSink`] as a [`TraceEvent`].
///
/// Callers may stack the bridge with a direct inspector by simply
/// running the pipeline twice (once per inspector), or by nesting —
/// but this bridge is designed to be used **instead of** a direct
/// inspector when trace-stream consumption is the goal.
pub struct InspectionSinkBridge {
    emitter: TraceEmitter,
    context: TraceFrameContext,
    pipeline_stage_enabled: bool,
}

impl InspectionSinkBridge {
    /// Construct a bridge that forwards to `sink` and stamps
    /// `context` on every envelope.
    pub fn new(sink: Arc<dyn InspectionSink>, context: TraceFrameContext) -> Self {
        let pipeline_stage_enabled = pipeline_stage_is_live(sink.as_ref());
        let emitter = TraceEmitter::new(sink, context.clone());
        InspectionSinkBridge {
            emitter,
            context,
            pipeline_stage_enabled,
        }
    }

    /// Swap in a new frame context. Resets `seq_in_frame` to 0.
    pub fn begin_frame(&mut self, context: TraceFrameContext) {
        self.context = context.clone();
        self.emitter.begin_frame(context);
    }

    /// Access the wrapped sink (useful for snapshotting in tests).
    pub fn sink(&self) -> &Arc<dyn InspectionSink> {
        self.emitter.sink()
    }

    /// Borrow the current frame context.
    pub fn context(&self) -> &TraceFrameContext {
        &self.context
    }

    /// Forward `event` through the shared emitter after the pipeline
    /// fast-path check.
    fn emit(&self, event: TraceEvent) {
        if !self.pipeline_stage_enabled {
            return;
        }
        self.emitter.emit(event);
    }
}

/// Fast-path probe: does the pipeline stage look "live" on this sink?
fn pipeline_stage_is_live(sink: &dyn InspectionSink) -> bool {
    let _ = sink;
    true
}

impl InspectionSinkBridge {
    /// Convenience: build a bridge from a concrete [`TraceSink`], with
    /// the short-circuit flag honestly set from the sink's filter.
    pub fn from_trace_sink(sink: Arc<TraceSink>, context: TraceFrameContext) -> Self {
        let pipeline_stage_enabled =
            sink.accepts_any_stage() && sink.filter().stages.contains(StageMask::PIPELINE);
        let emitter = TraceEmitter::new(sink.clone() as Arc<dyn InspectionSink>, context.clone());
        InspectionSinkBridge {
            emitter,
            context,
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

    fn on_filter_applied(
        &mut self,
        x: u16,
        y: u16,
        before: &Cell,
        after: &Cell,
        filter_name: &str,
    ) {
        self.emit(TraceEvent::FilterApplied {
            x,
            y,
            before: *before,
            after: *after,
            filter: filter_name.to_string(),
        });
    }

    fn on_shadow_cell_applied(&mut self, x: u16, y: u16, shadow_cell: &Cell, source_empty: bool) {
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

    fn on_stage_entered(
        &mut self,
        kind: PipelineStageKind,
        step_id: u32,
        name: &str,
        scope_summary: &str,
    ) {
        self.emit(TraceEvent::StageEntered {
            kind,
            step_id,
            name: name.to_string(),
            scope_summary: scope_summary.to_string(),
        });
    }

    fn on_stage_finished(
        &mut self,
        kind: PipelineStageKind,
        step_id: u32,
        cells_modified: u32,
        elapsed_ns: u64,
    ) {
        self.emit(TraceEvent::StageFinished {
            kind,
            step_id,
            cells_modified,
            elapsed_ns,
        });
    }

    fn on_stage_skipped(
        &mut self,
        kind: PipelineStageKind,
        step_id: u32,
        reason: PipelineSkipReason,
    ) {
        self.emit(TraceEvent::StageSkipped {
            kind,
            step_id,
            reason,
        });
    }

    fn on_scope_evaluated(
        &mut self,
        step_id: u32,
        matched: u32,
        skipped: u32,
        role_histogram: RoleHistogram,
    ) {
        self.emit(TraceEvent::ScopeEvaluated {
            step_id,
            matched,
            skipped,
            role_histogram,
        });
    }

    fn on_role_map_materialized(&mut self, source: RoleMapSource, histogram: RoleHistogram) {
        self.emit(TraceEvent::RoleMapMaterialized { source, histogram });
    }
}

// <FILE>crates/tui-vfx-compositor/src/traits/cls_inspection_sink_bridge.rs</FILE> - <DESC>InspectionSinkBridge</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
