// <FILE>tui-vfx-style/src/models/cls_trace_common.rs</FILE> - <DESC>Shared trace-route primitives and helpers</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Fix project_onto_polyline distance metric so off-segment cells include parallel offset</WCTX>
// <CLOG>MINOR: project_onto_polyline now returns true point-to-segment Euclidean distance, eliminating phantom-projection ghost lines on TracePath</CLOG>

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
    /// Distance from current segment start to the closest point in cells.
    pub segment_progress: f32,
    /// Length of the current segment in cells.
    pub segment_length: f32,
    /// Index of the segment the projection landed on.
    pub segment_index: usize,
    /// Whether the current segment is vertical.
    pub segment_vertical: bool,
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

        // Compute true point-to-segment distance, not just perpendicular
        // offset. For a cell outside the segment's primary-axis range we must
        // include the parallel "past the endpoint" offset; otherwise every
        // cell sharing the segment's row (or column) is reported at distance 0
        // and gets phantom-projected onto the segment endpoint, producing
        // ghost lines that span the full widget width when a horizontal
        // segment doesn't reach the edges.
        let (distance, progress_on_segment, segment_vertical) = if a.x == b.x {
            let min_y = ay.min(by);
            let max_y = ay.max(by);
            let clamped_y = cell_y.clamp(min_y, max_y);
            let dx = (cell_x - ax).abs();
            let dy_off = (cell_y - clamped_y).abs();
            let dist = (dx * dx + dy_off * dy_off).sqrt();
            (dist, (clamped_y - ay).abs(), true)
        } else if a.y == b.y {
            let min_x = ax.min(bx);
            let max_x = ax.max(bx);
            let clamped_x = cell_x.clamp(min_x, max_x);
            let dy = (cell_y - ay).abs();
            let dx_off = (cell_x - clamped_x).abs();
            let dist = (dx_off * dx_off + dy * dy).sqrt();
            (dist, (clamped_x - ax).abs(), false)
        } else {
            // Non-orthogonal segments are ignored for v1.
            traversed += (bx - ax).abs() + (by - ay).abs();
            continue;
        };

        let segment_length = (bx - ax).abs() + (by - ay).abs();
        let progress = traversed + progress_on_segment;
        let at_turn = segment_index > 0 && segment_index < path.points.len() - 2;
        let candidate = TraceProjection {
            distance,
            progress,
            segment_progress: progress_on_segment,
            segment_length,
            segment_index,
            segment_vertical,
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

pub fn weighted_segment_length(points: (TracePoint, TracePoint), vertical_weight: f32) -> f32 {
    let (a, b) = points;
    if a.x == b.x {
        (b.y as f32 - a.y as f32).abs() * vertical_weight
    } else {
        (b.x as f32 - a.x as f32).abs()
    }
}

pub fn weighted_polyline_total_length(path: &TracePolyline, vertical_weight: f32) -> f32 {
    path.points
        .windows(2)
        .map(|pair| weighted_segment_length((pair[0], pair[1]), vertical_weight))
        .sum()
}

pub fn weighted_progress_for_projection(
    path: &TracePolyline,
    projection: &TraceProjection,
    vertical_weight: f32,
) -> (f32, f32, f32) {
    let mut weighted_before = 0.0_f32;

    for idx in 0..projection.segment_index {
        let a = path.points[idx];
        let b = path.points[idx + 1];
        weighted_before += weighted_segment_length((a, b), vertical_weight);
    }

    let weighted_segment_progress = if projection.segment_vertical {
        projection.segment_progress * vertical_weight
    } else {
        projection.segment_progress
    };
    let weighted_segment_length = if projection.segment_vertical {
        projection.segment_length * vertical_weight
    } else {
        projection.segment_length
    };

    (
        weighted_before + weighted_segment_progress,
        weighted_segment_progress,
        weighted_segment_length,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn polyline(points: &[(u16, u16)]) -> TracePolyline {
        TracePolyline {
            points: points.iter().map(|(x, y)| TracePoint { x: *x, y: *y }).collect(),
            delay: 0.0,
        }
    }

    #[test]
    fn cell_on_horizontal_segment_has_zero_distance() {
        // Segment (5,3)→(15,3). Cell (10,3) is on the segment.
        let path = polyline(&[(5, 3), (15, 3)]);
        let p = project_onto_polyline(10.0, 3.0, &path).expect("projection");
        assert_eq!(p.distance, 0.0);
        assert_eq!(p.segment_progress, 5.0);
    }

    #[test]
    fn cell_off_horizontal_segment_has_perpendicular_distance() {
        // Cell (10,5) is 2 rows below the segment (5,3)→(15,3) but within x range.
        let path = polyline(&[(5, 3), (15, 3)]);
        let p = project_onto_polyline(10.0, 5.0, &path).expect("projection");
        assert_eq!(p.distance, 2.0);
    }

    #[test]
    fn cell_outside_horizontal_segment_x_range_is_not_phantom_projected() {
        // Regression: prior to v1.1.0, project_onto_polyline returned distance
        // = |cell_y - segment_y| even when the cell was outside the segment's
        // x range, causing every cell on row y=3 to phantom-project to the
        // segment endpoint with distance 0. The fix uses true point-to-segment
        // distance, so a cell at (1, 3) — same row as a segment that lives at
        // x=10..20 — must report a distance reflecting the parallel offset
        // (here, |1 - 10| = 9), not 0.
        let path = polyline(&[(10, 3), (20, 3)]);
        let p = project_onto_polyline(1.0, 3.0, &path).expect("projection");
        assert!(
            p.distance > 8.5,
            "expected ≥ 8.5 (parallel offset), got {}",
            p.distance
        );
    }

    #[test]
    fn cell_outside_vertical_segment_y_range_is_not_phantom_projected() {
        // Symmetric regression for vertical segments.
        let path = polyline(&[(10, 3), (10, 8)]);
        let p = project_onto_polyline(10.0, 0.0, &path).expect("projection");
        assert!(
            p.distance > 2.5,
            "expected ≥ 2.5 (parallel offset), got {}",
            p.distance
        );
    }

    #[test]
    fn diagonal_off_segment_uses_euclidean_distance() {
        // Cell (8, 5) relative to horizontal segment (10, 3)→(20, 3):
        // closest point is (10, 3); dx=2, dy=2 → distance √8 ≈ 2.828.
        let path = polyline(&[(10, 3), (20, 3)]);
        let p = project_onto_polyline(8.0, 5.0, &path).expect("projection");
        let expected = (8.0_f32).sqrt();
        assert!(
            (p.distance - expected).abs() < 0.01,
            "expected ≈{}, got {}",
            expected,
            p.distance
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_trace_common.rs</FILE> - <DESC>Shared trace-route primitives and helpers</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
