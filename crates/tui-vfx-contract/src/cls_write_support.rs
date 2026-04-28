// <FILE>crates/tui-vfx-contract/src/cls_write_support.rs</FILE> - <DESC>Effect descriptor write policy support DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: declare supported cell and role write policies.</WCTX>
// <CLOG>0.1.0: INIT — add write support declaration and support checks.</CLOG>

use crate::{CellWritePolicy, RoleWritePolicy, RoleWritePolicyKind};

/// Cell and role write policies an effect descriptor supports.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WriteSupport {
    /// Supported cell write policies.
    pub cell_policies: Vec<CellWritePolicy>,
    /// Supported role write policy kinds.
    pub role_policies: Vec<RoleWritePolicyKind>,
}

impl WriteSupport {
    /// Return true when this descriptor supports the cell write policy.
    pub fn supports_cell_policy(&self, policy: CellWritePolicy) -> bool {
        self.cell_policies.contains(&policy)
    }

    /// Return true when this descriptor supports the role write policy kind.
    pub fn supports_role_policy(&self, policy: &RoleWritePolicy) -> bool {
        self.role_policies
            .contains(&RoleWritePolicyKind::from(policy))
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_write_support.rs</FILE> - <DESC>Effect descriptor write policy support DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
