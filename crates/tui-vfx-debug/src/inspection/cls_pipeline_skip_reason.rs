// <FILE>crates/tui-vfx-debug/src/inspection/cls_pipeline_skip_reason.rs</FILE> - <DESC>PipelineSkipReason — structured reason for a StageSkipped event</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — structured payload for StageSkipped events. Without an explicit skip reason a stage that runs against a zero-cell scope is indistinguishable from a stage that ran successfully with a no-op input (the focused_row_btop bug class).</WCTX>
// <CLOG>0.1.0: initial enum with EmptyArea, ScopeMatchedZeroCells (carries predicate string + role histogram), DisabledByPolicy, BudgetExceeded variants.</CLOG>

//! Structured reason a pipeline stage skipped iteration.
//!
//! The shipped per-cell `*Applied` events do not fire when a stage's
//! scope matches zero cells, which leaves no evidence at all on the
//! event stream. `PipelineSkipReason` is the payload that turns a
//! silent skip into a signal: the variant names *why* the stage skipped,
//! and the discriminator carries the data needed to reproduce the
//! decision.
//!
//! `ScopeMatchedZeroCells` is the load-bearing variant for the
//! focused_row_btop case study — it carries both the predicate summary
//! string and the per-role cell histogram so a consumer can see "scope
//! `Role(Text)` matched zero cells out of 320, all of which were
//! tagged `Background`" without re-running the pipeline.

use serde::{Deserialize, Serialize};

use super::cls_role_histogram::RoleHistogram;

/// Structured reason a pipeline stage was skipped.
///
/// Attached to `crate::inspection::TraceEvent::StageSkipped`. Variants
/// are forward-compatible — new variants can be added without breaking
/// existing tapes (consumers `match` non-exhaustively today).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineSkipReason {
    /// The stage's effective area is empty (e.g. zero width or height).
    EmptyArea,
    /// The stage's scope predicate matched zero cells.
    ///
    /// `predicate` is the closed-vocabulary scope summary string (e.g.
    /// `"Role(Text)"`, `"And(RowRange,Channel(Background))"`).
    /// `role_histogram` shows what the predicate saw — which is what
    /// makes a role-map mismatch (the focused_row_btop bug class) visible
    /// in one event.
    ScopeMatchedZeroCells {
        /// Closed-vocabulary summary of the scope predicate that matched zero cells.
        predicate: String,
        /// Per-role cell counts the predicate visited; sum equals total cells in the area.
        role_histogram: RoleHistogram,
    },
    /// The stage was disabled by a runtime policy (feature flag,
    /// per-recipe disable, conditional compile-out, etc.).
    DisabledByPolicy {
        /// Free-form policy name that disabled the stage (e.g. `"shadow_disabled_by_recipe"`).
        policy: String,
    },
    /// The stage exceeded its per-stage time budget.
    BudgetExceeded {
        /// The budget the stage exceeded, in nanoseconds.
        budget_ns: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::{PipelineSkipReason, RoleHistogram};

    #[test]
    fn empty_area_round_trips() {
        let r = PipelineSkipReason::EmptyArea;
        let json = serde_json::to_string(&r).expect("serialize");
        let back: PipelineSkipReason = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(r, back);
    }

    #[test]
    fn scope_matched_zero_cells_round_trips() {
        let r = PipelineSkipReason::ScopeMatchedZeroCells {
            predicate: "Role(Text)".to_string(),
            role_histogram: RoleHistogram {
                background: 320,
                text: 0,
                border: 0,
                indicator: 0,
                highlight: 0,
            },
        };
        let json = serde_json::to_string(&r).expect("serialize");
        let back: PipelineSkipReason = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(r, back);
    }

    #[test]
    fn disabled_by_policy_round_trips() {
        let r = PipelineSkipReason::DisabledByPolicy {
            policy: "shadow_disabled_by_recipe".to_string(),
        };
        let json = serde_json::to_string(&r).expect("serialize");
        let back: PipelineSkipReason = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(r, back);
    }

    #[test]
    fn budget_exceeded_round_trips() {
        let r = PipelineSkipReason::BudgetExceeded {
            budget_ns: 16_700_000,
        };
        let json = serde_json::to_string(&r).expect("serialize");
        let back: PipelineSkipReason = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(r, back);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_pipeline_skip_reason.rs</FILE> - <DESC>PipelineSkipReason</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
