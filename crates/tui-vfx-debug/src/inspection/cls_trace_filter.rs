// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_filter.rs</FILE> - <DESC>TraceFilter combining selectors + stage mask + frame / time ranges</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — the filter consumers pass to TraceSink. OR semantics across selectors; AND-combined with stage mask, frame range, time range. accept_all() is the pass-all sentinel for debugging. accepts_any_stage() is the cheap short-circuit used by emit sites to skip envelope construction when the filter is totally inert.</WCTX>
// <CLOG>0.1.0: initial TraceFilter struct + accepts() predicate + accept_all() + accepts_any_stage() short-circuit.</CLOG>

//! Filter applied at sink-time to decide whether a [`TraceEnvelope`] is
//! retained.
//!
//! OR across selectors; AND across (selectors × stages × frames × time).
//!
//! # Emit-site short-circuit
//!
//! [`TraceFilter::accepts_any_stage`] returns whether *any* stage bit is
//! set. Emitters can call it first and bail without building a
//! [`TraceEnvelope`] when the filter is inert.

use serde::{Deserialize, Serialize};
use std::ops::Range;

use super::cls_stage_mask::StageMask;
use super::cls_trace_envelope::TraceEnvelope;
use super::cls_trace_selector::TraceSelector;

/// Declarative filter applied at sink-time.
///
/// - `selectors` — OR-semantics; an envelope passes if **any** selector
///   matches.
/// - `stages` — AND-semantics against the event's stage bit.
/// - `frames` — half-open range over `envelope.frame_no`.
/// - `time_ms` — half-open range over `envelope.t_ms`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceFilter {
    /// Union of selectors (OR). Empty list rejects everything.
    pub selectors: Vec<TraceSelector>,
    /// Stage bitmask (AND). `StageMask::NONE` rejects everything.
    pub stages: StageMask,
    /// Accept only frames in this half-open range.
    pub frames: Range<u64>,
    /// Accept only envelopes with `t_ms` in this half-open range.
    pub time_ms: Range<u64>,
}

impl TraceFilter {
    /// Construct the pass-all filter: every selector, every stage,
    /// every frame, every time.
    pub fn accept_all() -> Self {
        TraceFilter {
            selectors: vec![TraceSelector::All],
            stages: StageMask::ALL,
            frames: 0..u64::MAX,
            time_ms: 0..u64::MAX,
        }
    }

    /// True if at least one stage bit is set.
    ///
    /// Used by emit-site short-circuits to skip envelope construction
    /// when the filter cannot possibly accept anything.
    pub fn accepts_any_stage(&self) -> bool {
        !self.stages.is_empty() && !self.selectors.is_empty()
    }

    /// True if `envelope` passes every dimension of the filter.
    pub fn accepts(&self, envelope: &TraceEnvelope) -> bool {
        if self.selectors.is_empty() {
            return false;
        }
        if self.stages.is_empty() {
            return false;
        }
        if !self.stages.contains(envelope.event.stage()) {
            return false;
        }
        if !range_contains(&self.frames, envelope.frame_no) {
            return false;
        }
        if !range_contains(&self.time_ms, envelope.t_ms) {
            return false;
        }
        self.selectors.iter().any(|s| s.matches(envelope))
    }
}

impl Default for TraceFilter {
    fn default() -> Self {
        // A default filter should be a true no-op — reject everything.
        // `accept_all()` is the explicit pass-all opt-in.
        TraceFilter {
            selectors: Vec::new(),
            stages: StageMask::NONE,
            frames: 0..0,
            time_ms: 0..0,
        }
    }
}

#[inline]
fn range_contains(range: &Range<u64>, value: u64) -> bool {
    value >= range.start && value < range.end
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_filter.rs</FILE> - <DESC>TraceFilter</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
