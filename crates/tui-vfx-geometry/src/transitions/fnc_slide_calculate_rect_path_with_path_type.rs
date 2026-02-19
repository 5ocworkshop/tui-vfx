// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect_path_with_path_type.rs</FILE>
// <DESC>Slide rect calculation helper with explicit PathType interpolation</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted slide rect interpolation implementation</CLOG>

use tui_vfx_types::Rect;

use crate::types::{PathType, Position, SignedRect};

use super::col_interpolate_position::interpolate_position;
use super::types::{SlidePath, SlidePhase};

/// Same as `slide_calculate_rect_path`, but uses an explicit `PathType` for interpolation.
pub fn slide_calculate_rect_path_with_path_type(
    path: SlidePath,
    frame_area: Rect,
    progress: f64,
    phase: SlidePhase,
    path_type: &PathType,
) -> Rect {
    let progress = progress.clamp(0.0, 1.0);

    let (from, to) = match phase {
        SlidePhase::SlidingIn => (path.start, path.dwell),
        SlidePhase::SlidingOut => (path.dwell, path.end),
    };

    let (current_x, current_y) = interpolate_position(
        Position::new(from.x, from.y),
        Position::new(to.x, to.y),
        progress,
        path_type,
    );

    // Size is expected to remain constant for slide; use dwell size.
    let width = path.dwell.width as f32;
    let height = path.dwell.height as f32;

    let anim_x1 = current_x;
    let anim_y1 = current_y;
    let anim_x2 = current_x + width;
    let anim_y2 = current_y + height;

    let frame_x1 = frame_area.x as f32;
    let frame_y1 = frame_area.y as f32;
    let frame_x2 = frame_area.right() as f32;
    let frame_y2 = frame_area.bottom() as f32;

    let intersect_x1 = anim_x1.max(frame_x1);
    let intersect_y1 = anim_y1.max(frame_y1);
    let intersect_x2 = anim_x2.min(frame_x2);
    let intersect_y2 = anim_y2.min(frame_y2);

    let intersect_width = (intersect_x2 - intersect_x1).max(0.0);
    let intersect_height = (intersect_y2 - intersect_y1).max(0.0);

    let final_x = intersect_x1.round() as u16;
    let final_y = intersect_y1.round() as u16;
    let final_width = intersect_width.round() as u16;
    let final_height = intersect_height.round() as u16;

    let max_width = frame_area.right().saturating_sub(final_x);
    let max_height = frame_area.bottom().saturating_sub(final_y);

    let final_rect = Rect {
        x: final_x,
        y: final_y,
        width: final_width.min(max_width),
        height: final_height.min(max_height),
    };

    if final_rect.width > 0 && final_rect.height > 0 {
        final_rect
    } else {
        Rect::default()
    }
}

/// Get the unclamped (signed) position for slide animation.
/// Returns the raw interpolated position which can have negative coordinates.
pub fn slide_calculate_signed_rect(
    path: SlidePath,
    progress: f64,
    phase: SlidePhase,
    path_type: &PathType,
) -> SignedRect {
    let progress = progress.clamp(0.0, 1.0);

    let (from, to) = match phase {
        SlidePhase::SlidingIn => (path.start, path.dwell),
        SlidePhase::SlidingOut => (path.dwell, path.end),
    };

    let (current_x, current_y) = interpolate_position(
        Position::new(from.x, from.y),
        Position::new(to.x, to.y),
        progress,
        path_type,
    );

    SignedRect::new(
        current_x.round() as i32,
        current_y.round() as i32,
        path.dwell.width,
        path.dwell.height,
    )
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect_path_with_path_type.rs</FILE>
// <DESC>Slide rect calculation helper with explicit PathType interpolation</DESC>
// <VERS>END OF VERSION: 0.1.2 - 2025-12-19T00:45:00Z</VERS>
