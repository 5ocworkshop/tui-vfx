// <FILE>tui-vfx-geometry/src/transitions/fnc_expand_collapse_calculate_rect.rs</FILE>
// <DESC>Expand/collapse rect calculation helper</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted expand/collapse rect calculation</CLOG>

use tui_vfx_types::Rect;

use crate::types::SignedRect;

use super::col_lerp::lerp;
use super::types::ExpandPhase;

const MIN_WIDTH: u16 = 3;
const MIN_HEIGHT: u16 = 3;

/// Calculates the visible rectangle for an expand/collapse animation.
///
/// Interpolates size from/to 3×3 while staying centered on the full rect's center.
pub fn expand_collapse_calculate_rect(
    full_rect: SignedRect,
    phase: ExpandPhase,
    progress: f64,
) -> Rect {
    let progress = progress.clamp(0.0, 1.0);
    let progress32 = progress as f32;

    let (start_w, start_h, end_w, end_h) = match phase {
        ExpandPhase::Expanding => (
            MIN_WIDTH as f32,
            MIN_HEIGHT as f32,
            full_rect.width as f32,
            full_rect.height as f32,
        ),
        ExpandPhase::Collapsing => (
            full_rect.width as f32,
            full_rect.height as f32,
            MIN_WIDTH as f32,
            MIN_HEIGHT as f32,
        ),
    };

    let current_width_f32 = lerp(start_w, end_w, progress32);
    let current_height_f32 = lerp(start_h, end_h, progress32);

    let current_width = (current_width_f32.round() as u16).max(if progress > 0.0 { 1 } else { 0 });
    let current_height =
        (current_height_f32.round() as u16).max(if progress > 0.0 { 1 } else { 0 });

    let center_x_full = full_rect.x as f32 + (full_rect.width as f32 / 2.0);
    let center_y_full = full_rect.y as f32 + (full_rect.height as f32 / 2.0);

    let current_x = (center_x_full - (current_width as f32 / 2.0)).round() as i32;
    let current_y = (center_y_full - (current_height as f32 / 2.0)).round() as i32;

    if current_width == 0 || current_height == 0 {
        return Rect::default();
    }

    Rect::new(
        current_x.clamp(0, u16::MAX as i32) as u16,
        current_y.clamp(0, u16::MAX as i32) as u16,
        current_width,
        current_height,
    )
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_expand_collapse_calculate_rect.rs</FILE>
// <DESC>Expand/collapse rect calculation helper</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
