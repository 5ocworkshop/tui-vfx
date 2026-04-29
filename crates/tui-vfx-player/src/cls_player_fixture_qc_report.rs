// <FILE>crates/tui-vfx-player/src/cls_player_fixture_qc_report.rs</FILE> - <DESC>Fixture corpus QC report DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: compose fixture corpus reports into one QC gate.</WCTX>
// <CLOG>0.1.0: INIT — add schema-labeled fixture QC aggregate report.</CLOG>

use crate::DescriptorPackReport;

/// Aggregate clean-room fixture corpus quality report.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFixtureQcReport {
    /// Stable fixture QC schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate fixture QC counts and final status.
    pub summary: PlayerFixtureQcSummary,
    /// Composed existing player evidence reports.
    pub reports: PlayerFixtureQcReports,
    /// Per-recipe validation/render status summary.
    pub recipes: Vec<PlayerFixtureQcRecipe>,
    /// Non-fatal QC warnings.
    pub warnings: Vec<String>,
    /// Fatal QC errors.
    pub errors: Vec<String>,
}

/// Aggregate fixture QC counts.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFixtureQcSummary {
    /// Total recipes scanned.
    pub total_recipes: usize,
    /// Recipes that passed contract validation.
    pub validated: usize,
    /// Recipes that failed contract validation.
    pub validation_errors: usize,
    /// Recipes that rendered through the player.
    pub rendered: usize,
    /// Recipes that reported unsupported player adapters.
    pub unsupported: usize,
    /// Recipes that reported player errors.
    pub player_errors: usize,
    /// Visual frames produced by the visual-frame evidence surface.
    pub visual_frames: usize,
    /// Authored primitive fields not handled by current player adapters.
    pub field_coverage_unhandled: usize,
    /// Unresolved primitive adapter gap count.
    pub adapter_gap_unresolved: usize,
    /// Whether the first-recipe timeline smoke passed.
    pub timeline_smoke_passed: bool,
    /// Whether the first-recipe diff smoke passed.
    pub diff_smoke_passed: bool,
    /// Overall fixture QC status: pass, warn, or fail.
    pub overall_status: String,
}

/// Existing report surfaces embedded in fixture QC output.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFixtureQcReports {
    /// Recursive render-recipe evidence.
    pub render: serde_json::Value,
    /// Recursive visual-frame evidence.
    pub visual_frame: serde_json::Value,
    /// Primitive field coverage evidence.
    pub field_coverage: serde_json::Value,
    /// Primitive adapter gap evidence.
    pub adapter_gap: serde_json::Value,
    /// Timeline smoke evidence for the first recipe.
    pub timeline: Option<serde_json::Value>,
    /// Frame diff smoke evidence for the first recipe.
    pub diff: Option<serde_json::Value>,
}

/// Per-recipe fixture QC status.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFixtureQcRecipe {
    /// Recipe path.
    pub recipe_path: String,
    /// Whether contract validation passed.
    pub validated: bool,
    /// Player render status.
    pub player_status: String,
    /// Recipe render hash when available.
    pub render_hash: u64,
    /// Recipe non-empty cell count when available.
    pub non_empty_cells: usize,
    /// Validation/player diagnostics.
    pub errors: Vec<String>,
}

impl PlayerFixtureQcReport {
    /// Build a fixture QC report.
    pub fn new(
        root: String,
        descriptor_packs: Vec<DescriptorPackReport>,
        summary: PlayerFixtureQcSummary,
        reports: PlayerFixtureQcReports,
        recipes: Vec<PlayerFixtureQcRecipe>,
        warnings: Vec<String>,
        errors: Vec<String>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.fixtureQcReport.1",
            root,
            descriptor_packs,
            summary,
            reports,
            recipes,
            warnings,
            errors,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_fixture_qc_report.rs</FILE> - <DESC>Fixture corpus QC report DTOs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
