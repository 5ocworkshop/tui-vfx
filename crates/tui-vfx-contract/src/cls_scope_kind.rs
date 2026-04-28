// <FILE>crates/tui-vfx-contract/src/cls_scope_kind.rs</FILE> - <DESC>Descriptor-supported scope kind enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: add descriptor scope support vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — mirror ScopeSpec variants as supportable descriptor kinds.</CLOG>

use crate::ScopeSpec;

/// Scope shape a descriptor can explicitly support.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ScopeKind {
    /// Descriptor supports all-cell scopes.
    All,
    /// Descriptor supports role-based scopes.
    Role,
    /// Descriptor supports rectangular geometry scopes.
    Rect,
    /// Descriptor supports row range geometry scopes.
    RowRange,
    /// Descriptor supports column range geometry scopes.
    ColumnRange,
}

impl From<&ScopeSpec> for ScopeKind {
    fn from(scope: &ScopeSpec) -> Self {
        match scope {
            ScopeSpec::All => Self::All,
            ScopeSpec::Role { .. } => Self::Role,
            ScopeSpec::Rect { .. } => Self::Rect,
            ScopeSpec::RowRange { .. } => Self::RowRange,
            ScopeSpec::ColumnRange { .. } => Self::ColumnRange,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_kind.rs</FILE> - <DESC>Descriptor-supported scope kind enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
