// <FILE>crates/tui-vfx-next/src/cls_dim_effect.rs</FILE> - <DESC>Visual-only dim effect for surface contract tests</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase E1: keep proof effect independent of durable descriptor DTO.</WCTX>
// <CLOG>0.5.0: PATCH — remove old proof descriptor constant after durable descriptors move to tui-vfx-contract.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract DimEffect into one cohesive class file.</CLOG>

use tui_vfx_types::Cell;

use crate::{CellWrite, CellWritePolicy, RoleWritePolicy};

/// Visual-only dim effect. It changes color channels and preserves roles.
#[derive(
    Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DimEffect {
    /// Brightness factor applied to foreground and background colors.
    pub factor: f32,
}

impl DimEffect {
    /// Create a dim effect. Factor is clamped by color operations during apply.
    pub const fn new(factor: f32) -> Self {
        Self { factor }
    }

    /// Build a write for one sampled cell, preserving the destination role.
    pub fn write(&self, source: &Cell) -> CellWrite {
        let mut cell = *source;
        cell.fg = cell.fg.dim(self.factor);
        cell.bg = cell.bg.dim(self.factor);
        CellWrite {
            cell,
            cell_policy: CellWritePolicy::WriteCell,
            role_policy: RoleWritePolicy::PreserveDestination,
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_dim_effect.rs</FILE> - <DESC>Visual-only dim effect for surface contract tests</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
