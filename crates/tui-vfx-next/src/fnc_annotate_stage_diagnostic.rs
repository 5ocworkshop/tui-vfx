// <FILE>crates/tui-vfx-next/src/fnc_annotate_stage_diagnostic.rs</FILE> - <DESC>Attach stable stage identity to a diagnostic</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase C: diagnostics emitted by pipeline stages must name their stage.</WCTX>
// <CLOG>0.1.0: ADD — deterministic stage path and message prefix helper.</CLOG>

use crate::SurfaceDiagnostic;

/// Attach deterministic stage identity to one diagnostic.
pub fn annotate_stage_diagnostic(
    mut diagnostic: SurfaceDiagnostic,
    stage_index: usize,
    stage_name: &str,
) -> SurfaceDiagnostic {
    diagnostic.message = format!("stage `{stage_name}`: {}", diagnostic.message);
    diagnostic.path = Some(format!("pipeline.stage[{stage_index}].{stage_name}"));
    diagnostic
}

// <FILE>crates/tui-vfx-next/src/fnc_annotate_stage_diagnostic.rs</FILE> - <DESC>Attach stable stage identity to a diagnostic</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
