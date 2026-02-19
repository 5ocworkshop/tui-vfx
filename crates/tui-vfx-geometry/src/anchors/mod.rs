// <FILE>tui-vfx-geometry/src/anchors/mod.rs</FILE>
// <DESC>Anchor placement helpers (legacy parity)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity primitives</WCTX>
// <CLOG>Added anchor position + anchored rect helpers</CLOG>

use tui_vfx_types::{Point, Rect};

use crate::types::{Anchor, SignedRect};

/// Visual center offset used by legacy notifications.
///
/// The middle row anchors use 45% of the frame height rather than 50%.
const VISUAL_CENTER_PERCENT: u16 = 45;

/// Returns the anchor point within a frame (in absolute coordinates).
///
/// This matches legacy behavior:
/// - Right/bottom anchors return the last visible cell (`right()-1`, `bottom()-1`).
/// - Middle anchors use a 45% visual center offset.
pub fn calculate_anchor_position(anchor: Anchor, frame_area: Rect) -> Point {
    let visual_center_y = frame_area.y + (frame_area.height * VISUAL_CENTER_PERCENT / 100);

    match anchor {
        Anchor::TopLeft => Point::new(frame_area.x, frame_area.y),
        Anchor::TopCenter => Point::new(frame_area.x + frame_area.width / 2, frame_area.y),
        Anchor::TopRight => Point::new(frame_area.right().saturating_sub(1), frame_area.y),

        Anchor::MiddleLeft => Point::new(frame_area.x, visual_center_y),
        Anchor::Center => Point::new(frame_area.x + frame_area.width / 2, visual_center_y),
        Anchor::MiddleRight => Point::new(frame_area.right().saturating_sub(1), visual_center_y),

        Anchor::BottomLeft => Point::new(frame_area.x, frame_area.bottom().saturating_sub(1)),
        Anchor::BottomCenter => Point::new(
            frame_area.x + frame_area.width / 2,
            frame_area.bottom().saturating_sub(1),
        ),
        Anchor::BottomRight => Point::new(
            frame_area.right().saturating_sub(1),
            frame_area.bottom().saturating_sub(1),
        ),
        Anchor::Absolute(x, y) => Point::new(x, y),
    }
}

/// Computes the top-left positioned rectangle for a given anchor and size.
///
/// Middle anchors are centered around the *visual center* (45% from top).
pub fn anchored_rect(anchor: Anchor, frame_area: Rect, width: u16, height: u16) -> SignedRect {
    if let Anchor::Absolute(x, y) = anchor {
        return SignedRect::new(x as i32, y as i32, width, height);
    }
    let frame_x = frame_area.x as i32;
    let frame_y = frame_area.y as i32;
    let frame_right = frame_area.right() as i32;
    let frame_bottom = frame_area.bottom() as i32;

    let visual_center_y = frame_area.y as i32 + ((frame_area.height as i32) * 45 / 100);

    let x = match anchor {
        Anchor::TopLeft | Anchor::MiddleLeft | Anchor::BottomLeft => frame_x,
        Anchor::TopCenter | Anchor::Center | Anchor::BottomCenter => {
            // Use signed math so oversized widths center around the visual middle.
            frame_x + ((frame_area.width as i32 - width as i32) / 2)
        }
        Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => frame_right - width as i32,
        Anchor::Absolute(_, _) => unreachable!(),
    };

    let y = match anchor {
        Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight => frame_y,
        Anchor::MiddleLeft | Anchor::Center | Anchor::MiddleRight => {
            visual_center_y - (height as i32) / 2
        }
        Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight => {
            frame_bottom - height as i32
        }
        Anchor::Absolute(_, _) => unreachable!(),
    };

    SignedRect::new(x, y, width, height)
}

// <FILE>tui-vfx-geometry/src/anchors/mod.rs</FILE>
// <DESC>Anchor placement helpers (legacy parity)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
