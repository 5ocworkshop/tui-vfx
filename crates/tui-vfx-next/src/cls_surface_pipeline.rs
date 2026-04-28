// <FILE>crates/tui-vfx-next/src/cls_surface_pipeline.rs</FILE> - <DESC>Phase C ordered current-surface pipeline</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>New kernel Phase D0 verifier fix: document schema-visible pipeline stage list.</WCTX>
// <CLOG>0.4.1: DOC — add rustdoc for the schema-visible stages field.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.1.1: REFACTOR — delegate diagnostic annotation mapping to fnc_annotate_stage_diagnostics.</CLOG>

use crate::{
    PipelineOutcome, PipelineStage, Surface,
    fnc_annotate_stage_diagnostics::annotate_stage_diagnostics,
};

/// Ordered multi-stage surface pipeline.
#[derive(
    Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SurfacePipeline {
    /// Ordered stage list. Later stages read the surface produced by earlier stages.
    stages: Vec<PipelineStage>,
}

impl SurfacePipeline {
    /// Create an empty ordered pipeline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a stage. Stage order is semantic.
    pub fn then(mut self, stage: PipelineStage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Execute stages from the initial surface to a final current surface.
    pub fn run(&self, initial: &Surface) -> PipelineOutcome {
        let mut current = initial.clone();
        let mut outcomes = Vec::with_capacity(self.stages.len());
        for (index, stage) in self.stages.iter().enumerate() {
            let mut next = current.clone();
            let mut outcome = stage.apply(&current, &mut next);
            annotate_stage_diagnostics(&mut outcome, index, stage.name());
            outcomes.push(outcome);
            current = next;
        }
        PipelineOutcome::from_stage_outcomes(current, outcomes)
    }
}

// <FILE>crates/tui-vfx-next/src/cls_surface_pipeline.rs</FILE> - <DESC>Phase C ordered current-surface pipeline</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
