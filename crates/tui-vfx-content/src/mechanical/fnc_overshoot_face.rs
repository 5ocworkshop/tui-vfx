// <FILE>crates/tui-vfx-content/src/mechanical/fnc_overshoot_face.rs</FILE> - <DESC>Look up the overshoot face one step beyond a route's final face in the cycle direction</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of mechanical circular content cycles plan: spring/detent settle needs the next face past the target so the renderer can flash the overshoot face during the settle window.</WCTX>
// <CLOG>0.1.0: introduce overshoot_face_for that returns the overshoot face for circular cycles and None at bounded edges.</CLOG>

use crate::types::{CycleDirectionPolicy, CycleWrapMode};
use tui_vfx_types::OwnedGrid;

use super::cls_resolved_cycle::{MechanicalCycleRoute, ResolvedMechanicalCycle};

/// Return the overshoot face — the face one position past the route's
/// final face in the route's direction.
///
/// For circular cycles, this always exists. For bounded cycles at an
/// edge (forward route ending at the last face, or reverse route
/// ending at the first face), there is no next face and `None` is
/// returned; the caller falls back to rendering the target face.
pub(crate) fn overshoot_face_for<'a>(
    cycle: &'a ResolvedMechanicalCycle,
    route: &MechanicalCycleRoute,
) -> Option<&'a OwnedGrid> {
    let target_value = &route.faces.last()?.value;
    let target_idx = cycle.index_of(target_value)?;
    let len = cycle.faces.len();
    if len == 0 {
        return None;
    }
    let step: i64 = match route.selected_direction {
        CycleDirectionPolicy::Forward
        | CycleDirectionPolicy::Shortest
        | CycleDirectionPolicy::NumericDelta
        | CycleDirectionPolicy::Authored => 1,
        CycleDirectionPolicy::Reverse => -1,
    };
    let next = match cycle.wrap {
        CycleWrapMode::Circular => {
            ((target_idx as i64 + step).rem_euclid(len as i64)) as usize
        }
        CycleWrapMode::Bounded => {
            let raw = target_idx as i64 + step;
            if raw < 0 || raw >= len as i64 {
                return None;
            }
            raw as usize
        }
    };
    Some(&cycle.faces[next].grid)
}

#[cfg(test)]
mod tests {
    use super::super::fnc_grid_text::grid_to_text;
    use super::super::fnc_resolve_mechanical_cycle::resolve_mechanical_cycle;
    use super::super::fnc_route_between::*; // for route_between
    use super::super::types::MechanicalTile;
    use super::*;
    use crate::types::{
        CycleMissingFacePolicy, CycleTieBreaker, CycleWrapMode, MechanicalContentSource,
        MechanicalCyclePreset, MechanicalRouteConfig,
    };

    fn tile() -> MechanicalTile {
        MechanicalTile::new(1, 1).unwrap()
    }

    fn decimal_cycle() -> ResolvedMechanicalCycle {
        resolve_mechanical_cycle(
            &MechanicalContentSource::Preset {
                preset: MechanicalCyclePreset::DecimalDigits,
                wrap: CycleWrapMode::Circular,
            },
            tile(),
        )
        .unwrap()
    }

    fn forward_cfg() -> MechanicalRouteConfig {
        MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Forward,
            tie_breaker: CycleTieBreaker::Forward,
            extra_rotations: 0,
            missing_face: CycleMissingFacePolicy::Error,
        }
    }

    #[test]
    fn forward_route_overshoot_is_next_face_in_circular_cycle() {
        let cycle = decimal_cycle();
        let route = route_between(&cycle, "5", "8", forward_cfg(), None, tile()).unwrap();
        let overshoot = overshoot_face_for(&cycle, &route).expect("circular has overshoot");
        // Forward overshoot of 8 wraps via 9; sample at face[9].
        assert_eq!(grid_to_text(overshoot), "9");
    }

    #[test]
    fn forward_route_overshoot_at_nine_wraps_to_zero_in_circular_cycle() {
        let cycle = decimal_cycle();
        let route = route_between(&cycle, "0", "9", forward_cfg(), None, tile()).unwrap();
        let overshoot = overshoot_face_for(&cycle, &route).expect("circular wraps");
        assert_eq!(grid_to_text(overshoot), "0");
    }

    #[test]
    fn reverse_route_overshoot_is_previous_face_in_circular_cycle() {
        let cycle = decimal_cycle();
        let cfg = MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Reverse,
            ..forward_cfg()
        };
        let route = route_between(&cycle, "5", "2", cfg, None, tile()).unwrap();
        let overshoot = overshoot_face_for(&cycle, &route).expect("circular has overshoot");
        // Reverse overshoot of 2 goes to 1.
        assert_eq!(grid_to_text(overshoot), "1");
    }

    #[test]
    fn bounded_cycle_at_forward_edge_returns_none() {
        let bounded_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&bounded_src, tile()).unwrap();
        let route = route_between(&cycle, "A", "C", forward_cfg(), None, tile()).unwrap();
        // Target "C" is at the forward edge; overshoot would be one
        // past the end → None.
        assert!(overshoot_face_for(&cycle, &route).is_none());
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_overshoot_face.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
