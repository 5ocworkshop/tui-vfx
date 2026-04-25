// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_resolve_cell_placement.rs</FILE> - <DESC>Resolve cell-motion placements</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: signed placement resolution against selected bounds/local frame.</WCTX>
// <CLOG>0.1.0: add authored, origin, absolute, and offscreen resolution.</CLOG>

use super::{CellActor, CellPlacement, CellPlacementBasis};
use tui_vfx_geometry::types::{Position, SlideDirection};
use tui_vfx_types::Rect;

/// Placement-resolution context hoisted once per scheduler sample.
#[derive(Clone, Copy, Debug)]
pub struct CellPlacementContext {
    pub local_frame: Rect,
    pub selected_bounds: Option<Rect>,
}

/// Resolve one placement for one actor to a signed local-frame coordinate.
pub fn resolve_cell_placement(
    actor: &CellActor,
    placement: &CellPlacement,
    ctx: &CellPlacementContext,
) -> Position {
    let authored_x = ctx.local_frame.x as i32 + actor.authored_x as i32;
    let authored_y = ctx.local_frame.y as i32 + actor.authored_y as i32;
    match placement {
        CellPlacement::Authored => Position::new(authored_x, authored_y),
        CellPlacement::AuthoredOffset { dx, dy } => Position::new(authored_x + dx, authored_y + dy),
        CellPlacement::Absolute { x, y } => Position::new(*x, *y),
        CellPlacement::Origin { anchor, basis } => {
            let rect = match basis {
                CellPlacementBasis::SelectionBounds => {
                    ctx.selected_bounds.unwrap_or(ctx.local_frame)
                }
                CellPlacementBasis::LocalFrame => ctx.local_frame,
            };
            anchor_position(*anchor, rect)
        }
        CellPlacement::Offscreen {
            direction,
            margin_cells,
        } => {
            let m = *margin_cells as i32;
            let left = ctx.local_frame.x as i32 - 1 - m;
            let right = ctx.local_frame.right() as i32 + m;
            let top = ctx.local_frame.y as i32 - 1 - m;
            let bottom = ctx.local_frame.bottom() as i32 + m;
            match direction {
                SlideDirection::FromTop => Position::new(authored_x, top),
                SlideDirection::FromBottom => Position::new(authored_x, bottom),
                SlideDirection::FromLeft => Position::new(left, authored_y),
                SlideDirection::FromRight => Position::new(right, authored_y),
                SlideDirection::FromTopLeft => Position::new(left, top),
                SlideDirection::FromTopRight => Position::new(right, top),
                SlideDirection::FromBottomLeft => Position::new(left, bottom),
                SlideDirection::FromBottomRight => Position::new(right, bottom),
                _ => Position::new(authored_x, authored_y),
            }
        }
    }
}

fn anchor_position(anchor: tui_vfx_geometry::types::Anchor, rect: Rect) -> Position {
    let left = rect.x as i32;
    let top = rect.y as i32;
    let mid_x = rect.x as i32 + rect.width as i32 / 2;
    let mid_y = rect.y as i32 + rect.height as i32 / 2;
    let right = rect.right().saturating_sub(1) as i32;
    let bottom = rect.bottom().saturating_sub(1) as i32;
    match anchor {
        tui_vfx_geometry::types::Anchor::TopLeft => Position::new(left, top),
        tui_vfx_geometry::types::Anchor::TopCenter => Position::new(mid_x, top),
        tui_vfx_geometry::types::Anchor::TopRight => Position::new(right, top),
        tui_vfx_geometry::types::Anchor::MiddleLeft => Position::new(left, mid_y),
        tui_vfx_geometry::types::Anchor::Center => Position::new(mid_x, mid_y),
        tui_vfx_geometry::types::Anchor::MiddleRight => Position::new(right, mid_y),
        tui_vfx_geometry::types::Anchor::BottomLeft => Position::new(left, bottom),
        tui_vfx_geometry::types::Anchor::BottomCenter => Position::new(mid_x, bottom),
        tui_vfx_geometry::types::Anchor::BottomRight => Position::new(right, bottom),
        tui_vfx_geometry::types::Anchor::Absolute(x, y) => Position::new(x as i32, y as i32),
        _ => Position::new(mid_x, mid_y),
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_resolve_cell_placement.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
