// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_grid_scope_cells.rs</FILE> - <DESC>Collect styled-grid cells for scopes</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 scope decision: evaluate accepted built-in styled-grid scopes.</WCTX>
// <CLOG>0.2.0: MINOR — support modulo, non-empty, outer-band, and inner styled-grid scopes.</CLOG>

use tui_vfx_contract::ScopeSpec;

use crate::PlayerStyledGrid;

/// Collect styled-grid cells matching an optional scope.
pub(crate) fn collect_styled_grid_scope_cells(
    scope: Option<&ScopeSpec>,
    styled_grid: &PlayerStyledGrid,
) -> Vec<(usize, usize)> {
    all_cells(styled_grid)
        .into_iter()
        .filter(|(x, y)| scope_matches(scope, styled_grid, *x, *y))
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
    x: usize,
    y: usize,
) -> bool {
    match scope {
        None | Some(ScopeSpec::All) => true,
        Some(ScopeSpec::Role { role }) => {
            role.shorthand_name() == "border" && is_border(styled_grid, x, y)
        }
        Some(ScopeSpec::Rect { rect }) => rect.contains(x as u16, y as u16),
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

fn is_border(styled_grid: &PlayerStyledGrid, x: usize, y: usize) -> bool {
    styled_grid.contains(x, y)
        && (x == 0 || y == 0 || x + 1 == styled_grid.width() || y + 1 == styled_grid.height())
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_styled_grid_scope_cells.rs</FILE> - <DESC>Collect styled-grid cells for scopes</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
