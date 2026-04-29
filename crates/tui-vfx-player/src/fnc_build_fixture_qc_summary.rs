// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_summary.rs</FILE> - <DESC>Build fixture QC summary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: keep fixture QC aggregate status separate.</WCTX>
// <CLOG>0.1.0: INIT — extract fixture QC summary and status calculation.</CLOG>

use crate::{
    PlayerFixtureQcRecipe, PlayerFixtureQcSummary, PlayerPrimitiveAdapterGapReport,
    PlayerPrimitiveFieldCoverageReport, PlayerRunReport, PlayerVisualFrameReport,
};

/// Build the aggregate fixture QC summary.
pub(crate) fn build_fixture_qc_summary(
    render: &PlayerRunReport,
    visual_frame: &PlayerVisualFrameReport,
    field_coverage: &PlayerPrimitiveFieldCoverageReport,
    adapter_gap: &PlayerPrimitiveAdapterGapReport,
    timeline_smoke_passed: bool,
    diff_smoke_passed: bool,
    recipes: &[PlayerFixtureQcRecipe],
) -> PlayerFixtureQcSummary {
    let adapter_gap_unresolved = unresolved_adapter_gaps(adapter_gap);
    let validation_errors = recipes.iter().filter(|recipe| !recipe.validated).count();
    PlayerFixtureQcSummary {
        total_recipes: render.summary.total,
        validated: render.summary.total.saturating_sub(validation_errors),
        validation_errors,
        rendered: render.summary.rendered,
        unsupported: render.summary.unsupported,
        player_errors: render.summary.errors,
        visual_frames: visual_frame.frames.len(),
        field_coverage_unhandled: field_coverage.summary.used_but_unhandled_input_fields,
        adapter_gap_unresolved,
        timeline_smoke_passed,
        diff_smoke_passed,
        overall_status: overall_status(
            render,
            field_coverage,
            adapter_gap_unresolved,
            validation_errors,
            timeline_smoke_passed,
            diff_smoke_passed,
        ),
    }
}

/// Count unresolved adapter blocker classes.
pub(crate) fn unresolved_adapter_gaps(report: &PlayerPrimitiveAdapterGapReport) -> usize {
    report.summary.still_unsupported
        + report.summary.blocked_by_styled_cell_substrate
        + report.summary.blocked_by_semantic_decision
        + report.summary.missing_descriptor
}

fn overall_status(
    render: &PlayerRunReport,
    field_coverage: &PlayerPrimitiveFieldCoverageReport,
    adapter_gap_unresolved: usize,
    validation_errors: usize,
    timeline_smoke_passed: bool,
    diff_smoke_passed: bool,
) -> String {
    if has_failures(render, field_coverage, validation_errors) {
        "fail"
    } else if has_warnings(
        render,
        field_coverage,
        adapter_gap_unresolved,
        timeline_smoke_passed,
        diff_smoke_passed,
    ) {
        "warn"
    } else {
        "pass"
    }
    .to_string()
}

fn has_failures(
    render: &PlayerRunReport,
    field_coverage: &PlayerPrimitiveFieldCoverageReport,
    validation_errors: usize,
) -> bool {
    validation_errors > 0
        || render.summary.errors > 0
        || field_coverage.summary.missing_descriptor_input_fields > 0
}

fn has_warnings(
    render: &PlayerRunReport,
    field_coverage: &PlayerPrimitiveFieldCoverageReport,
    adapter_gap_unresolved: usize,
    timeline_smoke_passed: bool,
    diff_smoke_passed: bool,
) -> bool {
    render.summary.unsupported > 0
        || field_coverage.summary.used_but_unhandled_input_fields > 0
        || adapter_gap_unresolved > 0
        || !timeline_smoke_passed
        || !diff_smoke_passed
}

// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_summary.rs</FILE> - <DESC>Build fixture QC summary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
