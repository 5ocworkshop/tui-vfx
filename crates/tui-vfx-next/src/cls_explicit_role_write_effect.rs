// <FILE>crates/tui-vfx-next/src/cls_explicit_role_write_effect.rs</FILE> - <DESC>Procedural explicit-role write effect for contract tests</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase E1: keep proof effect independent of durable descriptor DTO.</WCTX>
// <CLOG>0.5.0: PATCH — remove old proof descriptor constant after durable descriptors move to tui-vfx-contract.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract ExplicitRoleWriteEffect into one cohesive class file.</CLOG>

use tui_vfx_types::{Cell, RoleTag};

use crate::{CellWrite, CellWritePolicy, RoleWritePolicy};

/// Tiny procedural writer that can explicitly set a destination role.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExplicitRoleWriteEffect {
    /// Cell to write.
    pub cell: Cell,
    /// Role to assign to the destination cell.
    pub role: RoleTag,
    /// Cell write policy.
    pub cell_policy: CellWritePolicy,
}

impl ExplicitRoleWriteEffect {
    /// Create a writer that writes a cell and sets an explicit role.
    pub fn new(cell: Cell, role: RoleTag) -> Self {
        Self {
            cell,
            role,
            cell_policy: CellWritePolicy::WriteCell,
        }
    }

    /// Build a write for one destination cell.
    pub fn write(&self) -> CellWrite {
        CellWrite {
            cell: self.cell,
            cell_policy: self.cell_policy,
            role_policy: RoleWritePolicy::SetExplicit {
                role: self.role.clone(),
            },
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_explicit_role_write_effect.rs</FILE> - <DESC>Procedural explicit-role write effect for contract tests</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
