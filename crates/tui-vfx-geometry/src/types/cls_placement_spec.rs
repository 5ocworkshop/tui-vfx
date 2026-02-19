// <FILE>tui-vfx-geometry/src/types/cls_placement_spec.rs</FILE> - <DESC>Unified placement specification</DESC>
// <VERS>VERSION: 2.0.0 - 2025-12-31</VERS>
// <WCTX>V2.2 schema standardization - snake_case serialization</WCTX>
// <CLOG>BREAKING: Changed from PascalCase to snake_case serde serialization</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Rect;

use super::anchor::Anchor;
use super::position::Position;
use super::slide_direction::SlideDirection;

/// A unified specification for positioning elements within or outside a frame.
///
/// `PlacementSpec` provides multiple ways to specify positions:
/// - `Absolute` - exact coordinates
/// - `FramePermille` - frame-relative positioning (0-1000 scale)
/// - `Anchor` - semantic positioning using predefined anchor points
/// - `Offscreen` - positions outside the visible frame for slide animations
///
/// This type is used for specifying `from`, `via`, and `to` positions in
/// motion specifications, enabling arbitrary waypoint animations.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[non_exhaustive]
pub enum PlacementSpec {
    /// Exact signed coordinates (can be offscreen).
    Absolute(Position),

    /// Position expressed in per-mille of the frame (0..=1000).
    ///
    /// - `x_permille = 0`   → left edge
    /// - `x_permille = 500` → center
    /// - `x_permille = 1000` → right edge
    FramePermille {
        #[config(
            help = "X in per-mille of frame width (0..=1000)",
            default = 500,
            min = 0,
            max = 1000
        )]
        x_permille: u16,
        #[config(
            help = "Y in per-mille of frame height (0..=1000)",
            default = 500,
            min = 0,
            max = 1000
        )]
        y_permille: u16,
    },

    /// Semantic positioning using predefined anchor points.
    Anchor { anchor: Anchor },

    /// Position outside the visible frame, used for slide-in/out animations.
    Offscreen {
        #[config(help = "Direction from which to slide")]
        direction: SlideDirection,
        #[serde(default)]
        #[config(help = "Additional margin in cells beyond the frame edge", default = 0)]
        margin_cells: u16,
    },
}

impl Default for PlacementSpec {
    fn default() -> Self {
        Self::Anchor {
            anchor: Anchor::Center,
        }
    }
}

impl PlacementSpec {
    /// Resolves the placement spec to an absolute position.
    ///
    /// # Arguments
    /// - `frame` - The frame rectangle to resolve relative positions against
    /// - `widget_rect` - Optional widget rectangle, required for Offscreen calculations
    ///
    /// # Returns
    /// A resolved `Position` in absolute coordinates
    pub fn resolve(self, frame: Rect, widget_rect: Option<Rect>) -> Position {
        match self {
            PlacementSpec::Absolute(pos) => pos,

            PlacementSpec::FramePermille {
                x_permille,
                y_permille,
            } => {
                let w = frame.width as i32;
                let h = frame.height as i32;

                if w <= 0 || h <= 0 {
                    return Position::new(frame.x as i32, frame.y as i32);
                }

                let x = (w as i64 * x_permille as i64 / 1000) as i32;
                let y = (h as i64 * y_permille as i64 / 1000) as i32;

                // Clamp to last valid cell within the frame
                let x = x.clamp(0, w.saturating_sub(1));
                let y = y.clamp(0, h.saturating_sub(1));

                Position::new(frame.x as i32 + x, frame.y as i32 + y)
            }

            PlacementSpec::Anchor { anchor } => {
                let (x, y) = anchor_to_position(anchor, frame);
                Position::new(x, y)
            }

            PlacementSpec::Offscreen {
                direction,
                margin_cells,
            } => {
                let widget = widget_rect.unwrap_or(Rect::new(0, 0, 1, 1));
                resolve_offscreen(frame, widget, direction, margin_cells)
            }
        }
    }

    /// Creates a FramePermille placement for the center of the frame.
    pub fn center() -> Self {
        Self::FramePermille {
            x_permille: 500,
            y_permille: 500,
        }
    }

    /// Creates a FramePermille placement for the given permille coordinates.
    pub fn permille(x: u16, y: u16) -> Self {
        Self::FramePermille {
            x_permille: x,
            y_permille: y,
        }
    }
}

/// Converts an Anchor to absolute position within a frame.
fn anchor_to_position(anchor: Anchor, frame: Rect) -> (i32, i32) {
    let w = frame.width as i32;
    let h = frame.height as i32;
    let fx = frame.x as i32;
    let fy = frame.y as i32;

    let max_x = (w - 1).max(0);
    let max_y = (h - 1).max(0);

    match anchor {
        Anchor::TopLeft => (fx, fy),
        Anchor::TopCenter => (fx + w / 2, fy),
        Anchor::TopRight => (fx + max_x, fy),
        Anchor::MiddleLeft => (fx, fy + h / 2),
        Anchor::Center => (fx + w / 2, fy + h / 2),
        Anchor::MiddleRight => (fx + max_x, fy + h / 2),
        Anchor::BottomLeft => (fx, fy + max_y),
        Anchor::BottomCenter => (fx + w / 2, fy + max_y),
        Anchor::BottomRight => (fx + max_x, fy + max_y),
        Anchor::Absolute(x, y) => (fx + x as i32, fy + y as i32),
    }
}

/// Resolves an offscreen position based on direction and margin.
fn resolve_offscreen(
    frame: Rect,
    widget: Rect,
    direction: SlideDirection,
    margin: u16,
) -> Position {
    let margin = margin as i32;
    let fw = frame.width as i32;
    let fh = frame.height as i32;
    let fx = frame.x as i32;
    let fy = frame.y as i32;
    let ww = widget.width as i32;
    let wh = widget.height as i32;

    // Default to widget's current position for axes that don't change
    let wx = widget.x as i32;
    let wy = widget.y as i32;

    match direction {
        SlideDirection::Default => Position::new(wx, wy),
        SlideDirection::FromLeft => Position::new(fx - ww - margin, wy),
        SlideDirection::FromRight => Position::new(fx + fw + margin, wy),
        SlideDirection::FromTop => Position::new(wx, fy - wh - margin),
        SlideDirection::FromBottom => Position::new(wx, fy + fh + margin),
        SlideDirection::FromTopLeft => Position::new(fx - ww - margin, fy - wh - margin),
        SlideDirection::FromTopRight => Position::new(fx + fw + margin, fy - wh - margin),
        SlideDirection::FromBottomLeft => Position::new(fx - ww - margin, fy + fh + margin),
        SlideDirection::FromBottomRight => Position::new(fx + fw + margin, fy + fh + margin),
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_placement_spec.rs</FILE> - <DESC>Unified placement specification</DESC>
// <VERS>END OF VERSION: 2.0.0 - 2025-12-31</VERS>
