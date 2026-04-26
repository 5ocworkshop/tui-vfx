// <FILE>crates/tui-vfx-debug/src/inspection/cls_asserting_inspector.rs</FILE> - <DESC>AssertingInspector — test-only InspectionSink that panics on forbidden events</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — mechanical regression guard. Wraps a list of forbidden-event predicates and panics on first match with a clear assertion message naming the offending event variant. The focused_row_btop regression test installs forbid_zero_cell_scope_matches() on the fixed recipe to refuse any future ScopeMatchedZeroCells skip.</WCTX>
// <CLOG>0.1.0: initial AssertingInspector with new(rules) constructor + forbid_zero_cell_scope_matches() convenience helper. Implements InspectionSink (Send + Sync via Mutex) so it composes with the existing InspectionSinkBridge.</CLOG>

//! Test-only [`InspectionSink`] that fails on forbidden events.
//!
//! The compositor's per-cell event stream is silent when a stage skips
//! iteration. Without an `AssertingInspector`, a regression test for
//! "this stage should fire" can only assert on positive events (cells
//! actually modified) — it cannot mechanically refuse the negative case
//! ("this stage should never silently skip").
//!
//! `AssertingInspector` closes that gap. A test installs it with a list
//! of forbidden-event predicates; the first time a matching event fires,
//! the sink panics with a message naming the variant and the predicate
//! description. The test catches the panic via the standard test harness.
//!
//! # Usage
//!
//! ```rust
//! use std::sync::Arc;
//! use tui_vfx_debug::inspection::{
//!     AssertingInspector, InspectionSink, TraceEnvelope, TraceEvent,
//!     PipelineSkipReason, PipelineStageKind, RoleHistogram,
//! };
//!
//! // Refuse any future ScopeMatchedZeroCells skip (the focused_row_btop
//! // bug class).
//! let sink: Arc<dyn InspectionSink> =
//!     Arc::new(AssertingInspector::forbid_zero_cell_scope_matches());
//!
//! // Plug into the InspectionSinkBridge or call .report() directly.
//! // A non-matching event does not panic:
//! sink.report(TraceEnvelope {
//!     event: TraceEvent::StageEntered {
//!         kind: PipelineStageKind::Shader,
//!         step_id: 1,
//!         name: "FocusedRowGradient".to_string(),
//!         scope_summary: "Role(Text)".to_string(),
//!     },
//!     frame_no: 0,
//!     t_ms: 0,
//!     recipe_id: None,
//!     seq_in_frame: 0,
//! });
//! ```

use std::sync::Mutex;

use super::cls_inspection_sink::InspectionSink;
use super::cls_pipeline_skip_reason::PipelineSkipReason;
use super::cls_trace_envelope::TraceEnvelope;
use super::cls_trace_event::TraceEvent;

/// One forbidden-event rule paired with a human-readable description.
///
/// The description appears in the panic message so a failing test
/// names *which* rule fired, not just *that* a rule fired.
pub struct ForbidRule {
    /// Human-readable description of what this rule forbids.
    pub description: String,
    /// Predicate that returns `true` when the event is forbidden.
    pub matches: Box<dyn Fn(&TraceEvent) -> bool + Send + Sync>,
}

impl ForbidRule {
    /// Construct a rule from a description string and a matcher closure.
    pub fn new<F>(description: impl Into<String>, matches: F) -> Self
    where
        F: Fn(&TraceEvent) -> bool + Send + Sync + 'static,
    {
        ForbidRule {
            description: description.into(),
            matches: Box::new(matches),
        }
    }
}

/// Test-only [`InspectionSink`] that panics on the first forbidden event.
///
/// `report` is called with `&self` (per the [`InspectionSink`] contract);
/// the rule list is cheap to evaluate (O(rules) per event) and the
/// internal `Mutex` only serialises against rule mutation, which today
/// never happens after construction.
pub struct AssertingInspector {
    rules: Mutex<Vec<ForbidRule>>,
}

impl AssertingInspector {
    /// Construct an inspector that panics on the first event matching
    /// any rule in `rules`.
    pub fn new(rules: Vec<ForbidRule>) -> Self {
        AssertingInspector {
            rules: Mutex::new(rules),
        }
    }

    /// Convenience constructor: refuse any
    /// [`TraceEvent::StageSkipped`] whose reason is
    /// [`PipelineSkipReason::ScopeMatchedZeroCells`].
    ///
    /// This is the canonical guard for the focused_row_btop bug class.
    /// A test installs it on a recipe that *should* fire its shader; if
    /// the shader's scope ever matches zero cells (regression), the
    /// inspector panics with a clear message.
    pub fn forbid_zero_cell_scope_matches() -> Self {
        AssertingInspector::new(vec![ForbidRule::new(
            "no StageSkipped { ScopeMatchedZeroCells } — the recipe must produce non-zero matches",
            |event| {
                matches!(
                    event,
                    TraceEvent::StageSkipped {
                        reason: PipelineSkipReason::ScopeMatchedZeroCells { .. },
                        ..
                    }
                )
            },
        )])
    }
}

impl InspectionSink for AssertingInspector {
    fn report(&self, envelope: TraceEnvelope) {
        let rules = self
            .rules
            .lock()
            .expect("AssertingInspector mutex poisoned");
        for rule in rules.iter() {
            if (rule.matches)(&envelope.event) {
                panic!(
                    "AssertingInspector: forbidden event fired — rule: {}; event: {:?}",
                    rule.description, envelope.event
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AssertingInspector, ForbidRule};
    use crate::inspection::{
        InspectionSink, PipelineSkipReason, PipelineStageKind, RoleHistogram, TraceEnvelope,
        TraceEvent,
    };

    fn envelope(event: TraceEvent) -> TraceEnvelope {
        TraceEnvelope {
            event,
            frame_no: 0,
            t_ms: 0,
            recipe_id: None,
            seq_in_frame: 0,
        }
    }

    #[test]
    fn non_matching_event_does_not_panic() {
        let sink = AssertingInspector::forbid_zero_cell_scope_matches();
        sink.report(envelope(TraceEvent::StageEntered {
            kind: PipelineStageKind::Shader,
            step_id: 1,
            name: "FocusedRowGradient".to_string(),
            scope_summary: "Role(Text)".to_string(),
        }));
        // ok — no panic
    }

    #[test]
    #[should_panic(expected = "no StageSkipped { ScopeMatchedZeroCells }")]
    fn forbid_zero_cell_scope_matches_panics_on_matching_event() {
        let sink = AssertingInspector::forbid_zero_cell_scope_matches();
        sink.report(envelope(TraceEvent::StageSkipped {
            kind: PipelineStageKind::Shader,
            step_id: 1,
            reason: PipelineSkipReason::ScopeMatchedZeroCells {
                predicate: "Role(Text)".to_string(),
                role_histogram: RoleHistogram {
                    background: 320,
                    ..RoleHistogram::EMPTY
                },
            },
        }));
    }

    #[test]
    #[should_panic(expected = "custom rule")]
    fn custom_rule_with_description_appears_in_panic_message() {
        let sink = AssertingInspector::new(vec![ForbidRule::new(
            "custom rule — no Filter stage finished events",
            |event| {
                matches!(
                    event,
                    TraceEvent::StageFinished {
                        kind: PipelineStageKind::Filter,
                        ..
                    }
                )
            },
        )]);
        sink.report(envelope(TraceEvent::StageFinished {
            kind: PipelineStageKind::Filter,
            step_id: 1,
            cells_modified: 0,
            elapsed_ns: 0,
        }));
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_asserting_inspector.rs</FILE> - <DESC>AssertingInspector</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
