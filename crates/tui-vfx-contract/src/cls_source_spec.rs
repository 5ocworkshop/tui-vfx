// <FILE>crates/tui-vfx-contract/src/cls_source_spec.rs</FILE> - <DESC>Canonical source instance contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: define source specs that bind inputs and structural asset refs.</WCTX>
// <CLOG>0.1.0: INIT — add source instance DTO and delegate validation to OFPF-sized helper.</CLOG>

use std::collections::BTreeMap;

use crate::{
    AssetId, AssetRef, AssetSpec, DescriptorValidationError, GraphValueKinds, ParameterId,
    ParameterSpec, SignalId, SignalSpec, SourceDescriptor, SourceId, SourceInputId, ValueSource,
    orc_validate_source_spec::validate_source_spec,
};

/// Canonical source instance referencing a source descriptor and supplying inputs/assets.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceSpec {
    /// Source descriptor id used by this instance.
    pub source: SourceId,
    /// Declarative values supplied to descriptor-local source inputs.
    #[schemars(transform = add_source_input_key_pattern)]
    pub inputs: BTreeMap<SourceInputId, ValueSource>,
    /// Structural asset refs supplied to descriptor-local source asset slots.
    #[serde(default)]
    #[schemars(transform = add_asset_key_pattern)]
    pub assets: BTreeMap<AssetId, AssetRef>,
}

impl SourceSpec {
    /// Validate this source instance against source descriptors, assets, and value sources.
    pub fn validate(
        &self,
        sources: &BTreeMap<SourceId, SourceDescriptor>,
        assets: &BTreeMap<AssetId, AssetSpec>,
        parameters: &BTreeMap<ParameterId, ParameterSpec>,
        signals: &BTreeMap<SignalId, SignalSpec>,
        graph_values: Option<&GraphValueKinds>,
    ) -> Result<(), DescriptorValidationError> {
        validate_source_spec(self, sources, assets, parameters, signals, graph_values)
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

// <FILE>crates/tui-vfx-contract/src/cls_source_spec.rs</FILE> - <DESC>Canonical source instance contract DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
