// <FILE>crates/tui-vfx-next/src/cls_pipeline_outcome.rs</FILE> - <DESC>Ordered pipeline execution outcome DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C: expose final current surface plus ordered diagnostics.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.1.0: ADD — record final pipeline surface, aggregate counts, and stage diagnostics.</CLOG>

use crate::{ApplyOutcome, Surface, SurfaceDiagnostic};

/// Result of an ordered multi-stage surface pipeline.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineOutcome {
    /// Final current surface after all stages have completed.
    pub surface: Surface,
    /// Total matched cells across all stages.
    pub matched_cells: usize,
    /// Total written cells across all stages.
    pub written_cells: usize,
    /// Stage-aware diagnostics in deterministic stage order.
    pub diagnostics: Vec<SurfaceDiagnostic>,
}

impl PipelineOutcome {
    /// Build a pipeline outcome from the final surface and accumulated stage outcomes.
    pub fn from_stage_outcomes(surface: Surface, outcomes: Vec<ApplyOutcome>) -> Self {
        let matched_cells = outcomes.iter().map(|outcome| outcome.matched_cells).sum();
        let written_cells = outcomes.iter().map(|outcome| outcome.written_cells).sum();
        let diagnostics = outcomes
            .into_iter()
            .flat_map(|outcome| outcome.diagnostics)
            .collect();
        Self {
            surface,
            matched_cells,
            written_cells,
            diagnostics,
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_pipeline_outcome.rs</FILE> - <DESC>Ordered pipeline execution outcome DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
