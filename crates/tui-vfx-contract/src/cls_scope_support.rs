// <FILE>crates/tui-vfx-contract/src/cls_scope_support.rs</FILE> - <DESC>Effect descriptor scope support DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: declare supported scope kinds and spaces.</WCTX>
// <CLOG>0.1.0: INIT — add scope capability declaration and support checks.</CLOG>

use crate::{CoordinateSpace, RoleSpace, ScopeKind, ScopeSpec};

/// Scope shapes and evaluation spaces an effect descriptor supports.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScopeSupport {
    /// Supported scope variant kinds.
    pub kinds: Vec<ScopeKind>,
    /// Supported coordinate spaces for geometry scopes.
    pub coordinate_spaces: Vec<CoordinateSpace>,
    /// Supported role spaces for role scopes.
    pub role_spaces: Vec<RoleSpace>,
}

impl ScopeSupport {
    /// Return true when this support declaration accepts the scope kind.
    pub fn supports_scope(&self, scope: &ScopeSpec) -> bool {
        self.kinds.contains(&ScopeKind::from(scope))
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_support.rs</FILE> - <DESC>Effect descriptor scope support DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
