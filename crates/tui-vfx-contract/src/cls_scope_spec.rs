// <FILE>crates/tui-vfx-contract/src/cls_scope_spec.rs</FILE> - <DESC>Minimal surface scope algebra</DESC>
// <VERS>VERSION: 0.8.1</VERS>
// <WCTX>v3.1 scope contract: add accepted built-in style scope vocabulary and explicit index-set scopes.</WCTX>
// <CLOG>0.8.1: PATCH — constrain row and column index-set schemas to non-empty unique arrays.
// 0.8.0: MINOR — add non-contiguous row and column index-set scopes.
// 0.7.0: MINOR — treat new time/expression value sources as non-static scope coordinates.
// 0.6.0: MINOR — add value-source-backed single-cell scopes for runtime-selected cells.
// 0.5.0: MINOR — add modulo, non-empty, outer-band, and inner scopes.
// 0.4.2: PATCH — add explicit Schemars descriptions for row/column range fields.</CLOG>

use tui_vfx_types::{Rect, RoleTag};

use crate::{
    CoordinateSpace, NumericRange, RoleSpace, ScopeEvalInput, Value, ValueSource,
    fnc_scope_coordinate::scope_coordinate,
};

/// Minimal scope algebra for active surface coordinate evaluation.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
    /// Match one cell using value sources for each coordinate.
    ///
    /// This preserves runtime-bound cell targeting from the source pathway while
    /// still allowing static literal coordinates for deterministic validation.
    Cell {
        /// X coordinate source in the active coordinate space.
        x: Box<ValueSource>,
        /// Y coordinate source in the active coordinate space.
        y: Box<ValueSource>,
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
    /// Match specific non-contiguous row indices in the active coordinate space.
    Rows {
        /// Row indices selected in the active coordinate space.
        #[schemars(length(min = 1), extend("uniqueItems" = true))]
        indices: Vec<usize>,
    },
    /// Match specific non-contiguous column indices in the active coordinate space.
    Columns {
        /// Column indices selected in the active coordinate space.
        #[schemars(length(min = 1), extend("uniqueItems" = true))]
        indices: Vec<usize>,
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
            ScopeSpec::Cell { x, y } => {
                let Some(target_x) = static_coordinate_source(x) else {
                    return false;
                };
                let Some(target_y) = static_coordinate_source(y) else {
                    return false;
                };
                let (cell_x, cell_y) = scope_coordinate(input, coordinate_space);
                cell_x == target_x && cell_y == target_y
            }
            ScopeSpec::RowRange { start, end } => {
                let (_, y) = scope_coordinate(input, coordinate_space);
                y >= *start && y < *end
            }
            ScopeSpec::ColumnRange { start, end } => {
                let (x, _) = scope_coordinate(input, coordinate_space);
                x >= *start && x < *end
            }
            ScopeSpec::Rows { indices } => {
                let (_, y) = scope_coordinate(input, coordinate_space);
                indices.contains(&y)
            }
            ScopeSpec::Columns { indices } => {
                let (x, _) = scope_coordinate(input, coordinate_space);
                indices.contains(&x)
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

fn static_coordinate_source(source: &ValueSource) -> Option<usize> {
    resolve_static_value_source(source).and_then(|value| value_to_usize(&value))
}

fn resolve_static_value_source(source: &ValueSource) -> Option<Value> {
    match source {
        ValueSource::Literal { value } => Some(value.clone()),
        ValueSource::Signal { fallback, .. }
        | ValueSource::Parameter { fallback, .. }
        | ValueSource::GraphValue { fallback, .. }
        | ValueSource::SampledField { fallback, .. }
        | ValueSource::SignalExpression { fallback, .. } => fallback.clone(),
        ValueSource::Map {
            from,
            input,
            output,
            clamp,
        } => map_static_numeric(resolve_static_value_source(from)?, *input, *output, *clamp),
        ValueSource::PhaseProgress { .. } | ValueSource::Clock { .. } => None,
    }
}

fn map_static_numeric(
    value: Value,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
) -> Option<Value> {
    let number = match value {
        Value::Integer(value) => value as f64,
        Value::Number(value) | Value::Duration(value) => value,
        _ => return None,
    };
    let min_in = input.min?;
    let max_in = input.max?;
    let min_out = output.min?;
    let max_out = output.max?;
    let mut normalized = (number - min_in) / (max_in - min_in);
    if clamp {
        normalized = normalized.clamp(0.0, 1.0);
    }
    Some(Value::Number(min_out + normalized * (max_out - min_out)))
}

fn value_to_usize(value: &Value) -> Option<usize> {
    match value {
        Value::Integer(value) => usize::try_from(*value).ok(),
        Value::Number(value) if value.is_finite() && *value >= 0.0 => Some(value.round() as usize),
        _ => None,
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
// <VERS>END OF VERSION: 0.8.1</VERS>
