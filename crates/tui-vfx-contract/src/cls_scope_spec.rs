// <FILE>crates/tui-vfx-contract/src/cls_scope_spec.rs</FILE> - <DESC>Minimal surface scope algebra</DESC>
// <VERS>VERSION: 0.4.2</VERS>
// <WCTX>New kernel Phase D0 verifier fix: make scope enum wire shape strict and fully described.</WCTX>
// <CLOG>0.4.2: PATCH — add explicit Schemars descriptions for row/column range fields.
// 0.4.1: PATCH — switch schema-visible variants to named fields so strict schemas include field descriptions.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract ScopeSpec enum and keep matching semantics.</CLOG>

use tui_vfx_types::{Rect, RoleTag};

use crate::{CoordinateSpace, RoleSpace, ScopeEvalInput, fnc_scope_coordinate::scope_coordinate};

/// Minimal Phase A/B scope algebra.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ScopeSpec {
    /// Match every in-bounds cell.
    All,
    /// Match cells with the requested role in the active role space.
    Role {
        /// Role required in the active role space.
        role: RoleTag,
    },
    /// Match a rectangle in the active coordinate space.
    Rect {
        /// Rectangle to match in the active coordinate space.
        rect: Rect,
    },
    /// Match rows `[start, end)` in the active coordinate space.
    RowRange {
        /// Inclusive start row in the active coordinate space.
        #[schemars(description = "Inclusive start row in the active coordinate space.")]
        start: usize,
        /// Exclusive end row in the active coordinate space.
        #[schemars(description = "Exclusive end row in the active coordinate space.")]
        end: usize,
    },
    /// Match columns `[start, end)` in the active coordinate space.
    ColumnRange {
        /// Inclusive start column in the active coordinate space.
        #[schemars(description = "Inclusive start column in the active coordinate space.")]
        start: usize,
        /// Exclusive end column in the active coordinate space.
        #[schemars(description = "Exclusive end column in the active coordinate space.")]
        end: usize,
    },
}

impl ScopeSpec {
    /// Evaluate this scope using explicit coordinate and role spaces.
    pub fn matches(
        &self,
        input: &ScopeEvalInput,
        coordinate_space: CoordinateSpace,
        role_space: RoleSpace,
    ) -> bool {
        match self {
            ScopeSpec::All => true,
            ScopeSpec::Role { role } => match role_space {
                RoleSpace::SampledSource => &input.sampled_source_role == role,
                RoleSpace::Destination => &input.destination_role == role,
            },
            ScopeSpec::Rect { rect } => {
                let (x, y) = scope_coordinate(input, coordinate_space);
                let Ok(x) = u16::try_from(x) else {
                    return false;
                };
                let Ok(y) = u16::try_from(y) else {
                    return false;
                };
                rect.contains(x, y)
            }
            ScopeSpec::RowRange { start, end } => {
                let (_, y) = scope_coordinate(input, coordinate_space);
                y >= *start && y < *end
            }
            ScopeSpec::ColumnRange { start, end } => {
                let (x, _) = scope_coordinate(input, coordinate_space);
                x >= *start && x < *end
            }
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_spec.rs</FILE> - <DESC>Minimal surface scope algebra</DESC>
// <VERS>END OF VERSION: 0.4.2</VERS>
