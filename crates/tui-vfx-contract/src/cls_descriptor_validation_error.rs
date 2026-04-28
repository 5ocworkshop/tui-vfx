// <FILE>crates/tui-vfx-contract/src/cls_descriptor_validation_error.rs</FILE> - <DESC>Descriptor capability validation error enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: report unsupported descriptor capability requests.</WCTX>
// <CLOG>0.1.0: INIT — add structured validation errors for scope, write policy, and channel checks.</CLOG>

use crate::{CellChannel, CellWritePolicy, RoleWritePolicyKind, ScopeKind};

/// Structured error returned when a request exceeds descriptor capabilities.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum DescriptorValidationError {
    /// Requested scope kind is not declared in descriptor support.
    UnsupportedScopeKind {
        /// Scope kind requested by the operation.
        requested: ScopeKind,
    },
    /// Requested cell write policy is not declared in descriptor support.
    UnsupportedCellWritePolicy {
        /// Cell write policy requested by the operation.
        requested: CellWritePolicy,
    },
    /// Requested role write policy kind is not declared in descriptor support.
    UnsupportedRoleWritePolicy {
        /// Role write policy kind requested by the operation.
        requested: RoleWritePolicyKind,
    },
    /// Requested channel write is outside descriptor-declared write access.
    UndeclaredWriteChannel {
        /// Cell channel requested for writing.
        channel: CellChannel,
    },
}

impl DescriptorValidationError {
    /// Return the requested scope kind when this is a scope support error.
    pub fn requested_scope_kind(&self) -> Option<ScopeKind> {
        match self {
            Self::UnsupportedScopeKind { requested } => Some(*requested),
            _ => None,
        }
    }

    /// Return the requested role policy kind when this is a role policy error.
    pub fn requested_role_policy_kind(&self) -> Option<RoleWritePolicyKind> {
        match self {
            Self::UnsupportedRoleWritePolicy { requested } => Some(*requested),
            _ => None,
        }
    }

    /// Return the requested channel when this is a channel access error.
    pub fn requested_channel(&self) -> Option<CellChannel> {
        match self {
            Self::UndeclaredWriteChannel { channel } => Some(*channel),
            _ => None,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_validation_error.rs</FILE> - <DESC>Descriptor capability validation error enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
