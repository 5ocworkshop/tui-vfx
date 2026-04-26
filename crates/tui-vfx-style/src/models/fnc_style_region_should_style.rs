// <FILE>crates/tui-vfx-style/src/models/fnc_style_region_should_style.rs</FILE> - <DESC>Pure function evaluating whether a StyleRegion matches a given cell (with optional role context)</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 3b: RowRange/ColumnRange/Modulo fields became BindableU16, so the predicate must consult `.literal()` and silently mismatch when any coordinate field is still a Binding (callers are expected to resolve via StyleRegion::resolved before entering the hot loop, mirroring the Cell variant's contract).</WCTX>
// <CLOG>RowRange / ColumnRange / Modulo arms now extract literals via `.literal()` and return false if any field is a Binding (matches the pre-existing Cell behavior).</CLOG>

//! `should_style`: decide whether a `StyleRegion` targets a given cell.
//!
//! This is the per-cell predicate the hot render loop calls once per cell
//! per shader layer. It is extracted from `cls_style_region.rs` as a free
//! function so:
//!
//! 1. The method on `StyleRegion` stays a thin delegator (keeping the
//!    `cls_` file within the OFPF size budget).
//! 2. Tests can exercise the predicate without constructing a method call.
//! 3. The signature explicitly carries the `role` context that the new
//!    `StyleRegion::Role(RoleTag)` variant needs — old call-sites that
//!    don't know the role pass `None` and the `Role` variant then matches
//!    nothing (which is the correct "no semantic info" behavior).
//!
//! # Behavior by variant
//!
//! | Variant             | Evaluation                                              |
//! |---------------------|---------------------------------------------------------|
//! | `All`               | always true                                             |
//! | `Role(tag)`         | `role == Some(tag)` (false if `role` is `None`)         |
//! | `Rows(ys)`          | `ys.contains(&y)`                                       |
//! | `RowRange { s, e }` | `y ∈ [s, e)`                                            |
//! | `Cell { x, y }`     | both coords literal and match                           |
//! | `Cells(cells)`      | any coord match                                         |
//! | `Column(c)`         | `x == c`                                                |
//! | `Columns(cs)`       | `cs.contains(&x)`                                       |
//! | `ColumnRange { s,e }`| `x ∈ [s, e)`                                           |
//! | `Modulo { … }`      | per modulus+remainder match on chosen axis              |

use super::cls_bindable_u16::BindableU16;
use super::cls_style_region::{ModuloAxis, StyleRegion};
use tui_vfx_types::{Rect, RoleTag};

/// Decide whether `region` targets the cell at `(x, y)` within `area`, given
/// the cell's optional semantic `role`.
///
/// The `area` argument carries the widget rectangle. Currently only
/// `area.width` / `area.height` are consulted (the region always evaluates
/// in the widget-local coordinate space), but future variants may consult
/// `area.x` / `area.y` — keeping the parameter a `Rect` lets the contract
/// stay future-proof.
///
/// A `Cell { x, y }` whose coordinates are still `BindableU16::Binding`
/// values silently matches nothing. Callers are expected to resolve the
/// region via `StyleRegion::resolved` before entering the hot loop.
pub fn should_style(
    region: &StyleRegion,
    x: u16,
    y: u16,
    role: Option<RoleTag>,
    area: Rect,
) -> bool {
    let width = area.width;
    let height = area.height;
    match region {
        StyleRegion::All => true,
        StyleRegion::Role(tag) => match &role {
            Some(r) => r == tag,
            None => false,
        },
        StyleRegion::Rows(rows) => rows.contains(&y),
        StyleRegion::RowRange { start, end } => match (literal_or(start), literal_or(end)) {
            (Some(s), Some(e)) => y >= s && y < e,
            _ => false,
        },
        StyleRegion::Cell { x: cx, y: cy } => match (literal_or(cx), literal_or(cy)) {
            (Some(cx_lit), Some(cy_lit)) => x == cx_lit && y == cy_lit,
            _ => false,
        },
        StyleRegion::Cells(cells) => cells.iter().any(|c| c.x == x && c.y == y),
        StyleRegion::Column(col) => x == *col,
        StyleRegion::Columns(cols) => cols.contains(&x),
        StyleRegion::ColumnRange { start, end } => match (literal_or(start), literal_or(end)) {
            (Some(s), Some(e)) => x >= s && x < e,
            _ => false,
        },
        StyleRegion::Modulo {
            axis,
            modulus,
            remainder,
        } => {
            let (Some(modulus), Some(remainder)) = (literal_or(modulus), literal_or(remainder))
            else {
                return false;
            };
            if modulus == 0 || remainder >= modulus {
                return false;
            }
            let coord = match axis {
                ModuloAxis::Horizontal => y,
                ModuloAxis::Vertical => x,
            };
            coord % modulus == remainder
        }
        // Silence unused-variable warning when width/height aren't read by
        // the current variant set. They are preserved on the contract for
        // future role-aware or area-sensitive variants.
        #[allow(unreachable_patterns)]
        _ => {
            let _ = (width, height);
            false
        }
    }
}

/// Return the inner literal of a `BindableU16` if it's already lowered to
/// `BindableU16::Literal`, else `None`. Re-exported locally so we don't
/// have to make `BindableU16`'s internals public.
#[inline]
fn literal_or(b: &BindableU16) -> Option<u16> {
    b.literal()
}

// <FILE>crates/tui-vfx-style/src/models/fnc_style_region_should_style.rs</FILE> - <DESC>Pure function evaluating whether a StyleRegion matches a given cell</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
