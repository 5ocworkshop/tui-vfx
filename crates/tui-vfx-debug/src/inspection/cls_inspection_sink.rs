// <FILE>crates/tui-vfx-debug/src/inspection/cls_inspection_sink.rs</FILE> - <DESC>InspectionSink trait — object-safe Send+Sync report hook</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — the one hook every stage emitter calls. Object-safe so sinks can be stored behind `Arc<dyn InspectionSink>`; Send+Sync so the same sink can be shared across the compositor bridge, the scene composer, and the lifecycle manager without interior-mutability gymnastics.</WCTX>
// <CLOG>0.1.0: initial trait with a single report(&self, envelope) method; object-safe; Send+Sync bounds.</CLOG>

//! The single hook every stage emitter calls.
//!
//! A concrete sink (notably [`crate::inspection::TraceSink`]) decides
//! what to do with the envelope — collect, filter, forward, ignore.
//! Keeping the trait a single `report(&self, TraceEnvelope)` method
//! makes it object-safe and trivially mockable in tests.

use super::cls_trace_envelope::TraceEnvelope;

/// Object-safe sink for [`TraceEnvelope`]s.
///
/// Implementors accept envelopes from every pipeline stage. `&self` is
/// deliberate: a sink must provide its own interior synchronisation
/// so it can be shared behind `Arc<dyn InspectionSink>` across threads.
pub trait InspectionSink: Send + Sync {
    /// Report one envelope to the sink.
    fn report(&self, envelope: TraceEnvelope);
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_inspection_sink.rs</FILE> - <DESC>InspectionSink trait</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
