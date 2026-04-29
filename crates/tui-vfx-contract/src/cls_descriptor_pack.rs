// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack.rs</FILE> - <DESC>Shared source/effect descriptor pack DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: store reusable primitive descriptors outside recipe fixtures.</WCTX>
// <CLOG>0.1.0: INIT — add schema-backed descriptor pack with source and effect descriptors.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorPackId, DescriptorValidationError, EffectDescriptor, EffectId, SourceDescriptor,
    SourceId,
};

/// Shared descriptor pack that can provide source and effect descriptors to recipes.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DescriptorPack {
    /// Stable descriptor pack id.
    pub id: DescriptorPackId,
    /// Descriptor pack version string.
    pub version: String,
    /// Human-facing pack name for docs and reports.
    pub display_name: String,
    /// Optional human-facing category label for catalogs.
    pub category: Option<String>,
    /// Source descriptors supplied by this pack.
    #[serde(default)]
    #[schemars(transform = add_source_descriptor_key_pattern)]
    pub source_descriptors: BTreeMap<SourceId, SourceDescriptor>,
    /// Effect descriptors supplied by this pack.
    #[serde(default)]
    #[schemars(transform = add_effect_descriptor_key_pattern)]
    pub effects: BTreeMap<EffectId, EffectDescriptor>,
}

impl DescriptorPack {
    /// Validate descriptor pack identity and nested source/effect descriptors.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidDescriptorPackId {
                id: self.id.clone(),
            });
        }
        validate_sources(self)?;
        validate_effects(self)
    }
}

fn validate_sources(pack: &DescriptorPack) -> Result<(), DescriptorValidationError> {
    for (id, descriptor) in &pack.source_descriptors {
        if &descriptor.id != id {
            return Err(DescriptorValidationError::PackSourceDescriptorIdMismatch {
                pack: pack.id.clone(),
                key: id.clone(),
                source: descriptor.id.clone(),
            });
        }
        descriptor.validate_contract()?;
    }
    Ok(())
}

fn validate_effects(pack: &DescriptorPack) -> Result<(), DescriptorValidationError> {
    for (id, descriptor) in &pack.effects {
        if &descriptor.id != id {
            return Err(DescriptorValidationError::PackEffectDescriptorIdMismatch {
                pack: pack.id.clone(),
                key: id.clone(),
                effect: descriptor.id.clone(),
            });
        }
        descriptor.validate_io()?;
    }
    Ok(())
}

fn add_source_descriptor_key_pattern(schema: &mut schemars::Schema) {
    add_dotted_key_pattern(schema, "Source descriptor ids");
}

fn add_effect_descriptor_key_pattern(schema: &mut schemars::Schema) {
    add_dotted_key_pattern(schema, "Effect descriptor ids");
}

fn add_dotted_key_pattern(schema: &mut schemars::Schema, description_prefix: &str) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": format!("{description_prefix} must follow the canonical dotted identifier shape."),
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z][A-Za-z0-9_-]*)*$"
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack.rs</FILE> - <DESC>Shared source/effect descriptor pack DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
