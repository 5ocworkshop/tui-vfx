// <FILE>crates/tui-vfx-shadow/src/fnc_extract_shadow_envelope.rs</FILE> - <DESC>Pure function that extracts the cell-mask envelope for shadow extrusion given an optional RoleTag filter</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.3.4 — introduce CellMask + extract_shadow_envelope pure function that returns the set of source cells the shadow stage should extrude from. None = every non-empty source cell; Some(role) = only cells whose role matches. Tiny-rect (1x1, 2x1) and zero-dim sources must never panic.</WCTX>
// <CLOG>0.1.0: initial CellMask + extract_shadow_envelope. CellMask stores row-major Vec<bool>; provides width(), height(), get((x,y)), and bounding_rect() for downstream shadow-rect derivation. The extract function takes the minimum of grid and role-map dimensions (defensive for dimension-mismatch cases). No panic for zero or tiny rectangles.</CLOG>

//! Pure function and supporting `CellMask` type that compute the shadow
//! extrusion envelope.
//!
//! The shadow extrusion stage uses this to decide WHERE to emit shadow
//! cells. Two modes:
//!
//! 1. **`source_region = None`** (default): every non-empty source cell
//!    contributes to the envelope. Equivalent to today's rect-based
//!    behaviour collapsed to a per-cell mask.
//! 2. **`source_region = Some(role)`**: only cells whose role matches
//!    `role` contribute. The shadow stage typically consumes
//!    [`CellMask::bounding_rect`] and passes it to
//!    [`crate::render_shadow`] as the effective element rectangle.
//!
//! # Tiny / empty rectangles
//!
//! The function handles zero-dimensional sources (0×0, 0×N, N×0) and
//! tiny rectangles (1×1, 2×1) without panicking. A mask with no set
//! cells has `bounding_rect() == None`.
//!
//! # Dimension mismatches
//!
//! If the grid and role map disagree on size, the function uses the
//! smaller dimension on each axis — safer than panicking and sufficient
//! for the compositor pathway where the two may drift briefly during
//! migration phases.

use tui_vfx_types::{Grid, Rect, RoleMap, RoleTag};

/// Dense per-cell boolean mask used by the shadow extrusion stage.
///
/// Cells are stored row-major (`y * width + x`). Out-of-bounds reads
/// return `false`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellMask {
    width: u16,
    height: u16,
    cells: Vec<bool>,
}

impl CellMask {
    /// Construct a new mask with the given dimensions; every cell is `false`.
    pub fn empty(width: u16, height: u16) -> Self {
        let len = width as usize * height as usize;
        Self {
            width,
            height,
            cells: vec![false; len],
        }
    }

    /// Return the mask width in cells.
    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Return the mask height in cells.
    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Read the mask bit at `(x, y)`. Out of bounds returns `false`.
    #[inline]
    pub fn get(&self, pos: (u16, u16)) -> bool {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return false;
        }
        self.cells[y as usize * self.width as usize + x as usize]
    }

    /// Set the mask bit at `(x, y)`. Out-of-bounds positions silently
    /// no-op.
    #[inline]
    pub fn set(&mut self, pos: (u16, u16), value: bool) {
        let (x, y) = pos;
        if x < self.width && y < self.height {
            self.cells[y as usize * self.width as usize + x as usize] = value;
        }
    }

    /// Return the tight bounding rectangle of set cells, or `None` if
    /// the mask is empty (no cells set).
    ///
    /// Used by the shadow stage to derive the effective element
    /// rectangle for role-filtered extrusion.
    pub fn bounding_rect(&self) -> Option<Rect> {
        let mut min_x = u16::MAX;
        let mut min_y = u16::MAX;
        let mut max_x = 0u16;
        let mut max_y = 0u16;
        let mut any = false;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get((x, y)) {
                    any = true;
                    if x < min_x {
                        min_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
        if !any {
            return None;
        }
        Some(Rect::new(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
    }

    /// Count the number of cells set to `true`.
    pub fn count(&self) -> usize {
        self.cells.iter().filter(|&&b| b).count()
    }
}

/// Compute the shadow extrusion envelope for a source grid and its role
/// map, optionally restricting to cells whose role matches `source_region`.
///
/// Returns a [`CellMask`] whose set bits are the source cells that
/// should contribute to shadow extrusion.
///
/// # Parameters
///
/// - `source_grid` — the source surface. Non-empty cells (glyph != ' '
///   or non-transparent colour) are eligible for the envelope.
/// - `source_roles` — per-cell role tags. Queried only when
///   `source_region` is `Some`.
/// - `source_region` — `None` to include every non-empty source cell;
///   `Some(role)` to restrict to cells whose role tag equals `role`.
///
/// # Dimensions
///
/// The returned mask's dimensions follow the role map's dimensions (in
/// `u16`). When the grid and role map disagree, the smaller axis wins
/// for reads and the mask is sized to the role map.
///
/// # Example
///
/// ```
/// use tui_vfx_shadow::{CellMask, extract_shadow_envelope};
/// use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleMap, RoleTag};
///
/// // A 4x3 grid fully filled with 'X'.
/// let mut grid = OwnedGrid::new(4, 3);
/// for y in 0..3 {
///     for x in 0..4 {
///         grid.set(x, y, Cell::new('X'));
///     }
/// }
/// let mut roles = RoleMap::new_with_default(4, 3, RoleTag::Text);
/// roles.set((0, 0), RoleTag::Border);
///
/// // No filter: every filled cell is in the envelope.
/// let full = extract_shadow_envelope(&grid, &roles, None);
/// assert_eq!(full.count(), 4 * 3);
///
/// // Border filter: only the single (0, 0) cell.
/// let border = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
/// assert_eq!(border.count(), 1);
/// assert!(border.get((0, 0)));
/// ```
pub fn extract_shadow_envelope<G: Grid + ?Sized>(
    source_grid: &G,
    source_roles: &RoleMap,
    source_region: Option<RoleTag>,
) -> CellMask {
    let width = source_roles.width();
    let height = source_roles.height();
    let mut mask = CellMask::empty(width, height);
    if width == 0 || height == 0 {
        return mask;
    }

    for y in 0..height {
        for x in 0..width {
            // Consult the source grid for non-emptiness. Outside the
            // grid's dimensions we treat the cell as empty.
            let cell_nonempty = match source_grid.get(x as usize, y as usize) {
                Some(cell) => {
                    cell.ch != ' ' || cell.bg.a != 0 || cell.fg.a != 0
                }
                None => false,
            };
            if !cell_nonempty {
                continue;
            }
            let include = match &source_region {
                None => true,
                Some(role) => source_roles.get((x, y)).as_ref() == Some(role),
            };
            if include {
                mask.set((x, y), true);
            }
        }
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_mask_has_no_set_cells() {
        let mask = CellMask::empty(4, 3);
        assert_eq!(mask.count(), 0);
        assert_eq!(mask.bounding_rect(), None);
    }

    #[test]
    fn set_and_get_round_trip() {
        let mut mask = CellMask::empty(4, 3);
        mask.set((1, 2), true);
        assert!(mask.get((1, 2)));
        assert!(!mask.get((0, 0)));
        assert_eq!(mask.bounding_rect(), Some(Rect::new(1, 2, 1, 1)));
    }

    #[test]
    fn set_out_of_bounds_noop() {
        let mut mask = CellMask::empty(2, 2);
        mask.set((10, 10), true);
        assert_eq!(mask.count(), 0);
    }
}

// <FILE>crates/tui-vfx-shadow/src/fnc_extract_shadow_envelope.rs</FILE> - <DESC>Pure extract_shadow_envelope function + CellMask</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
