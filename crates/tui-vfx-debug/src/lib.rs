// <FILE>tui-vfx-debug/src/lib.rs</FILE> - <DESC>Centralized debug logger + unified inspection foundation crate root</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — tui-vfx-debug becomes logger + unified inspection foundation per spec §9.1. Add public `inspection` module carrying TraceEvent / TraceEnvelope / TraceSelector / TraceFilter / StageMask / InspectionSink / TraceSink / TraceReport. Existing logger surface preserved unchanged.</WCTX>
// <CLOG>1.1.0: MINOR additive — expose `pub mod inspection` alongside the existing logger types. No changes to config.rs / logger.rs.
// 1.0.0: initial creation as standalone crate (logger only).</CLOG>

//! # tui-vfx-debug
//!
//! This crate carries two complementary responsibilities:
//!
//! 1. **Debug logger.** A module-scoped logging factory (with a
//!    shared global singleton) used across the tui-vfx ecosystem
//!    during debugging sessions. See [`DebugLogger`], [`Logger`],
//!    [`LogLevel`].
//! 2. **Unified inspection foundation** (added in 1.1.0 as part of
//!    Sub-plan A Phase A.4). The [`inspection`] module owns the
//!    canonical [`TraceEvent`](inspection::TraceEvent) taxonomy,
//!    the [`TraceEnvelope`](inspection::TraceEnvelope) frame/time
//!    carrier, the [`TraceFilter`](inspection::TraceFilter) +
//!    [`TraceSelector`](inspection::TraceSelector) predicate pair,
//!    the [`InspectionSink`](inspection::InspectionSink) trait, the
//!    concrete thread-safe [`TraceSink`](inspection::TraceSink) + its
//!    [`TraceReport`](inspection::TraceReport) output, and the
//!    [`StageMask`](inspection::StageMask) bit mask that gates events
//!    at sink-time.
//!
//! `CompositorInspector` stays in `tui-vfx-compositor`
//! (`crates/tui-vfx-compositor/src/traits/pipeline_inspector.rs`); the
//! compositor ships an additive bridge that forwards its callback
//! stream into a registered [`inspection::InspectionSink`] without
//! disturbing existing direct implementors.

mod config;
pub mod inspection;
mod logger;

pub use config::{LogLevel, ModuleConfig};
pub use logger::{DebugLogger, LogEntry, Logger, create_logger, get_global_logger};

// <FILE>tui-vfx-debug/src/lib.rs</FILE> - <DESC>Centralized debug logger + inspection foundation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
