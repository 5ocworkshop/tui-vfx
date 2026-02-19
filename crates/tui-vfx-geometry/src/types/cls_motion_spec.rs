// <FILE>tui-vfx-geometry/src/types/cls_motion_spec.rs</FILE> - <DESC>Motion specification with from/via/to positioning</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T23:55:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Arbitrary waypoint support</WCTX>
// <CLOG>Initial implementation of MotionSpec for arbitrary waypoint animations</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Rect;

use crate::easing::EasingType;

use super::cls_easing_curve::EasingCurve;
use super::cls_placement_spec::PlacementSpec;
use super::path_type::PathType;
use super::position::Position;
use super::snapping_strategy::SnappingStrategy;

/// A complete motion specification including transition parameters and positioning.
///
/// `MotionSpec` extends the concept of `TransitionSpec` by adding explicit
/// `from`, `via`, and `to` placement specifications. This enables:
///
/// - **Arbitrary start positions**: Not limited to offscreen or anchor positions
/// - **Waypoint control**: The `via` point allows specifying intermediate positions
///   for curved paths (used as the Bezier control point)
/// - **Flexible destinations**: Explicit `to` positioning beyond widget anchors
///
/// # Example: Arc from mid-left to mid-right via top-center
///
/// ```json
/// {
///     "duration_ms": 2000,
///     "ease": "QuadOut",
///     "path": { "type": "Bezier", "control_x": 0, "control_y": 0 },
///     "from": { "type": "FramePermille", "x_permille": 0, "y_permille": 500 },
///     "via": { "type": "FramePermille", "x_permille": 500, "y_permille": 0 },
///     "to": { "type": "FramePermille", "x_permille": 1000, "y_permille": 500 }
/// }
/// ```
///
/// When resolved, the `via` position populates the Bezier control point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct MotionSpec {
    /// Duration of the motion in milliseconds.
    #[config(help = "Duration in milliseconds", default = 500, min = 0)]
    pub duration_ms: u64,

    /// Easing function for the motion.
    pub ease: EasingCurve,

    /// Path type for the motion curve.
    pub path: PathType,

    /// Snapping strategy for final coordinates.
    pub snap: SnappingStrategy,

    /// Starting position specification. If None, uses the widget's current position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<PlacementSpec>,

    /// Intermediate waypoint for curved paths. When present with `PathType::Bezier`,
    /// this position becomes the Bezier control point after resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<PlacementSpec>,

    /// Ending position specification. If None, uses the widget's anchor position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<PlacementSpec>,
}

impl Default for MotionSpec {
    fn default() -> Self {
        Self {
            duration_ms: 500,
            ease: EasingCurve::Type(EasingType::Linear),
            path: PathType::Linear,
            snap: SnappingStrategy::Round,
            from: None,
            via: None,
            to: None,
        }
    }
}

impl MotionSpec {
    /// Resolves the `from` placement to an absolute position.
    ///
    /// Returns `None` if `from` is not specified.
    pub fn resolve_from(&self, frame: Rect, widget_rect: Rect) -> Option<Position> {
        self.from.map(|spec| spec.resolve(frame, Some(widget_rect)))
    }

    /// Resolves the `via` placement to an absolute position.
    ///
    /// Returns `None` if `via` is not specified.
    pub fn resolve_via(&self, frame: Rect, widget_rect: Rect) -> Option<Position> {
        self.via.map(|spec| spec.resolve(frame, Some(widget_rect)))
    }

    /// Resolves the `to` placement to an absolute position.
    ///
    /// Returns `None` if `to` is not specified.
    pub fn resolve_to(&self, frame: Rect, widget_rect: Rect) -> Option<Position> {
        self.to.map(|spec| spec.resolve(frame, Some(widget_rect)))
    }

    /// Resolves the path type, populating Bezier control points from the `via` position if present.
    ///
    /// For `PathType::Bezier`, if a `via` placement is specified, the resolved `via` position
    /// becomes the control point coordinates. Other path types are returned unchanged.
    pub fn resolve_path(&self, frame: Rect, widget_rect: Rect) -> PathType {
        match &self.path {
            PathType::Bezier {
                control_x,
                control_y,
            } => {
                if let Some(via_pos) = self.resolve_via(frame, widget_rect) {
                    PathType::Bezier {
                        control_x: via_pos.x as f32,
                        control_y: via_pos.y as f32,
                    }
                } else {
                    // Keep the original control point values
                    PathType::Bezier {
                        control_x: *control_x,
                        control_y: *control_y,
                    }
                }
            }
            other => other.clone(),
        }
    }

    /// Creates a MotionSpec for a simple linear slide from offscreen.
    pub fn slide_in(
        direction: super::slide_direction::SlideDirection,
        duration_ms: u64,
        ease: EasingCurve,
    ) -> Self {
        Self {
            duration_ms,
            ease,
            path: PathType::Linear,
            snap: SnappingStrategy::Round,
            from: Some(PlacementSpec::Offscreen {
                direction,
                margin_cells: 0,
            }),
            via: None,
            to: None,
        }
    }

    /// Creates a MotionSpec for a Bezier curve animation with explicit waypoint.
    pub fn bezier_arc(
        from: PlacementSpec,
        via: PlacementSpec,
        to: PlacementSpec,
        duration_ms: u64,
        ease: EasingCurve,
    ) -> Self {
        Self {
            duration_ms,
            ease,
            path: PathType::Bezier {
                control_x: 0.0,
                control_y: 0.0,
            },
            snap: SnappingStrategy::Round,
            from: Some(from),
            via: Some(via),
            to: Some(to),
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_motion_spec.rs</FILE> - <DESC>Motion specification with from/via/to positioning</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T23:55:00Z</VERS>
