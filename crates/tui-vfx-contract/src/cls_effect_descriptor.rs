// <FILE>crates/tui-vfx-contract/src/cls_effect_descriptor.rs</FILE> - <DESC>Minimal effect descriptor contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: add schema-backed durable effect descriptor model.</WCTX>
// <CLOG>0.1.0: INIT — declare identity, domain, access, scope, write, and lifecycle capabilities.</CLOG>

use crate::{
    CellAccess, CellChannel, CellWritePolicy, DescriptorValidationError, EffectDomain, EffectId,
    EffectLifecycle, RoleWritePolicy, RoleWritePolicyKind, ScopeKind, ScopeSpec, ScopeSupport,
    WriteSupport,
};

/// Minimal durable v3.1 effect descriptor contract.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
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
    /// Minimal lifecycle metadata for planning.
    pub lifecycle: EffectLifecycle,
}

impl EffectDescriptor {
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
// <VERS>END OF VERSION: 0.1.0</VERS>
