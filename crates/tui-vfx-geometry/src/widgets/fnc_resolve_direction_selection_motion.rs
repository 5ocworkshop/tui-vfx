// <FILE>tui-vfx-geometry/src/widgets/fnc_resolve_direction_selection_motion.rs</FILE>
// <DESC>Map numpad direction selection to motion primitives (direction + path)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy parity: edge "arc hints" should map to Arc path bulges</WCTX>
// <CLOG>Added DirectionSelectionMotion resolver</CLOG>

use crate::types::{PathType, SlideDirection};
use crate::widgets::col_numpad_mapping::{anchor_from_numpad_digit, direction_from_numpad_digit};
use crate::widgets::types::{DirectionNumpadSelection, DirectionSelectionMotion};

fn arc_bulge_for_edge_hint(digit: char, hint: SlideDirection, magnitude: f32) -> Option<f32> {
    if magnitude <= 0.0 {
        return None;
    }

    use SlideDirection::*;

    // Slide arcs are authored in screen axes:
    // - left/right edges use up/down bulges
    // - top/bottom edges use left/right bulges
    match (digit, hint) {
        ('4' | '6', FromTop) => Some(-magnitude),
        ('4' | '6', FromBottom) => Some(magnitude),

        ('8' | '2', FromLeft) => Some(-magnitude),
        ('8' | '2', FromRight) => Some(magnitude),

        _ => None,
    }
}

/// Resolves a `DirectionNumpadSelection` into:
/// - an anchor position (numpad digit)
/// - a base slide direction (used for offscreen positioning)
/// - a `PathType` that encodes edge "arc hints" via `PathType::Arc`
pub fn resolve_direction_selection_motion(
    selection: DirectionNumpadSelection,
    arc_bulge_magnitude: f32,
) -> Option<DirectionSelectionMotion> {
    let anchor = anchor_from_numpad_digit(selection.digit)?;
    let hint_direction = selection.resolve();

    // Edges interpret non-straight selections as "arc hints".
    let base_straight = direction_from_numpad_digit(selection.digit);
    let (base_direction, path) = if let Some(base) = base_straight {
        if matches!(selection.digit, '2' | '4' | '6' | '8') {
            if let Some(bulge) =
                arc_bulge_for_edge_hint(selection.digit, hint_direction, arc_bulge_magnitude)
            {
                (base, PathType::Arc { bulge })
            } else {
                (hint_direction, PathType::Linear)
            }
        } else {
            // Corners and center treat the selection literally.
            (hint_direction, PathType::Linear)
        }
    } else {
        (hint_direction, PathType::Linear)
    };

    Some(DirectionSelectionMotion {
        anchor,
        hint_direction,
        base_direction,
        path,
    })
}

// <FILE>tui-vfx-geometry/src/widgets/fnc_resolve_direction_selection_motion.rs</FILE>
// <DESC>Map numpad direction selection to motion primitives (direction + path)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
