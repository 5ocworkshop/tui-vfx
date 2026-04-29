// <FILE>crates/tui-vfx-contract/src/cls_source_descriptor.rs</FILE> - <DESC>Surface source descriptor contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: define source descriptors before recipe schema work.</WCTX>
// <CLOG>0.1.0: INIT — add stable source identity, inputs, assets, output, lifecycle, and validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    AssetId, AssetRequirement, DescriptorValidationError, SourceId, SourceInputId, SourceInputSpec,
    SourceKind, SourceLifecycle, SourceOutputSpec,
};

/// Stable v3.1 descriptor for a source that produces an initial semantic surface.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceDescriptor {
    /// Stable canonical source identifier.
    pub id: SourceId,
    /// Descriptor version string owned by the source family.
    pub version: String,
    /// Human-facing display name.
    pub display_name: String,
    /// Optional human-facing category label for catalogs and docs.
    pub category: Option<String>,
    /// Broad source kind for planning and capability grouping.
    pub kind: SourceKind,
    /// Descriptor-local typed input specifications keyed by stable input id.
    #[schemars(transform = add_source_input_key_pattern)]
    pub inputs: BTreeMap<SourceInputId, SourceInputSpec>,
    /// Descriptor-local asset slots keyed by stable asset slot id.
    #[schemars(transform = add_asset_key_pattern)]
    pub assets: BTreeMap<AssetId, AssetRequirement>,
    /// Output semantic surface contract.
    pub output: SourceOutputSpec,
    /// Minimal lifecycle metadata for planning.
    pub lifecycle: SourceLifecycle,
}

impl SourceDescriptor {
    /// Validate descriptor identity, input specs, and asset slot ids.
    pub fn validate_contract(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidSourceId {
                id: self.id.clone(),
            });
        }
        for (id, input) in &self.inputs {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidSourceInputId { id: id.clone() });
            }
            input.validate()?;
        }
        for id in self.assets.keys() {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidAssetId { id: id.clone() });
            }
        }
        Ok(())
    }
}

fn add_source_input_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Source input ids may be dotted paths; each segment starts with an ASCII letter and then contains only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z][A-Za-z0-9_-]*)*$"
        })
        .to_value(),
    );
}

fn add_asset_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Asset slot ids must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_source_descriptor.rs</FILE> - <DESC>Surface source descriptor contract DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
