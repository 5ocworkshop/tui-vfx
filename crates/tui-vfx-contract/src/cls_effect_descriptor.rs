// <FILE>crates/tui-vfx-contract/src/cls_effect_descriptor.rs</FILE> - <DESC>Minimal effect descriptor contract DTO</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase G4: add descriptor-declared effect outputs.</WCTX>
// <CLOG>0.3.0: MINOR — add descriptor-local typed output spec map and validation.
// 0.2.0: MINOR — add descriptor-local typed input spec map and validation.
// 0.1.0: INIT — declare identity, domain, access, scope, write, and lifecycle capabilities.</CLOG>

use std::collections::BTreeMap;

use crate::{
    CellAccess, CellChannel, CellWritePolicy, DescriptorValidationError, EffectDomain, EffectId,
    EffectInputId, EffectInputSpec, EffectLifecycle, EffectOutputId, EffectOutputSpec,
    RoleWritePolicy, RoleWritePolicyKind, ScopeKind, ScopeSpec, ScopeSupport, WriteSupport,
};

/// Minimal durable v3.1 effect descriptor contract.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectDescriptor {
    /// Stable canonical effect identifier.
    pub id: EffectId,
    /// Descriptor version string owned by the effect family.
    pub version: String,
    /// Human-facing display name.
    pub display_name: String,
    /// Optional human-facing category label for catalogs and docs.
    pub category: Option<String>,
    /// Broad execution domain for planning and capability grouping.
    pub domain: EffectDomain,
    /// Cell channels the effect may read or write.
    pub cell_access: CellAccess,
    /// Scope kinds and spaces this effect supports.
    pub scope_support: ScopeSupport,
    /// Cell and role write policies this effect supports.
    pub write_support: WriteSupport,
    /// Descriptor-local typed input specifications keyed by stable input id.
    #[schemars(transform = add_effect_input_key_pattern)]
    pub inputs: BTreeMap<EffectInputId, EffectInputSpec>,
    /// Descriptor-local typed output specifications keyed by stable output id.
    #[schemars(transform = add_effect_output_key_pattern)]
    pub outputs: BTreeMap<EffectOutputId, EffectOutputSpec>,
    /// Minimal lifecycle metadata for planning.
    pub lifecycle: EffectLifecycle,
}

fn add_effect_input_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Effect input ids must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

fn add_effect_output_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Effect output ids must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

impl EffectDescriptor {
    /// Validate all descriptor-local input and output specifications.
    pub fn validate_io(&self) -> Result<(), DescriptorValidationError> {
        for (id, input) in &self.inputs {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidInputId { id: id.clone() });
            }
            input.validate()?;
        }

        for id in self.outputs.keys() {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidEffectOutputId { id: id.clone() });
            }
        }

        Ok(())
    }

    /// Validate all descriptor-local input and output specifications.
    pub fn validate_inputs(&self) -> Result<(), DescriptorValidationError> {
        self.validate_io()
    }

    /// Validate that the descriptor supports the requested scope kind.
    pub fn validate_scope(&self, scope: &ScopeSpec) -> Result<(), DescriptorValidationError> {
        if self.scope_support.supports_scope(scope) {
            Ok(())
        } else {
            Err(DescriptorValidationError::UnsupportedScopeKind {
                requested: ScopeKind::from(scope),
            })
        }
    }

    /// Validate that the descriptor supports the requested cell write policy.
    pub fn validate_cell_write_policy(
        &self,
        policy: CellWritePolicy,
    ) -> Result<(), DescriptorValidationError> {
        if self.write_support.supports_cell_policy(policy) {
            Ok(())
        } else {
            Err(DescriptorValidationError::UnsupportedCellWritePolicy { requested: policy })
        }
    }

    /// Validate that the descriptor supports the requested role write policy.
    pub fn validate_role_write_policy(
        &self,
        policy: &RoleWritePolicy,
    ) -> Result<(), DescriptorValidationError> {
        if self.write_support.supports_role_policy(policy) {
            Ok(())
        } else {
            Err(DescriptorValidationError::UnsupportedRoleWritePolicy {
                requested: RoleWritePolicyKind::from(policy),
            })
        }
    }

    /// Validate that the descriptor declares write access for a cell channel.
    pub fn validate_write_channel(
        &self,
        channel: CellChannel,
    ) -> Result<(), DescriptorValidationError> {
        if self.cell_access.can_write(channel) {
            Ok(())
        } else {
            Err(DescriptorValidationError::UndeclaredWriteChannel { channel })
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_descriptor.rs</FILE> - <DESC>Minimal effect descriptor contract DTO</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
