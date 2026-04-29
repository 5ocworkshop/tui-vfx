// <FILE>crates/tui-vfx-player/src/cls_player_primitive_field_coverage.rs</FILE> - <DESC>Primitive input field coverage report DTOs</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Player report de-slop: keep primitive field coverage metadata phase-neutral.</WCTX>
// <CLOG>0.2.1: PATCH — collapse historical DTO metadata into latest-change context.</CLOG>

/// Aggregate primitive input-field coverage report.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveFieldCoverageReport {
    /// Stable primitive-field coverage schema label.
    pub schema_version: &'static str,
    /// Root path or invocation label.
    pub root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<crate::DescriptorPackReport>,
    /// Aggregate coverage counts.
    pub summary: PlayerPrimitiveFieldCoverageSummary,
    /// Per-recipe primitive field coverage entries.
    pub recipes: Vec<PlayerPrimitiveFieldCoverageRecipe>,
}

/// Aggregate primitive input-field coverage counts.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveFieldCoverageSummary {
    /// Total recipes scanned.
    pub total_recipes: usize,
    /// Total source/effect primitive instances scanned.
    pub total_primitive_instances: usize,
    /// Authored input fields present in scanned recipes.
    pub used_input_fields: usize,
    /// Authored input fields covered by explicit current player handling.
    pub handled_input_fields: usize,
    /// Authored input fields not consumed by the current player adapter contract.
    pub used_but_unhandled_input_fields: usize,
    /// Descriptor fields not exercised by the scanned fixture corpus.
    pub declared_but_unused_input_fields: usize,
    /// Authored input fields with no descriptor declaration.
    pub missing_descriptor_input_fields: usize,
    /// Fields needing a schema-level decision before adapter work can proceed.
    pub schema_decision_needed_fields: usize,
}

/// Per-recipe primitive field coverage.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveFieldCoverageRecipe {
    /// Recipe file path.
    pub recipe_path: String,
    /// Coarse recipe scan status.
    pub status: String,
    /// Primitive instances found in this recipe.
    pub primitive_instances: Vec<PlayerPrimitiveFieldCoverageInstance>,
    /// Scan errors for this recipe.
    pub errors: Vec<String>,
    /// Scan warnings for this recipe.
    pub warnings: Vec<String>,
}

/// Per primitive-instance input field coverage.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveFieldCoverageInstance {
    /// Primitive kind: source or effect.
    pub kind: String,
    /// Primitive descriptor id.
    pub descriptor_id: String,
    /// Graph node id for effect instances.
    pub node_id: Option<String>,
    /// Source instance id for source instances.
    pub source_instance_id: Option<String>,
    /// Descriptor domain when available.
    pub domain: Option<String>,
    /// Authored input names present on the instance.
    pub used_inputs: Vec<String>,
    /// Inputs declared by the descriptor.
    pub descriptor_inputs: Vec<String>,
    /// Authored inputs consumed by the current player tooling contract.
    pub adapter_handled_inputs: Vec<String>,
    /// Authored inputs declared by descriptors but not handled by player tooling.
    pub used_but_unhandled_inputs: Vec<String>,
    /// Descriptor inputs not authored by this instance.
    pub declared_but_unused_inputs: Vec<String>,
    /// Authored inputs missing from the descriptor.
    pub missing_descriptor_inputs: Vec<String>,
    /// Coarse classification for scan-friendly gating.
    pub classification: String,
    /// Recommended next action for this instance classification.
    pub recommendation: String,
}

impl PlayerPrimitiveFieldCoverageReport {
    /// Build a primitive field coverage report.
    pub fn new(
        root: String,
        descriptor_packs: Vec<crate::DescriptorPackReport>,
        summary: PlayerPrimitiveFieldCoverageSummary,
        recipes: Vec<PlayerPrimitiveFieldCoverageRecipe>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.primitiveFieldCoverage.1",
            root,
            descriptor_packs,
            summary,
            recipes,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_primitive_field_coverage.rs</FILE> - <DESC>Primitive input field coverage report DTOs</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
