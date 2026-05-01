// <FILE>crates/tui-vfx-contract/src/cls_recipe_document.rs</FILE> - <DESC>Canonical v3.1 recipe document root DTO</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase J2: allow canonical recipes to reference external descriptor packs.</WCTX>
// <CLOG>0.3.0: MINOR — add descriptor pack refs and catalog-aware validation.
// 0.2.0: MINOR — add optional recipe-level lifecycle contract.
// 0.1.0: INIT — add recipe document root and delegate cross-document validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    AssetId, AssetSpec, DescriptorCatalog, DescriptorPackRef, DescriptorValidationError, GraphSpec,
    LifecycleSpec, RecipeId, RecipeMetadata, RecipeScene, SourceDescriptor, SourceId,
    SourceInstanceId, SourceSpec, TransitionId, TransitionSpec,
    fnc_validate_recipe_with_catalog::validate_recipe_with_catalog,
    orc_validate_recipe_document::validate_recipe_document,
};

/// Strict canonical v3.1 recipe document consumed after authoring canonicalization.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeDocument {
    /// Stable canonical recipe identifier.
    pub id: RecipeId,
    /// Canonical recipe contract version string.
    pub version: String,
    /// Human-facing metadata carried with the canonical document.
    pub metadata: RecipeMetadata,
    /// Optional recipe-level lifecycle semantics for enter, dwell, and exit.
    pub lifecycle: Option<LifecycleSpec>,
    /// Named native transitions available to scenes, elements, and future graph transition nodes.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(transform = add_transition_key_pattern)]
    pub transitions: BTreeMap<TransitionId, TransitionSpec>,
    /// Assets declared once and referenced structurally by source instances.
    #[schemars(transform = add_asset_key_pattern)]
    pub assets: BTreeMap<AssetId, AssetSpec>,
    /// External descriptor packs required to resolve source and effect descriptors.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub descriptor_packs: Vec<DescriptorPackRef>,
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
    /// Validate canonical recipe identity, lifecycle, assets, sources, graph, and scene references.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        validate_recipe_document(self)
    }

    /// Validate after resolving external descriptor packs from a loaded catalog.
    pub fn validate_with_catalog(
        &self,
        catalog: &DescriptorCatalog,
    ) -> Result<(), DescriptorValidationError> {
        validate_recipe_with_catalog(self, catalog)
    }
}

fn add_transition_key_pattern(schema: &mut schemars::Schema) {
    add_key_pattern(schema, "Transition ids", "^[A-Za-z][A-Za-z0-9_-]*$");
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
// <VERS>END OF VERSION: 0.3.0</VERS>
