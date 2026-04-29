// <FILE>crates/tui-vfx-contract/src/cls_scope_kind.rs</FILE> - <DESC>Descriptor-supported scope kind enum</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 schema decision burn-down: mirror accepted built-in scope variants in descriptor support.</WCTX>
// <CLOG>0.2.0: MINOR — add modulo, non-empty, outer-band, and inner scope support kinds.
// 0.1.0: INIT — mirror ScopeSpec variants as supportable descriptor kinds.</CLOG>

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
    /// Descriptor supports row modulo scopes.
    ModuloRows,
    /// Descriptor supports column modulo scopes.
    ModuloColumns,
    /// Descriptor supports non-empty content scopes.
    NonEmpty,
    /// Descriptor supports the outer perimeter band.
    OuterBand,
    /// Descriptor supports cells inside the outer perimeter.
    Inner,
}

impl From<&ScopeSpec> for ScopeKind {
    fn from(scope: &ScopeSpec) -> Self {
        match scope {
            ScopeSpec::All => Self::All,
            ScopeSpec::Role { .. } => Self::Role,
            ScopeSpec::Rect { .. } => Self::Rect,
            ScopeSpec::RowRange { .. } => Self::RowRange,
            ScopeSpec::ColumnRange { .. } => Self::ColumnRange,
            ScopeSpec::ModuloRows { .. } => Self::ModuloRows,
            ScopeSpec::ModuloColumns { .. } => Self::ModuloColumns,
            ScopeSpec::NonEmpty => Self::NonEmpty,
            ScopeSpec::OuterBand => Self::OuterBand,
            ScopeSpec::Inner => Self::Inner,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_kind.rs</FILE> - <DESC>Descriptor-supported scope kind enum</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
