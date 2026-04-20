// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_envelope.rs</FILE> - <DESC>TraceEnvelope — a TraceEvent plus its frame/time/recipe/seq context</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — envelope that carries the frame context around every TraceEvent. seq_in_frame (per peer-review S4) makes replay order deterministic even when multiple events fire at the same t_ms in the same frame.</WCTX>
// <CLOG>0.1.0: initial TraceEnvelope struct with event / frame_no / t_ms / optional recipe_id / seq_in_frame; serde derive; Clone/Debug/PartialEq.</CLOG>

//! Envelope that wraps a [`TraceEvent`] with its frame/time context.
//!
//! Every event the inspection pipeline reports is packaged in a
//! [`TraceEnvelope`] so downstream consumers have a uniform way to
//! reason about timing, per-frame ordering, and recipe identity.
//!
//! # Why `seq_in_frame`?
//!
//! Many events can share the same `t_ms` inside one frame (they are all
//! produced by the same render pass). `seq_in_frame` is the monotonic
//! per-frame counter that fixes the replay order. See the recipe-scene
//! composer spec §9.6 (determinism).

use serde::{Deserialize, Serialize};
use tui_vfx_types::RecipeId;

use super::cls_trace_event::TraceEvent;

/// A [`TraceEvent`] plus its frame/time/recipe/seq context.
///
/// # Fields
///
/// - `event` — the wrapped trace event.
/// - `frame_no` — monotonic frame counter since manager start.
/// - `t_ms` — elapsed milliseconds since manager start.
/// - `recipe_id` — recipe identity if the event occurred within a
///   managed recipe run. `None` for events that are not recipe-scoped
///   (e.g. workspace-level resolution events from a CLI).
/// - `seq_in_frame` — monotonic per-frame sequence counter (resets at
///   the start of each frame). Gives replay a stable ordering.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TraceEnvelope {
    /// The wrapped trace event.
    pub event: TraceEvent,
    /// Monotonic frame counter since the manager started.
    pub frame_no: u64,
    /// Elapsed milliseconds since the manager started.
    pub t_ms: u64,
    /// Recipe identity if the event occurred within a managed run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipe_id: Option<RecipeId>,
    /// Per-frame sequence counter (resets at each new frame).
    #[serde(default)]
    pub seq_in_frame: u32,
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_envelope.rs</FILE> - <DESC>TraceEnvelope</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
