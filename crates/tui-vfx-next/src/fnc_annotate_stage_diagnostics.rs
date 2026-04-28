// <FILE>crates/tui-vfx-next/src/fnc_annotate_stage_diagnostics.rs</FILE> - <DESC>Annotate all diagnostics from one pipeline stage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase C deslop: keep SurfacePipeline focused by extracting multi-diagnostic annotation.</WCTX>
// <CLOG>0.1.0: ADD — move stage-outcome diagnostic mapping from cls_surface_pipeline into an OFPF fnc_ helper.</CLOG>

use crate::{ApplyOutcome, fnc_annotate_stage_diagnostic::annotate_stage_diagnostic};

/// Attach deterministic stage identity to every diagnostic in an outcome.
pub(crate) fn annotate_stage_diagnostics(
    outcome: &mut ApplyOutcome,
    stage_index: usize,
    stage_name: &str,
) {
    outcome.diagnostics = outcome
        .diagnostics
        .drain(..)
        .map(|diagnostic| annotate_stage_diagnostic(diagnostic, stage_index, stage_name))
        .collect();
}

// <FILE>crates/tui-vfx-next/src/fnc_annotate_stage_diagnostics.rs</FILE> - <DESC>Annotate all diagnostics from one pipeline stage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
