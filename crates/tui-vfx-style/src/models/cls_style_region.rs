// <FILE>tui-vfx-style/src/models/cls_style_region.rs</FILE> - <DESC>Style region targeting enum</DESC>
// <VERS>VERSION: 4.1.0</VERS>
// <WCTX>Region-relative shader coordinates for RevealWipe</WCTX>
// <CLOG>Add bounding_rect() and to_local_coords() for region-relative shader context</CLOG>

use serde::{Deserialize, Serialize};

/// A cell coordinate for per-cell targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CellCoord {
    /// X coordinate (column, 0-based from left)
    pub x: u16,
    /// Y coordinate (row, 0-based from top)
    pub y: u16,
}

impl CellCoord {
    /// Create a new cell coordinate.
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/// Axis for modulo-based region targeting.
///
/// Used with `StyleRegion::Modulo` to specify whether the pattern
/// applies to rows (Horizontal) or columns (Vertical).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "PascalCase")]
pub enum ModuloAxis {
    /// Apply modulo pattern to rows (y coordinate)
    /// e.g., for CRT scanlines that span horizontally
    Horizontal,
    /// Apply modulo pattern to columns (x coordinate)
    /// e.g., for vertical stripe effects
    Vertical,
}

/// Specifies which region of a widget should receive style effects.
///
/// Region targeting enables effects like:
/// - `BorderOnly`: Border sweep animations on just the border
/// - `TextOnly`: Highlighter effects on just the text content
/// - `All`: Apply effects to the entire widget (default)
/// - `Rows`: Target specific rows for progress indicators or scanners
/// - `RowRange`: Target a contiguous range of rows
/// - `Cell`: Target a single cell for per-cell effects
/// - `Cells`: Target multiple specific cells
/// - `Column`: Target a single column
/// - `Columns`: Target multiple specific columns
/// - `ColumnRange`: Target a contiguous range of columns
/// - `Modulo`: Target rows/columns matching a modulo pattern (e.g., every other row for scanlines)
#[derive(
    Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum StyleRegion {
    /// Apply style to the entire widget (default behavior)
    #[default]
    All,
    /// Apply style only to text content, not borders or background
    TextOnly,
    /// Apply style only to border cells
    BorderOnly,
    /// Apply style only to background (non-text, non-border)
    BackgroundOnly,
    /// Apply style only to specific rows (0-based from widget top)
    Rows(Vec<u16>),
    /// Apply style to a contiguous range of rows [start, end)
    RowRange {
        /// First row to include (0-based)
        start: u16,
        /// First row to exclude (exclusive end)
        end: u16,
    },
    /// Apply style to a single cell at (x, y)
    Cell {
        /// X coordinate (column, 0-based from left)
        x: u16,
        /// Y coordinate (row, 0-based from top)
        y: u16,
    },
    /// Apply style to multiple specific cells
    Cells(Vec<CellCoord>),
    /// Apply style only to a specific column (0-based from left)
    Column(u16),
    /// Apply style only to specific columns (0-based from left)
    Columns(Vec<u16>),
    /// Apply style to a contiguous range of columns [start, end)
    ColumnRange {
        /// First column to include (0-based)
        start: u16,
        /// First column to exclude (exclusive end)
        end: u16,
    },
    /// Apply style to rows/columns matching a modulo pattern.
    ///
    /// Useful for CRT scanline effects, alternating stripes, or periodic patterns.
    /// Example: `Modulo { axis: Horizontal, modulus: 2, remainder: 0 }` targets
    /// every other row (rows 0, 2, 4, ...).
    Modulo {
        /// Which axis to apply the modulo pattern to
        axis: ModuloAxis,
        /// The divisor for the modulo operation (e.g., 2 for every other)
        modulus: u16,
        /// The remainder to match (e.g., 0 for 0,2,4... or 1 for 1,3,5...)
        remainder: u16,
    },
}

impl StyleRegion {
    /// Check if a cell at (x, y) within a rect of (width, height) should receive styling.
    ///
    /// Border cells are at x=0, x=width-1, y=0, or y=height-1.
    /// Text cells are interior cells (not on the border).
    /// Background cells would be interior cells without text (contextual).
    /// Row/column-based variants check coordinates against specified indices.
    /// Cell-based variants check exact (x, y) coordinates.
    pub fn should_style(&self, x: u16, y: u16, width: u16, height: u16) -> bool {
        match self {
            StyleRegion::All => true,
            StyleRegion::BorderOnly => {
                x == 0 || y == 0 || x == width.saturating_sub(1) || y == height.saturating_sub(1)
            }
            StyleRegion::TextOnly => {
                // Interior cells (not on border)
                x > 0 && y > 0 && x < width.saturating_sub(1) && y < height.saturating_sub(1)
            }
            StyleRegion::BackgroundOnly => {
                // Same as TextOnly for now - context would determine text vs bg
                x > 0 && y > 0 && x < width.saturating_sub(1) && y < height.saturating_sub(1)
            }
            StyleRegion::Rows(rows) => {
                // Match if y is in the specified row list
                rows.contains(&y)
            }
            StyleRegion::RowRange { start, end } => {
                // Match if y is in [start, end) range
                y >= *start && y < *end
            }
            StyleRegion::Cell { x: cx, y: cy } => {
                // Match exact cell position
                x == *cx && y == *cy
            }
            StyleRegion::Cells(cells) => {
                // Match if (x, y) is in the cell list
                cells.iter().any(|c| c.x == x && c.y == y)
            }
            StyleRegion::Column(col) => {
                // Match if x equals the specified column
                x == *col
            }
            StyleRegion::Columns(cols) => {
                // Match if x is in the specified column list
                cols.contains(&x)
            }
            StyleRegion::ColumnRange { start, end } => {
                // Match if x is in [start, end) range
                x >= *start && x < *end
            }
            StyleRegion::Modulo {
                axis,
                modulus,
                remainder,
            } => {
                // Modulo 0 is invalid - match nothing to avoid panic
                if *modulus == 0 {
                    return false;
                }
                // Remainder >= modulus is impossible - match nothing
                if *remainder >= *modulus {
                    return false;
                }
                let coord = match axis {
                    ModuloAxis::Horizontal => y,
                    ModuloAxis::Vertical => x,
                };
                coord % modulus == *remainder
            }
        }
    }

