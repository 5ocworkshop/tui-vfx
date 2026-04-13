// <FILE>tui-vfx-style/src/models/cls_trace_common.rs</FILE> - <DESC>Shared trace-route primitives and helpers</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Provide a common foundation for authored trace paths now and obstacle-aware auto-routing later</WCTX>
// <CLOG>Add shared trace enums, points, polylines, and projection/blending helpers</CLOG>

use crate::models::ColorSpace;
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

/// Starting point for a routed trace pulse.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TraceOrigin {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// Which color channel(s) a trace effect should affect.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TraceApplyTo {
    #[default]
    Foreground,
    Background,
    Both,
}

/// A grid point used by authored trace routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TracePoint {
    pub x: u16,
    pub y: u16,
}

/// One authored polyline route.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TracePolyline {
    /// Orthogonal points making up the route.
    pub points: Vec<TracePoint>,

    /// Optional phase delay before this route activates.
    #[serde(default)]
    pub delay: f32,
}

/// Projected position of a cell onto an authored route.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TraceProjection {
    /// Distance from cell center to the route in cells.
    pub distance: f32,
    /// Distance from route start to the closest point in cells.
    pub progress: f32,
    /// Whether the closest point is a route junction/turn.
    pub at_turn: bool,
}

pub fn origin_point(origin: TraceOrigin, width: u16, height: u16) -> (f32, f32) {
    let max_x = width.saturating_sub(1) as f32;
    let max_y = height.saturating_sub(1) as f32;
    match origin {
        TraceOrigin::TopLeft => (0.0, 0.0),
        TraceOrigin::TopRight => (max_x, 0.0),
        TraceOrigin::BottomLeft => (0.0, max_y),
        TraceOrigin::BottomRight => (max_x, max_y),
        TraceOrigin::Center => (max_x / 2.0, max_y / 2.0),
    }
}

pub fn blend_trace_target(base: Color, trace: Color, alpha: f32) -> Color {
    if base == Color::TRANSPARENT {
        trace
    } else {
        blend_colors(base, trace, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
    }
}

pub fn max_distance_from_origin(origin_x: f32, origin_y: f32, width: u16, height: u16) -> f32 {
    let corners = [
        (0.0, 0.0),
        (width.saturating_sub(1) as f32, 0.0),
        (0.0, height.saturating_sub(1) as f32),
        (
            width.saturating_sub(1) as f32,
            height.saturating_sub(1) as f32,
        ),
    ];
    corners
        .into_iter()
        .map(|(x, y)| (x - origin_x).abs() + (y - origin_y).abs())
        .fold(0.0, f32::max)
}

/// Returns the distance/progress of a cell relative to an authored orthogonal polyline.
pub fn project_onto_polyline(
    cell_x: f32,
    cell_y: f32,
    path: &TracePolyline,
) -> Option<TraceProjection> {
    if path.points.len() < 2 {
        return None;
    }

    let mut best: Option<TraceProjection> = None;
    let mut traversed = 0.0_f32;

    for segment_index in 0..(path.points.len() - 1) {
        let a = path.points[segment_index];
        let b = path.points[segment_index + 1];
        let ax = a.x as f32;
        let ay = a.y as f32;
        let bx = b.x as f32;
        let by = b.y as f32;

        let (distance, progress_on_segment) = if a.x == b.x {
            let min_y = ay.min(by);
            let max_y = ay.max(by);
            let clamped_y = cell_y.clamp(min_y, max_y);
            ((cell_x - ax).abs(), (clamped_y - ay).abs())
        } else if a.y == b.y {
            let min_x = ax.min(bx);
            let max_x = ax.max(bx);
            let clamped_x = cell_x.clamp(min_x, max_x);
            ((cell_y - ay).abs(), (clamped_x - ax).abs())
        } else {
            // Non-orthogonal segments are ignored for v1.
            traversed += (bx - ax).abs() + (by - ay).abs();
            continue;
        };

        let progress = traversed + progress_on_segment;
        let at_turn = segment_index > 0 && segment_index < path.points.len() - 2;
        let candidate = TraceProjection {
            distance,
            progress,
            at_turn,
        };

        best = match best {
            Some(current) if current.distance <= candidate.distance => Some(current),
            _ => Some(candidate),
        };

        traversed += (bx - ax).abs() + (by - ay).abs();
    }

    best
}

pub fn polyline_total_length(path: &TracePolyline) -> f32 {
    path.points
        .windows(2)
        .map(|pair| {
            let a = pair[0];
            let b = pair[1];
            (b.x as f32 - a.x as f32).abs() + (b.y as f32 - a.y as f32).abs()
        })
        .sum()
}

// <FILE>tui-vfx-style/src/models/cls_trace_common.rs</FILE> - <DESC>Shared trace-route primitives and helpers</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
