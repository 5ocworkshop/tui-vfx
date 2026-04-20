// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_sink.rs</FILE> - <DESC>TraceSink — bounded, filtered, thread-safe InspectionSink impl</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — concrete sink the rest of the pipeline reports into. Thread-safe via an internal Mutex over a VecDeque; bounded mode drops oldest with an explicit dropped counter; TraceFilter::accepts_any_stage short-circuit makes the no-op path allocation-free (proved by bench_emit_overhead).</WCTX>
// <CLOG>0.1.0: initial TraceSink with new / with_capacity constructors; InspectionSink impl; snapshot() materialises the TraceReport with per-stage summary.</CLOG>

//! Thread-safe, filtered, optionally-bounded [`InspectionSink`]
//! implementation.
//!
//! # Modes
//!
//! - **Unbounded** ([`TraceSink::new`]): accepted envelopes are appended
//!   forever. Suitable for short traces (one lab session / 120 frames).
//! - **Bounded** ([`TraceSink::with_capacity`]): accepted envelopes past
//!   the capacity drop the oldest, incrementing a `dropped` counter
//!   so [`TraceReport`] can flag truncation. Suitable for long-running
//!   captures (CLI streaming).
//!
//! # Thread-safety
//!
//! A single `Mutex<Inner>` serialises emits. The sink is a drop-in
//! `Arc<dyn InspectionSink>` across the compositor bridge, the scene
//! composer, and the lifecycle manager.

use std::collections::VecDeque;
use std::sync::Mutex;

use super::cls_inspection_sink::InspectionSink;
use super::cls_trace_envelope::TraceEnvelope;
use super::cls_trace_filter::TraceFilter;
use super::cls_trace_report::{TraceReport, TraceReportSummary};

/// Thread-safe, filtered, optionally-bounded sink for trace envelopes.
pub struct TraceSink {
    filter: TraceFilter,
    capacity: Option<usize>,
    inner: Mutex<Inner>,
}

struct Inner {
    envelopes: VecDeque<TraceEnvelope>,
    dropped: u64,
}

impl TraceSink {
    /// Unbounded sink with the supplied filter.
    pub fn new(filter: TraceFilter) -> Self {
        TraceSink {
            filter,
            capacity: None,
            inner: Mutex::new(Inner {
                envelopes: VecDeque::new(),
                dropped: 0,
            }),
        }
    }

    /// Bounded sink with the supplied filter and capacity `n`.
    ///
    /// When the sink is full and a new envelope is accepted, the
    /// oldest envelope is dropped and the dropped counter is
    /// incremented.
    ///
    /// `capacity` is clamped to at least 1.
    pub fn with_capacity(filter: TraceFilter, capacity: usize) -> Self {
        let capacity = capacity.max(1);
        TraceSink {
            filter,
            capacity: Some(capacity),
            inner: Mutex::new(Inner {
                envelopes: VecDeque::with_capacity(capacity),
                dropped: 0,
            }),
        }
    }

    /// Fast emit-site short-circuit: whether the sink's filter could
    /// accept any event at all.
    ///
    /// Emitters call this first and bail before building the envelope
    /// when the filter is fully inert (no selectors, or
    /// [`crate::inspection::StageMask::NONE`]).
    pub fn accepts_any_stage(&self) -> bool {
        self.filter.accepts_any_stage()
    }

    /// Borrow the active filter (read-only).
    pub fn filter(&self) -> &TraceFilter {
        &self.filter
    }

    /// Materialise the current sink contents into a [`TraceReport`].
    ///
    /// Does not drain the sink — it clones the current envelope list
    /// and reads the dropped counter. Subsequent emits continue to
    /// accumulate.
    pub fn snapshot(&self) -> TraceReport {
        let inner = self.inner.lock().expect("TraceSink mutex poisoned");
        let envelopes: Vec<TraceEnvelope> = inner.envelopes.iter().cloned().collect();
        let summary = TraceReportSummary::of(&envelopes);
        TraceReport {
            envelopes,
            summary,
            dropped: inner.dropped,
        }
    }

    /// Drain the sink, returning a [`TraceReport`] and emptying the
    /// internal buffer. The dropped counter resets.
    pub fn drain(&self) -> TraceReport {
        let mut inner = self.inner.lock().expect("TraceSink mutex poisoned");
        let envelopes: Vec<TraceEnvelope> = inner.envelopes.drain(..).collect();
        let dropped = std::mem::replace(&mut inner.dropped, 0);
        let summary = TraceReportSummary::of(&envelopes);
        TraceReport {
            envelopes,
            summary,
            dropped,
        }
    }
}

impl InspectionSink for TraceSink {
    fn report(&self, envelope: TraceEnvelope) {
        // Fast path: if no stages of interest or no selectors, skip.
        if !self.filter.accepts_any_stage() {
            return;
        }
        if !self.filter.accepts(&envelope) {
            return;
        }
        let mut inner = self.inner.lock().expect("TraceSink mutex poisoned");
        if let Some(cap) = self.capacity {
            while inner.envelopes.len() >= cap {
                inner.envelopes.pop_front();
                inner.dropped = inner.dropped.saturating_add(1);
            }
        }
        inner.envelopes.push_back(envelope);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_sink.rs</FILE> - <DESC>TraceSink</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