    /// Get the bounding rectangle for this region.
    ///
    /// Returns `Some((min_x, min_y, width, height))` for bounded regions,
    /// or `None` for unbounded regions like `All`, `TextOnly`, `BorderOnly`, etc.
    ///
    /// This is used to compute region-relative coordinates for spatial shaders.
    pub fn bounding_rect(&self) -> Option<(u16, u16, u16, u16)> {
        match self {
            // Unbounded regions - need grid dimensions to compute bounds
            StyleRegion::All
            | StyleRegion::TextOnly
            | StyleRegion::BorderOnly
            | StyleRegion::BackgroundOnly
            | StyleRegion::Modulo { .. } => None,

            // Row-based regions - unbounded in X
            StyleRegion::Rows(_) | StyleRegion::RowRange { .. } => None,

            // Column-based regions - unbounded in Y
            StyleRegion::Column(_) | StyleRegion::Columns(_) | StyleRegion::ColumnRange { .. } => {
                None
            }

            // Single cell - bounded
            StyleRegion::Cell { x, y } => Some((*x, *y, 1, 1)),

            // Multiple cells - compute bounding box
            StyleRegion::Cells(cells) => {
                if cells.is_empty() {
                    return None;
                }
                let min_x = cells.iter().map(|c| c.x).min().unwrap_or(0);
                let max_x = cells.iter().map(|c| c.x).max().unwrap_or(0);
                let min_y = cells.iter().map(|c| c.y).min().unwrap_or(0);
                let max_y = cells.iter().map(|c| c.y).max().unwrap_or(0);
                let width = max_x.saturating_sub(min_x) + 1;
                let height = max_y.saturating_sub(min_y) + 1;
                Some((min_x, min_y, width, height))
            }
        }
    }

    /// Convert grid coordinates to region-relative coordinates.
    ///
    /// Returns `Some((local_x, local_y, region_width, region_height))` if the region
    /// has a computable bounding box, or `None` for unbounded regions.
    ///
    /// For unbounded regions, the caller should use grid-relative coordinates.
    pub fn to_local_coords(&self, x: u16, y: u16) -> Option<(u16, u16, u16, u16)> {
        self.bounding_rect().map(|(min_x, min_y, width, height)| {
            let local_x = x.saturating_sub(min_x);
            let local_y = y.saturating_sub(min_y);
            (local_x, local_y, width, height)
        })
    }
}

// <FILE>tui-vfx-style/src/models/cls_style_region.rs</FILE> - <DESC>Style region targeting enum</DESC>
// <VERS>END OF VERSION: 4.1.0</VERS>
