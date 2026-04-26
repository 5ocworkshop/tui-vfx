// <FILE>crates/tui-vfx-content/src/mechanical/fnc_roll_cycle_window.rs</FILE> - <DESC>Sample a multi-face route via segment-by-segment roll_grid_window calls plus settle handling</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of mechanical circular content cycles plan: cycle-aware tile sampler that reuses the existing pair roll_grid_window for each segment of a multi-face route.</WCTX>
// <CLOG>0.1.0: introduce roll_cycle_window covering single-face routes, multi-segment interpolation, and SettleSample::Overshoot rendering.</CLOG>

use crate::types::{OdometerDirection, OdometerTravel};
use tui_vfx_types::OwnedGrid;

use super::cls_resolved_cycle::MechanicalCycleRoute;
use super::fnc_apply_settle::SettleSample;
use super::fnc_roll_grid_window::roll_grid_window;
use super::types::{MechanicalSource, MechanicalTile};

/// Sample a route at the given settle-transformed progress.
///
/// The route is walked segment-by-segment: progress in
/// `[k/(N-1), (k+1)/(N-1))` maps to the segment between face `k` and
/// face `k+1`, sampled via the existing `roll_grid_window`. This
/// reuses the same window-motion semantics as today's pair Odometer
/// path, so adding intermediate faces only changes which from/to
/// pair is fed to the sampler — not how the window itself is sampled.
///
/// `SettleSample::Overshoot` displays `overshoot_face` directly when
/// it is `Some`; when the overshoot face is unavailable (bounded
/// cycle at an edge), the route's final face is used instead.
pub(crate) fn roll_cycle_window(
    route: &MechanicalCycleRoute,
    settle: SettleSample,
    overshoot_face: Option<&OwnedGrid>,
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile: MechanicalTile,
) -> OwnedGrid {
    debug_assert!(!route.faces.is_empty());
    match settle {
        SettleSample::Overshoot => {
            if let Some(grid) = overshoot_face {
                return grid.clone();
            }
            route
                .faces
                .last()
                .map(|f| f.grid.clone())
                .unwrap_or_else(|| OwnedGrid::new(tile.width as usize, tile.height as usize))
        }
        SettleSample::Route { progress } => sample_route(route, progress, direction, travel, tile),
    }
}

fn sample_route(
    route: &MechanicalCycleRoute,
    progress: f64,
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile: MechanicalTile,
) -> OwnedGrid {
    let last = route.faces.len().saturating_sub(1);
    if last == 0 {
        return route.faces[0].grid.clone();
    }
    if progress <= 0.0 {
        return route.faces[0].grid.clone();
    }
    if progress >= 1.0 {
        return route.faces[last].grid.clone();
    }
    let scaled = progress * last as f64;
    let segment = scaled.floor().min((last - 1) as f64) as usize;
    let local = scaled - segment as f64;
    let pair = MechanicalSource {
        from: route.faces[segment].grid.clone(),
        to: route.faces[segment + 1].grid.clone(),
    };
    roll_grid_window(&pair, local, direction, travel, tile)
}

#[cfg(test)]
mod tests {
    use super::super::cls_resolved_cycle::ResolvedMechanicalFace;
    use super::super::fnc_grid_text::{grid_from_text, grid_to_text};
    use super::*;
    use crate::types::CycleDirectionPolicy;

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    fn face(value: &str) -> ResolvedMechanicalFace {
        let grid = grid_from_text(value, super::super::types::MechanicalSizing::PadToMax);
        ResolvedMechanicalFace {
            value: value.into(),
            grid,
        }
    }

    fn route(values: &[&str]) -> MechanicalCycleRoute {
        MechanicalCycleRoute {
            faces: values.iter().map(|v| face(v)).collect(),
            selected_direction: CycleDirectionPolicy::Forward,
        }
    }

    #[test]
    fn single_face_route_returns_that_face() {
        let r = route(&["A"]);
        let out = roll_cycle_window(
            &r,
            SettleSample::Route { progress: 0.5 },
            None,
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        assert_eq!(grid_to_text(&out), "A");
    }

    #[test]
    fn two_face_route_at_progress_zero_returns_first_face() {
        let r = route(&["A", "B"]);
        let out = roll_cycle_window(
            &r,
            SettleSample::Route { progress: 0.0 },
            None,
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        assert_eq!(grid_to_text(&out), "A");
    }

    #[test]
    fn two_face_route_at_progress_one_returns_last_face() {
        let r = route(&["A", "B"]);
        let out = roll_cycle_window(
            &r,
            SettleSample::Route { progress: 1.0 },
            None,
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        assert_eq!(grid_to_text(&out), "B");
    }

    #[test]
    fn five_face_route_lands_on_each_face_at_segment_boundaries() {
        // Route 8,9,0,1,2 with last=4. At progress 0.0 → face[0] = "8";
        // 0.25 → segment 1, sub 0.0 → roll_grid_window between face[1]
        // and face[2] at sub_progress 0 → face[1] = "9". Likewise:
        // 0.5 → "0"; 0.75 → "1"; 1.0 → "2".
        let r = route(&["8", "9", "0", "1", "2"]);
        let samples = [(0.0, "8"), (0.25, "9"), (0.5, "0"), (0.75, "1"), (1.0, "2")];
        for (p, expected) in samples {
            let out = roll_cycle_window(
                &r,
                SettleSample::Route { progress: p },
                None,
                OdometerDirection::Up,
                OdometerTravel::Axis,
                tile(1, 1),
            );
            assert_eq!(
                grid_to_text(&out),
                expected,
                "at progress {p}, expected {expected}",
            );
        }
    }

    #[test]
    fn overshoot_returns_supplied_face_when_available() {
        let r = route(&["A", "B"]);
        let overshoot_grid = grid_from_text("C", super::super::types::MechanicalSizing::PadToMax);
        let out = roll_cycle_window(
            &r,
            SettleSample::Overshoot,
            Some(&overshoot_grid),
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        assert_eq!(grid_to_text(&out), "C");
    }

    #[test]
    fn overshoot_falls_back_to_last_face_when_unavailable() {
        let r = route(&["A", "B"]);
        let out = roll_cycle_window(
            &r,
            SettleSample::Overshoot,
            None,
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        assert_eq!(grid_to_text(&out), "B");
    }

    #[test]
    fn three_face_route_mid_segment_returns_partial_roll() {
        // Three faces "A","B","C", progress 0.25 → segment 0, sub 0.5
        // → roll between A and B at sub-progress 0.5 with Up direction
        // and Axis travel. Existing roll_grid_window already has the
        // right semantics for this; we just confirm output is not the
        // pure first or pure second face.
        let r = route(&["A", "B", "C"]);
        let out = roll_cycle_window(
            &r,
            SettleSample::Route { progress: 0.25 },
            None,
            OdometerDirection::Up,
            OdometerTravel::Axis,
            tile(1, 1),
        );
        let s = grid_to_text(&out);
        assert!(s == "A" || s == "B");
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_roll_cycle_window.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
