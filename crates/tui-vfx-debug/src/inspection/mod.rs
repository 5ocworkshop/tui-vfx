// <FILE>crates/tui-vfx-debug/src/inspection/mod.rs</FILE> - <DESC>Inspection module root for the unified trace pipeline</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — inspection module root. tui-vfx-debug becomes logger + unified inspection foundation per spec §9.1. Re-exports the public surface: StageMask, TraceEvent, TraceEnvelope, TraceSelector, TraceFilter, InspectionSink, TraceSink, TraceReport, TraceReportSummary, TraceFrameContext, and TraceEmitter.</WCTX>
// <CLOG>0.2.0: add TraceFrameContext + TraceEmitter so seq_in_frame stamping is centralized for cross-repo emit sites.
// 0.1.0: initial module tree with OFPF cls_* files for each primitive; re-exports at module root.</CLOG>

//! Unified inspection foundation for the recipe scene composer.
//!
//! This module realises the inspection surface defined in the
//! recipe-scene-composer spec §9:
//!
//! - A single [`InspectionSink`] hook every pipeline stage reports to.
//! - A canonical [`TraceEvent`] taxonomy across four stages (lifecycle,
//!   resolution, composition, pipeline).
//! - An envelope carrying frame / time / recipe / seq-in-frame context.
//! - A declarative filter at sink-time (selector set + stage mask +
//!   frame range + time range).
//! - A concrete [`TraceSink`] that is thread-safe, filter-aware, and
//!   optionally bounded.
//! - A [`TraceReport`] with NDJSON round-trip for AI / CLI consumption.
//!
//! # Quick-start
//!
//! ```
//! use std::sync::Arc;
//! use tui_vfx_debug::inspection::{
//!     InspectionSink, StageMask, TraceEnvelope, TraceEvent, TraceFilter,
//!     TraceSelector, TraceSink,
//! };
//! use tui_vfx_types::{Cell, RecipeId};
//!
//! // Sink that keeps only pipeline-stage events touching cells in a rect.
//! let filter = TraceFilter {
//!     selectors: vec![TraceSelector::All],
//!     stages: StageMask::PIPELINE,
//!     frames: 0..u64::MAX,
//!     time_ms: 0..u64::MAX,
//! };
//! let sink: Arc<dyn InspectionSink> = Arc::new(TraceSink::new(filter));
//!
//! // Emitters report envelopes through the InspectionSink hook.
//! sink.report(TraceEnvelope {
//!     event: TraceEvent::CellRendered { x: 0, y: 0, final_cell: Cell::default() },
//!     frame_no: 0,
//!     t_ms: 0,
//!     recipe_id: Some(RecipeId::from("splash.v2")),
//!     seq_in_frame: 0,
//! });
//! ```

mod cls_inspection_sink;
mod cls_stage_mask;
mod cls_trace_emitter;
mod cls_trace_envelope;
mod cls_trace_event;
mod cls_trace_filter;
mod cls_trace_report;
mod cls_trace_selector;
mod cls_trace_sink;

pub use cls_inspection_sink::InspectionSink;
pub use cls_stage_mask::StageMask;
pub use cls_trace_emitter::{TraceEmitter, TraceFrameContext};
pub use cls_trace_envelope::TraceEnvelope;
pub use cls_trace_event::TraceEvent;
pub use cls_trace_filter::TraceFilter;
pub use cls_trace_report::{TraceReport, TraceReportSummary};
pub use cls_trace_selector::TraceSelector;
pub use cls_trace_sink::TraceSink;

// <FILE>crates/tui-vfx-debug/src/inspection/mod.rs</FILE> - <DESC>Inspection module root</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
