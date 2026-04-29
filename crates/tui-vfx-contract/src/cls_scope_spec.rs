// <FILE>crates/tui-vfx-contract/src/cls_scope_spec.rs</FILE> - <DESC>Minimal surface scope algebra</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>K2.13 schema decision burn-down: add accepted built-in style scope vocabulary.</WCTX>
// <CLOG>0.5.0: MINOR — add modulo, non-empty, outer-band, and inner scopes.
// 0.4.2: PATCH — add explicit Schemars descriptions for row/column range fields.</CLOG>

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
    /// Match rows where `row % modulus == remainder`.
    ModuloRows {
        /// Positive modulus used to select rows.
        modulus: usize,
        /// Remainder selected by the modulo expression.
        remainder: usize,
    },
    /// Match columns where `column % modulus == remainder`.
    ModuloColumns {
        /// Positive modulus used to select columns.
        modulus: usize,
        /// Remainder selected by the modulo expression.
        remainder: usize,
    },
    /// Match cells containing non-empty glyph content.
    NonEmpty,
    /// Match the outer perimeter band.
    OuterBand,
    /// Match cells that are not on the outer perimeter.
    Inner,
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
            ScopeSpec::ModuloRows { modulus, remainder } => {
                let (_, y) = scope_coordinate(input, coordinate_space);
                *modulus > 0 && y % *modulus == *remainder
            }
            ScopeSpec::ModuloColumns { modulus, remainder } => {
                let (x, _) = scope_coordinate(input, coordinate_space);
                *modulus > 0 && x % *modulus == *remainder
            }
            ScopeSpec::NonEmpty => {
                scope_glyph(input, coordinate_space).is_some_and(|glyph| !glyph.trim().is_empty())
            }
            ScopeSpec::OuterBand => {
                let (x, y) = scope_coordinate(input, coordinate_space);
                scope_dimensions(input, coordinate_space).is_some_and(|(width, height)| {
                    width > 0
                        && height > 0
                        && (x == 0 || y == 0 || x + 1 == width || y + 1 == height)
                })
            }
            ScopeSpec::Inner => {
                let (x, y) = scope_coordinate(input, coordinate_space);
                scope_dimensions(input, coordinate_space).is_some_and(|(width, height)| {
                    width > 2 && height > 2 && x > 0 && y > 0 && x + 1 < width && y + 1 < height
                })
            }
        }
    }
}

fn scope_glyph(input: &ScopeEvalInput, coordinate_space: CoordinateSpace) -> Option<&str> {
    match coordinate_space {
        CoordinateSpace::DestinationLocal => input.destination_glyph.as_deref(),
        CoordinateSpace::SampledSource => input.sampled_source_glyph.as_deref(),
    }
}

fn scope_dimensions(
    input: &ScopeEvalInput,
    coordinate_space: CoordinateSpace,
) -> Option<(usize, usize)> {
    match coordinate_space {
        CoordinateSpace::DestinationLocal => {
            Some((input.destination_width?, input.destination_height?))
        }
        CoordinateSpace::SampledSource => {
            Some((input.sampled_source_width?, input.sampled_source_height?))
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_spec.rs</FILE> - <DESC>Minimal surface scope algebra</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
