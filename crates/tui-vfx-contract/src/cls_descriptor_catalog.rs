// <FILE>crates/tui-vfx-contract/src/cls_descriptor_catalog.rs</FILE> - <DESC>Loaded descriptor pack catalog DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: validate recipes against externally loaded descriptor packs.</WCTX>
// <CLOG>0.1.0: INIT — add descriptor catalog container and validation helper.</CLOG>

use std::collections::BTreeMap;

use crate::{DescriptorPack, DescriptorPackId, DescriptorValidationError};

/// Loaded collection of descriptor packs available to recipe validation.
#[derive(
    Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DescriptorCatalog {
    /// Descriptor packs keyed by stable pack id.
    #[serde(default)]
    #[schemars(transform = add_pack_key_pattern)]
    pub packs: BTreeMap<DescriptorPackId, DescriptorPack>,
}

impl DescriptorCatalog {
    /// Validate all loaded descriptor packs and key consistency.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        for (id, pack) in &self.packs {
            if &pack.id != id {
                return Err(DescriptorValidationError::DescriptorPackIdMismatch {
                    key: id.clone(),
                    pack: pack.id.clone(),
                });
            }
            pack.validate()?;
        }
        Ok(())
    }

    /// Return a descriptor pack by id.
    pub fn pack(&self, id: &DescriptorPackId) -> Option<&DescriptorPack> {
        self.packs.get(id)
    }
}

fn add_pack_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Descriptor pack ids must follow the canonical dotted identifier shape.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*(\\.[A-Za-z0-9][A-Za-z0-9_-]*)*$"
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_catalog.rs</FILE> - <DESC>Loaded descriptor pack catalog DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
