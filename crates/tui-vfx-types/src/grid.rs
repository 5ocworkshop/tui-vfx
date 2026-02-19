// <FILE>crates/tui-vfx-types/src/grid.rs</FILE> - <DESC>Grid trait and implementations</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Audit fixes - address OAI peer review findings</WCTX>
// <CLOG>Fix remap_coord panic on empty grids (zero-size guard)</CLOG>

//! Grid trait and implementations for cell-based operations.

use crate::Cell;

/// How to handle coordinates outside grid bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoundaryMode {
    /// Clamp coordinates to valid range.
    #[default]
    Clamp,
    /// Wrap coordinates around (modulo).
    Wrap,
    /// Skip out-of-bounds operations (no-op).
    Skip,
}

/// A 2D grid of cells that effects can operate on.
///
/// This is the primary abstraction for spatial canvas operations.
pub trait Grid {
    /// Get the grid width.
    fn width(&self) -> usize;

    /// Get the grid height.
    fn height(&self) -> usize;

    /// Get a cell at the given coordinates.
    fn get(&self, x: usize, y: usize) -> Option<&Cell>;

    /// Get a mutable reference to a cell.
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell>;

    /// Set a cell at the given coordinates.
    fn set(&mut self, x: usize, y: usize, cell: Cell);

    /// Check if coordinates are within bounds.
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    /// Get total cell count.
    fn len(&self) -> usize {
        self.width() * self.height()
    }

    /// Check if grid is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Extension trait with higher-level grid operations.
pub trait GridExt: Grid {
    /// Fill the entire grid with a cell.
    fn fill(&mut self, cell: Cell) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set(x, y, cell);
            }
        }
    }

    /// Fill a rectangular region.
    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, cell: Cell) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if self.in_bounds(px, py) {
                    self.set(px, py, cell);
                }
            }
        }
    }

    /// Apply a function to each cell.
    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut Cell),
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(cell) = self.get_mut(x, y) {
                    f(x, y, cell);
                }
            }
        }
    }

    /// Remap coordinates with boundary handling.
    ///
    /// Returns `None` for empty grids (zero width or height).
    fn remap_coord(&self, x: i32, y: i32, mode: BoundaryMode) -> Option<(usize, usize)> {
        let w = self.width() as i32;
        let h = self.height() as i32;

        // Guard against empty grids to prevent panic in Clamp (w-1 underflow)
        // and Wrap (rem_euclid(0) is undefined)
        if w == 0 || h == 0 {
            return None;
        }

        let (rx, ry) = match mode {
            BoundaryMode::Clamp => (x.clamp(0, w - 1), y.clamp(0, h - 1)),
            BoundaryMode::Wrap => (x.rem_euclid(w), y.rem_euclid(h)),
            BoundaryMode::Skip => {
                if x < 0 || x >= w || y < 0 || y >= h {
                    return None;
                }
                (x, y)
            }
        };

        Some((rx as usize, ry as usize))
    }
}

// Blanket implementation for all Grid types
impl<T: Grid> GridExt for T {}

/// Simple owned grid implementation.
#[derive(Clone, Debug)]
pub struct OwnedGrid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl OwnedGrid {
    /// Create a new grid filled with default cells.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
        }
    }

    /// Create a grid from existing cells.
    pub fn from_cells(width: usize, height: usize, cells: Vec<Cell>) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "Cell count must match dimensions"
        );
        Self {
            width,
            height,
            cells,
        }
    }

    /// Get the underlying cells as a slice.
    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Get the underlying cells as a mutable slice.
    pub fn cells_mut(&mut self) -> &mut [Cell] {
        &mut self.cells
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

impl Grid for OwnedGrid {
    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.index(x, y).map(|i| &self.cells[i])
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.index(x, y).map(|i| &mut self.cells[i])
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = cell;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn test_owned_grid_new() {
        let grid = OwnedGrid::new(10, 5);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 5);
        assert_eq!(grid.len(), 50);
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = OwnedGrid::new(3, 3);
        let cell = Cell::new('X').with_fg(Color::RED);
        grid.set(1, 1, cell);

        let got = grid.get(1, 1).unwrap();
        assert_eq!(got.ch, 'X');
        assert_eq!(got.fg, Color::RED);
    }

    #[test]
    fn test_grid_bounds() {
        let grid = OwnedGrid::new(5, 5);
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(4, 4));
        assert!(!grid.in_bounds(5, 5));
        assert!(grid.get(10, 10).is_none());
    }

    #[test]
    fn test_grid_fill() {
        let mut grid = OwnedGrid::new(3, 3);
        let cell = Cell::new('*');
        grid.fill(cell);

        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(grid.get(x, y).unwrap().ch, '*');
            }
        }
    }

    #[test]
    fn test_remap_coord_clamp() {
        let grid = OwnedGrid::new(10, 10);
        assert_eq!(grid.remap_coord(-5, -5, BoundaryMode::Clamp), Some((0, 0)));
        assert_eq!(
            grid.remap_coord(100, 100, BoundaryMode::Clamp),
            Some((9, 9))
        );
    }

    #[test]
    fn test_remap_coord_wrap() {
        let grid = OwnedGrid::new(10, 10);
        assert_eq!(grid.remap_coord(-1, -1, BoundaryMode::Wrap), Some((9, 9)));
        assert_eq!(grid.remap_coord(12, 15, BoundaryMode::Wrap), Some((2, 5)));
    }

    #[test]
    fn test_remap_coord_skip() {
        let grid = OwnedGrid::new(10, 10);
        assert_eq!(grid.remap_coord(5, 5, BoundaryMode::Skip), Some((5, 5)));
        assert_eq!(grid.remap_coord(-1, 5, BoundaryMode::Skip), None);
        assert_eq!(grid.remap_coord(5, 100, BoundaryMode::Skip), None);
    }

    #[test]
    fn test_remap_coord_empty_grid() {
        // Zero width
        let grid = OwnedGrid::new(0, 10);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Clamp), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Wrap), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Skip), None);

        // Zero height
        let grid = OwnedGrid::new(10, 0);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Clamp), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Wrap), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Skip), None);

        // Both zero
        let grid = OwnedGrid::new(0, 0);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Clamp), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Wrap), None);
        assert_eq!(grid.remap_coord(0, 0, BoundaryMode::Skip), None);
    }
}

// <FILE>crates/tui-vfx-types/src/grid.rs</FILE> - <DESC>Grid trait and implementations</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
