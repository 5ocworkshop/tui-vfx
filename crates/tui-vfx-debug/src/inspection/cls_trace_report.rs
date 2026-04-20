// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_report.rs</FILE> - <DESC>TraceReport — materialised trace with summary, NDJSON round-trip methods</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — materialised snapshot of a sink's contents. to_ndjson emits one envelope per line; from_ndjson round-trips the envelope list (dropped counter is sink-side metadata and resets on read per peer-review S12). Summary carries per-stage counts so downstream analysis doesn't re-scan the envelope list.</WCTX>
// <CLOG>0.1.0: initial TraceReport + TraceReportSummary; to_ndjson / from_ndjson methods; per-stage count accessor.</CLOG>

//! Materialised view of a trace capture.
//!
//! A [`TraceReport`] is the end-state produced by
//! [`crate::inspection::TraceSink::snapshot`] (or `drain`). It carries
//! the full envelope list, a per-stage summary, and a `dropped` counter
//! so bounded-mode truncation is observable.
//!
//! # NDJSON
//!
//! [`TraceReport::to_ndjson`] serialises one envelope per line — the
//! format CLIs and AI agents consume directly. [`TraceReport::from_ndjson`]
//! parses the same representation back. `dropped` is a sink-side
//! concept and does not survive a round-trip (it resets to 0 on read).

use std::io::{BufRead, BufReader, Read, Result as IoResult, Write};

use serde::{Deserialize, Serialize};

use super::cls_stage_mask::StageMask;
use super::cls_trace_envelope::TraceEnvelope;

/// Materialised snapshot of a trace capture.
///
/// Fields:
/// - `envelopes` — the accepted envelopes in emit order.
/// - `summary` — per-stage aggregated counts.
/// - `dropped` — bounded-mode drops (0 in unbounded mode).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TraceReport {
    /// Envelopes in emit order (oldest first).
    pub envelopes: Vec<TraceEnvelope>,
    /// Per-stage aggregate counts.
    pub summary: TraceReportSummary,
    /// Number of envelopes dropped by a bounded sink. Zero in
    /// unbounded mode.
    pub dropped: u64,
}

impl TraceReport {
    /// Serialise one envelope per line (NDJSON) to `writer`.
    ///
    /// Each line is a standalone JSON document representing a
    /// [`TraceEnvelope`]. Lines are terminated with `\n`. `dropped` is
    /// not emitted (it is sink-side metadata; replays start fresh).
    pub fn to_ndjson(&self, mut writer: impl Write) -> IoResult<()> {
        for envelope in &self.envelopes {
            let line = serde_json::to_string(envelope)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        Ok(())
    }

    /// Parse envelopes from an NDJSON stream on `reader`.
    ///
    /// Blank lines are silently skipped. Any non-blank line that fails
    /// to parse as a [`TraceEnvelope`] produces an `InvalidData`
    /// error. The returned report's `dropped` counter is 0; the summary
    /// is computed from the parsed envelopes.
    pub fn from_ndjson(reader: impl Read) -> IoResult<Self> {
        let buf = BufReader::new(reader);
        let mut envelopes: Vec<TraceEnvelope> = Vec::new();
        for line in buf.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let envelope: TraceEnvelope = serde_json::from_str(&line)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            envelopes.push(envelope);
        }
        let summary = TraceReportSummary::of(&envelopes);
        Ok(TraceReport {
            envelopes,
            summary,
            dropped: 0,
        })
    }
}

/// Per-stage aggregate counts for a [`TraceReport`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TraceReportSummary {
    /// Count of lifecycle-stage envelopes.
    pub lifecycle: u64,
    /// Count of resolution-stage envelopes.
    pub resolution: u64,
    /// Count of composition-stage envelopes.
    pub composition: u64,
    /// Count of pipeline-stage envelopes.
    pub pipeline: u64,
    /// Total envelopes (sum of the above).
    pub total: u64,
}

impl TraceReportSummary {
    /// Compute a summary from an envelope slice.
    pub fn of(envelopes: &[TraceEnvelope]) -> Self {
        let mut s = TraceReportSummary::default();
        for envelope in envelopes {
            let stage = envelope.event.stage();
            if stage.contains(StageMask::LIFECYCLE) {
                s.lifecycle += 1;
            } else if stage.contains(StageMask::RESOLUTION) {
                s.resolution += 1;
            } else if stage.contains(StageMask::COMPOSITION) {
                s.composition += 1;
            } else if stage.contains(StageMask::PIPELINE) {
                s.pipeline += 1;
            }
        }
        s.total = s.lifecycle + s.resolution + s.composition + s.pipeline;
        s
    }

    /// Look up the count for a single-bit stage mask.
    ///
    /// Multi-bit masks are permitted — the counts are summed.
    pub fn count_for(&self, stage: StageMask) -> u64 {
        let mut total = 0u64;
        if stage.contains(StageMask::LIFECYCLE) {
            total += self.lifecycle;
        }
        if stage.contains(StageMask::RESOLUTION) {
            total += self.resolution;
        }
        if stage.contains(StageMask::COMPOSITION) {
            total += self.composition;
        }
        if stage.contains(StageMask::PIPELINE) {
            total += self.pipeline;
        }
        total
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_report.rs</FILE> - <DESC>TraceReport</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
