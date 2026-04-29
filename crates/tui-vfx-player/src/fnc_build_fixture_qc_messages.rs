// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_messages.rs</FILE> - <DESC>Build fixture QC messages</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: keep fixture QC warning/error message construction separate.</WCTX>
// <CLOG>0.1.0: INIT — extract fixture QC warning and error builders.</CLOG>

use crate::{
    PlayerFixtureQcRecipe, PlayerPrimitiveAdapterGapReport, PlayerPrimitiveFieldCoverageReport,
    PlayerSummary, fnc_build_fixture_qc_summary::unresolved_adapter_gaps,
};

/// Build non-fatal fixture QC warnings.
pub(crate) fn build_fixture_qc_warnings(
    field_coverage: &PlayerPrimitiveFieldCoverageReport,
    adapter_gap: &PlayerPrimitiveAdapterGapReport,
    render_summary: &PlayerSummary,
) -> Vec<String> {
    let mut warnings = Vec::new();
    if render_summary.unsupported > 0 {
        warnings.push(format!(
            "{} recipes reported unsupported",
            render_summary.unsupported
        ));
    }
    if field_coverage.summary.used_but_unhandled_input_fields > 0 {
        warnings.push(format!(
            "{} primitive fields are used but unhandled",
            field_coverage.summary.used_but_unhandled_input_fields
        ));
    }
    let unresolved = unresolved_adapter_gaps(adapter_gap);
    if unresolved > 0 {
        warnings.push(format!(
            "{unresolved} primitive adapter gaps remain unresolved"
        ));
    }
    warnings
}

/// Build fatal fixture QC error messages.
pub(crate) fn build_fixture_qc_errors(recipes: &[PlayerFixtureQcRecipe]) -> Vec<String> {
    recipes
        .iter()
        .flat_map(|recipe| {
            recipe
                .errors
                .iter()
                .map(|error| format!("{}: {error}", recipe.recipe_path))
        })
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_messages.rs</FILE> - <DESC>Build fixture QC messages</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
