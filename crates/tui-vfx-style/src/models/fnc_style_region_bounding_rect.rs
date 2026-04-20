// <FILE>crates/tui-vfx-style/src/models/fnc_style_region_bounding_rect.rs</FILE> - <DESC>Pure function returning a StyleRegion's bounding rectangle within the widget area</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.0 — pre-extract bounding_rect from cls_style_region.rs so we stay within the `cls_` LOC budget while adding the role-aware Role(RoleTag) variant</WCTX>
// <CLOG>0.1.0: initial extraction. Returns a concrete `Rect` (origin + size) rather than the historical `(u16, u16, u16, u16)` tuple so the result composes directly with other geometry primitives. `None` result semantics preserved (unbounded or unresolved regions). `Role(RoleTag)` variant is unbounded (same as `All`) — role membership is role-map-indexed, not rectangular.</CLOG>

//! `bounding_rect`: the smallest rectangle enclosing all cells the region
//! targets, or `None` if the region is unbounded (or depends on per-cell
//! semantic data that rectangles can't express).
//!
//! Extracted from `cls_style_region.rs` for the same reasons as
//! `fnc_style_region_should_style.rs`: keeps the `cls_` file a data-only
//! enum with serde, lets tests call the predicate directly, and gives the
//! new `Role(RoleTag)` variant a home without blowing the LOC budget.

use super::cls_style_region::StyleRegion;
use tui_vfx_types::Rect;

/// Return the bounding rectangle of `region` within the widget `area`.
///
/// Returns `None` when the region is unbounded in one or both axes
/// (`All`, `Role(...)`, `Modulo`, row-based, column-based) OR when a
/// `Cell` variant still carries unresolved `BindableU16::Binding`
/// coordinates (callers should `resolved()` first).
///
/// The `area` argument is available for future variants whose bounds
/// depend on the widget rectangle; the current variant set does not
/// consult it.
pub fn bounding_rect(region: &StyleRegion, area: Rect) -> Option<Rect> {
    let _ = area;
    match region {
        // Unbounded regions
        StyleRegion::All
        | StyleRegion::Role(_)
        | StyleRegion::Modulo { .. }
        | StyleRegion::Rows(_)
        | StyleRegion::RowRange { .. }
        | StyleRegion::Column(_)
        | StyleRegion::Columns(_)
        | StyleRegion::ColumnRange { .. } => None,

        // Single cell — literal coords only
        StyleRegion::Cell { x, y } => match (x.literal(), y.literal()) {
            (Some(xl), Some(yl)) => Some(Rect::new(xl, yl, 1, 1)),
            _ => None,
        },

        // Multiple cells — tight bounding box
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
            Some(Rect::new(min_x, min_y, width, height))
        }
    }
}

// <FILE>crates/tui-vfx-style/src/models/fnc_style_region_bounding_rect.rs</FILE> - <DESC>Pure function returning a StyleRegion's bounding rectangle</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
