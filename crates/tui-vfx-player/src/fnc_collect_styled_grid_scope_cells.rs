// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_grid_scope_cells.rs</FILE> - <DESC>Collect styled-grid cells for scopes</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Player scope evaluation: evaluate accepted built-in styled-grid scopes.</WCTX>
// <CLOG>0.3.0: MINOR — resolve dynamic single-cell scopes from sample request values.
// 0.2.0: MINOR — support modulo, non-empty, outer-band, and inner styled-grid scopes.

use tui_vfx_contract::{ScopeSpec, Value, ValueSource};

use crate::{
    PlayerSampleRequest, PlayerStyledGrid,
    fnc_resolve_value_source::resolve_value_source_with_graph_values,
};

/// Collect styled-grid cells matching an optional scope.
pub(crate) fn collect_styled_grid_scope_cells(
    scope: Option<&ScopeSpec>,
    styled_grid: &PlayerStyledGrid,
    request: &PlayerSampleRequest,
) -> Vec<(usize, usize)> {
    all_cells(styled_grid)
        .into_iter()
        .filter(|(x, y)| scope_matches(scope, styled_grid, request, *x, *y))
        .collect()
}

/// Collect styled-grid border cells.
pub(crate) fn collect_styled_grid_border_cells(
    styled_grid: &PlayerStyledGrid,
) -> Vec<(usize, usize)> {
    all_cells(styled_grid)
        .into_iter()
        .filter(|(x, y)| is_border(styled_grid, *x, *y))
        .collect()
}

fn all_cells(styled_grid: &PlayerStyledGrid) -> Vec<(usize, usize)> {
    (0..styled_grid.height())
        .flat_map(|y| (0..styled_grid.width()).map(move |x| (x, y)))
        .collect()
}

fn scope_matches(
    scope: Option<&ScopeSpec>,
    styled_grid: &PlayerStyledGrid,
    request: &PlayerSampleRequest,
    x: usize,
    y: usize,
) -> bool {
    match scope {
        None | Some(ScopeSpec::All) => true,
        Some(ScopeSpec::Role { role }) => {
            role.shorthand_name() == "border" && is_border(styled_grid, x, y)
        }
        Some(ScopeSpec::Rect { rect }) => rect.contains(x as u16, y as u16),
        Some(ScopeSpec::Cell {
            x: source_x,
            y: source_y,
        }) => {
            resolve_scope_coordinate(source_x, request) == Some(x)
                && resolve_scope_coordinate(source_y, request) == Some(y)
        }
        Some(ScopeSpec::RowRange { start, end }) => y >= *start && y < *end,
        Some(ScopeSpec::ColumnRange { start, end }) => x >= *start && x < *end,
        Some(ScopeSpec::ModuloRows { modulus, remainder }) => {
            *modulus > 0 && y % *modulus == *remainder
        }
        Some(ScopeSpec::ModuloColumns { modulus, remainder }) => {
            *modulus > 0 && x % *modulus == *remainder
        }
        Some(ScopeSpec::NonEmpty) => styled_grid
            .cells()
            .iter()
            .any(|cell| cell.x == x && cell.y == y && !cell.glyph.trim().is_empty()),
        Some(ScopeSpec::OuterBand) => is_border(styled_grid, x, y),
        Some(ScopeSpec::Inner) => styled_grid.contains(x, y) && !is_border(styled_grid, x, y),
    }
}

fn resolve_scope_coordinate(source: &ValueSource, request: &PlayerSampleRequest) -> Option<usize> {
    let value =
        resolve_value_source_with_graph_values(source, &request.signals, &request.graph_values)?;
    match value {
        Value::Integer(value) => usize::try_from(value).ok(),
        Value::Number(value) if value.is_finite() && value >= 0.0 => Some(value.round() as usize),
        _ => None,
    }
}

fn is_border(styled_grid: &PlayerStyledGrid, x: usize, y: usize) -> bool {
    styled_grid.contains(x, y)
        && (x == 0 || y == 0 || x + 1 == styled_grid.width() || y + 1 == styled_grid.height())
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_grid_scope_cells.rs</FILE> - <DESC>Collect styled-grid cells for scopes</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
