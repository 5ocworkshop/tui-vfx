// <FILE>crates/tui-vfx-contract/src/cls_recipe_document.rs</FILE> - <DESC>Canonical v3.1 recipe document root DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: package stable contract pieces into one canonical recipe root.</WCTX>
// <CLOG>0.1.0: INIT — add recipe document root and delegate cross-document validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    AssetId, AssetSpec, DescriptorValidationError, GraphSpec, RecipeId, RecipeMetadata,
    RecipeScene, SourceDescriptor, SourceId, SourceInstanceId, SourceSpec,
    orc_validate_recipe_document::validate_recipe_document,
};

/// Strict canonical v3.1 recipe document consumed after authoring/lowering.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeDocument {
    /// Stable canonical recipe identifier.
    pub id: RecipeId,
    /// Canonical recipe contract version string.
    pub version: String,
    /// Human-facing metadata carried with the canonical document.
    pub metadata: RecipeMetadata,
    /// Assets declared once and referenced structurally by source instances.
    #[schemars(transform = add_asset_key_pattern)]
    pub assets: BTreeMap<AssetId, AssetSpec>,
    /// Source descriptors available to source instances.
    #[schemars(transform = add_source_descriptor_key_pattern)]
    pub source_descriptors: BTreeMap<SourceId, SourceDescriptor>,
    /// Source instances keyed by recipe-local source instance id.
    #[schemars(transform = add_source_instance_key_pattern)]
    pub sources: BTreeMap<SourceInstanceId, SourceSpec>,
    /// Canonical graph containing parameters, signals, bindings, effects, nodes, and topology.
    pub graph: GraphSpec,
    /// Scene declarations referencing source-produced surfaces and optional element pipelines.
    pub scenes: Vec<RecipeScene>,
}

impl RecipeDocument {
    /// Validate canonical recipe identity, assets, sources, graph, and scene references.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        validate_recipe_document(self)
    }
}

fn add_asset_key_pattern(schema: &mut schemars::Schema) {
    add_key_pattern(schema, "Asset ids", "^[A-Za-z][A-Za-z0-9_-]*$");
}

fn add_source_descriptor_key_pattern(schema: &mut schemars::Schema) {
    add_key_pattern(
        schema,
        "Source descriptor ids",
        "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z][A-Za-z0-9_-]*)*$",
    );
}

fn add_source_instance_key_pattern(schema: &mut schemars::Schema) {
    add_key_pattern(schema, "Source instance ids", "^[A-Za-z][A-Za-z0-9_-]*$");
}

fn add_key_pattern(schema: &mut schemars::Schema, description_prefix: &str, pattern: &str) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": format!("{description_prefix} must follow the canonical identifier shape."),
            "type": "string",
            "pattern": pattern
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_recipe_document.rs</FILE> - <DESC>Canonical v3.1 recipe document root DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
