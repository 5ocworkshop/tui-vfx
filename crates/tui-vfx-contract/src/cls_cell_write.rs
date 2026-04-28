// <FILE>crates/tui-vfx-contract/src/cls_cell_write.rs</FILE> - <DESC>Concrete cell write request DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract CellWrite DTO and helpers.</CLOG>

use tui_vfx_types::Cell;

use crate::{CellWritePolicy, RoleWritePolicy};

/// One concrete cell write request.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CellWrite {
    /// Cell channel payload to write.
    pub cell: Cell,
    /// Cell channel policy.
    pub cell_policy: CellWritePolicy,
    /// Role channel policy.
    pub role_policy: RoleWritePolicy,
}

impl CellWrite {
    /// Build a copy write that preserves sampled-source roles.
    pub fn copy_sampled_source(cell: Cell) -> Self {
        Self {
            cell,
            cell_policy: CellWritePolicy::WriteCell,
            role_policy: RoleWritePolicy::CopySampledSource,
        }
    }

    /// Returns true when the write should be skipped by cell policy.
    pub fn is_skipped(&self) -> bool {
        self.cell_policy == CellWritePolicy::SkipTransparentEmpty && self.cell.is_empty()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_cell_write.rs</FILE> - <DESC>Concrete cell write request DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
