// <FILE>crates/tui-vfx-content/src/mechanical/fnc_route_between.rs</FILE> - <DESC>Build a concrete face-by-face route between source and target through a ResolvedMechanicalCycle</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: route construction honoring direction policy, tie-breaker, extra rotations, missing-face policy, wrap mode, and numeric-delta hint.</WCTX>
// <CLOG>0.1.0: introduce route_between covering Pair, Forward, Reverse, Shortest, NumericDelta with Hint or numeric inference; PairFallback and InsertAtEnd missing-face policies; Bounded rejection.</CLOG>

use crate::types::{
    CycleDirectionPolicy, CycleMissingFacePolicy, CycleTieBreaker, CycleWrapMode,
    MechanicalRouteConfig,
};

use super::cls_resolved_cycle::{
    MechanicalCycleRoute, NumericRouteHint, ResolvedMechanicalCycle, ResolvedMechanicalFace,
};
use super::enum_cycle_error::MechanicalCycleError;
use super::fnc_normalize_cycle_face::normalize_cycle_face;
use super::types::MechanicalTile;

/// Build a route from `from` to `to` through `cycle`.
///
/// The returned route always includes both endpoints. The walk
/// direction is decided by `config.direction` together with the
/// `numeric_hint` for `CycleDirectionPolicy::NumericDelta`. `extra_
/// rotations` appends complete wraps in the selected direction before
/// the final face. Missing endpoints are handled per
/// `config.missing_face`.
///
/// `tile` is used only to normalize Pair-fallback face grids; for
/// every other path the resolved cycle's normalized grids are reused.
pub(crate) fn route_between(
    cycle: &ResolvedMechanicalCycle,
    from: &str,
    to: &str,
    config: MechanicalRouteConfig,
    numeric_hint: Option<NumericRouteHint>,
    tile: MechanicalTile,
) -> Result<MechanicalCycleRoute, MechanicalCycleError> {
    if matches!(config.direction, CycleDirectionPolicy::Authored) {
        return Err(MechanicalCycleError::AuthoredDirectionReserved);
    }
    if config.extra_rotations > 0 && !matches!(cycle.wrap, CycleWrapMode::Circular) {
        return Err(MechanicalCycleError::ExtraRotationsRequireCircular);
    }

    if cycle.faces.is_empty() {
        // Pair-shaped resolved cycle: no intermediate faces. Return
        // [from, to] using the supplied tile to normalize endpoints.
        return pair_route(from, to, tile, CycleDirectionPolicy::Forward);
    }

    let from_idx_opt = cycle.index_of(from);
    let to_idx_opt = cycle.index_of(to);

    let (from_idx, to_idx) = match (from_idx_opt, to_idx_opt) {
        (Some(a), Some(b)) => (a, b),
        _ => match config.missing_face {
            CycleMissingFacePolicy::Error => {
                let missing = if from_idx_opt.is_none() { from } else { to };
                return Err(MechanicalCycleError::MissingFace {
                    value: missing.to_string(),
                });
            }
            CycleMissingFacePolicy::PairFallback => {
                return pair_route(from, to, tile, CycleDirectionPolicy::Forward);
            }
            CycleMissingFacePolicy::InsertAtEnd => {
                return insert_at_end_route(cycle, from, to, tile, &config, numeric_hint);
            }
        },
    };

    let direction = resolve_direction(cycle, from_idx, to_idx, &config, numeric_hint, from, to)?;

    let walk = walk_indices(cycle, from_idx, to_idx, direction, config.extra_rotations)?;
    let faces: Vec<ResolvedMechanicalFace> = walk
        .into_iter()
        .map(|idx| cycle.faces[idx].clone())
        .collect();
    Ok(MechanicalCycleRoute {
        faces,
        selected_direction: direction,
    })
}

fn pair_route(
    from: &str,
    to: &str,
    tile: MechanicalTile,
    selected_direction: CycleDirectionPolicy,
) -> Result<MechanicalCycleRoute, MechanicalCycleError> {
    let from_grid = normalize_cycle_face(from, tile)?;
    let to_grid = normalize_cycle_face(to, tile)?;
    Ok(MechanicalCycleRoute {
        faces: vec![
            ResolvedMechanicalFace {
                value: from.to_string(),
                grid: from_grid,
            },
            ResolvedMechanicalFace {
                value: to.to_string(),
                grid: to_grid,
            },
        ],
        selected_direction,
    })
}

fn insert_at_end_route(
    cycle: &ResolvedMechanicalCycle,
    from: &str,
    to: &str,
    tile: MechanicalTile,
    config: &MechanicalRouteConfig,
    numeric_hint: Option<NumericRouteHint>,
) -> Result<MechanicalCycleRoute, MechanicalCycleError> {
    let mut extended = cycle.clone();
    if extended.index_of(from).is_none() {
        let grid = normalize_cycle_face(from, tile)?;
        extended.faces.push(ResolvedMechanicalFace {
            value: from.to_string(),
            grid,
        });
    }
    if extended.index_of(to).is_none() {
        let grid = normalize_cycle_face(to, tile)?;
        extended.faces.push(ResolvedMechanicalFace {
            value: to.to_string(),
            grid,
        });
    }
    // Recurse with strict missing-face since both endpoints are now
    // present; avoid infinite recursion by overriding the policy.
    let strict = MechanicalRouteConfig {
        direction: config.direction,
        tie_breaker: config.tie_breaker,
        extra_rotations: config.extra_rotations,
        missing_face: CycleMissingFacePolicy::Error,
    };
    route_between(&extended, from, to, strict, numeric_hint, tile)
}

fn resolve_direction(
    cycle: &ResolvedMechanicalCycle,
    from_idx: usize,
    to_idx: usize,
    config: &MechanicalRouteConfig,
    numeric_hint: Option<NumericRouteHint>,
    from: &str,
    to: &str,
) -> Result<CycleDirectionPolicy, MechanicalCycleError> {
    match config.direction {
        CycleDirectionPolicy::Forward | CycleDirectionPolicy::Reverse => Ok(config.direction),
        CycleDirectionPolicy::Authored => Err(MechanicalCycleError::AuthoredDirectionReserved),
        CycleDirectionPolicy::Shortest => resolve_shortest(cycle, from_idx, to_idx, config),
        CycleDirectionPolicy::NumericDelta => resolve_numeric_delta(cycle, numeric_hint, from, to),
    }
}

fn resolve_shortest(
    cycle: &ResolvedMechanicalCycle,
    from_idx: usize,
    to_idx: usize,
    config: &MechanicalRouteConfig,
) -> Result<CycleDirectionPolicy, MechanicalCycleError> {
    if !matches!(cycle.wrap, CycleWrapMode::Circular) {
        return Err(MechanicalCycleError::ShortestRequiresCircular);
    }
    let len = cycle.faces.len();
    let forward_dist = (to_idx + len - from_idx) % len;
    let reverse_dist = (from_idx + len - to_idx) % len;
    if forward_dist == reverse_dist {
        Ok(match config.tie_breaker {
            CycleTieBreaker::Forward => CycleDirectionPolicy::Forward,
            CycleTieBreaker::Reverse => CycleDirectionPolicy::Reverse,
        })
    } else if forward_dist < reverse_dist {
        Ok(CycleDirectionPolicy::Forward)
    } else {
        Ok(CycleDirectionPolicy::Reverse)
    }
}

fn resolve_numeric_delta(
    cycle: &ResolvedMechanicalCycle,
    numeric_hint: Option<NumericRouteHint>,
    from: &str,
    to: &str,
) -> Result<CycleDirectionPolicy, MechanicalCycleError> {
    if !cycle_is_decimal_digits(cycle) {
        return Err(MechanicalCycleError::NumericDeltaRequiresDigits);
    }
    if let Some(hint) = numeric_hint {
        return Ok(match hint {
            NumericRouteHint::Increment => CycleDirectionPolicy::Forward,
            NumericRouteHint::Decrement => CycleDirectionPolicy::Reverse,
        });
    }
    let from_n: i64 = from
        .parse()
        .map_err(|_| MechanicalCycleError::NumericDeltaRequiresDigits)?;
    let to_n: i64 = to
        .parse()
        .map_err(|_| MechanicalCycleError::NumericDeltaRequiresDigits)?;
    if to_n >= from_n {
        Ok(CycleDirectionPolicy::Forward)
    } else {
        Ok(CycleDirectionPolicy::Reverse)
    }
}

fn cycle_is_decimal_digits(cycle: &ResolvedMechanicalCycle) -> bool {
    if cycle.faces.len() != 10 {
        return false;
    }
    cycle
        .faces
        .iter()
        .enumerate()
        .all(|(i, f)| f.value == i.to_string())
}

fn walk_indices(
    cycle: &ResolvedMechanicalCycle,
    from_idx: usize,
    to_idx: usize,
    direction: CycleDirectionPolicy,
    extra_rotations: u16,
) -> Result<Vec<usize>, MechanicalCycleError> {
    let len = cycle.faces.len();
    debug_assert!(len >= 1);

    if from_idx == to_idx && extra_rotations == 0 {
        return Ok(vec![from_idx]);
    }

    let step: i64 = match direction {
        CycleDirectionPolicy::Forward => 1,
        CycleDirectionPolicy::Reverse => -1,
        // Should be reduced to Forward/Reverse before reaching here.
        CycleDirectionPolicy::Shortest
        | CycleDirectionPolicy::NumericDelta
        | CycleDirectionPolicy::Authored => 1,
    };

    let circular = matches!(cycle.wrap, CycleWrapMode::Circular);
    if !circular {
        let ordered_ok = match direction {
            CycleDirectionPolicy::Forward => from_idx <= to_idx,
            CycleDirectionPolicy::Reverse => from_idx >= to_idx,
            _ => false,
        };
        if !ordered_ok {
            return Err(MechanicalCycleError::BoundedRouteImpossible {
                from: cycle.faces[from_idx].value.clone(),
                to: cycle.faces[to_idx].value.clone(),
                direction: match direction {
                    CycleDirectionPolicy::Forward => "forward",
                    CycleDirectionPolicy::Reverse => "reverse",
                    _ => "unknown",
                },
            });
        }
    }

    let mut indices: Vec<usize> = Vec::new();
    let mut idx = from_idx as i64;
    indices.push(idx as usize);
    let len_i = len as i64;
    // First-pass walk to to_idx.
    while (idx as usize) != to_idx {
        idx = if circular {
            (idx + step).rem_euclid(len_i)
        } else {
            idx + step
        };
        if !circular && (idx < 0 || idx >= len_i) {
            return Err(MechanicalCycleError::BoundedRouteImpossible {
                from: cycle.faces[from_idx].value.clone(),
                to: cycle.faces[to_idx].value.clone(),
                direction: match direction {
                    CycleDirectionPolicy::Forward => "forward",
                    CycleDirectionPolicy::Reverse => "reverse",
                    _ => "unknown",
                },
            });
        }
        indices.push(idx as usize);
    }

    // extra_rotations: append complete wraps in the same direction.
    // After the first leg, idx is at to_idx (already in `indices`).
    // Each wrap walks `len` steps, ending back on to_idx — pushing
    // every step gives a wrap that does not double the landing face.
    for _ in 0..extra_rotations {
        for _ in 0..len {
            idx = (idx + step).rem_euclid(len_i);
            indices.push(idx as usize);
        }
    }

    Ok(indices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mechanical::fnc_resolve_mechanical_cycle::resolve_mechanical_cycle;
    use crate::types::{MechanicalContentSource, MechanicalCyclePreset};

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    fn decimal_cycle() -> ResolvedMechanicalCycle {
        resolve_mechanical_cycle(
            &MechanicalContentSource::Preset {
                preset: MechanicalCyclePreset::DecimalDigits,
                wrap: CycleWrapMode::Circular,
                font: None,
            },
            tile(1, 1),
        )
        .unwrap()
    }

    fn route_values(route: &MechanicalCycleRoute) -> Vec<&str> {
        route.faces.iter().map(|f| f.value.as_str()).collect()
    }

    fn cfg(direction: CycleDirectionPolicy) -> MechanicalRouteConfig {
        MechanicalRouteConfig {
            direction,
            tie_breaker: CycleTieBreaker::Forward,
            extra_rotations: 0,
            missing_face: CycleMissingFacePolicy::Error,
        }
    }

    #[test]
    fn decimal_forward_8_to_2() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "8",
            "2",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["8", "9", "0", "1", "2"]);
        assert_eq!(route.selected_direction, CycleDirectionPolicy::Forward);
    }

    #[test]
    fn decimal_reverse_2_to_8() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "2",
            "8",
            cfg(CycleDirectionPolicy::Reverse),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["2", "1", "0", "9", "8"]);
        assert_eq!(route.selected_direction, CycleDirectionPolicy::Reverse);
    }

    #[test]
    fn decimal_forward_9_to_0_wraps_circular_in_one_step() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "9",
            "0",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["9", "0"]);
    }

    #[test]
    fn decimal_forward_0_to_9_walks_all_ten_faces() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "0",
            "9",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(
            route_values(&route),
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
        );
    }

    #[test]
    fn decimal_shortest_8_to_2_picks_forward() {
        // forward 8→2 = 4 steps; reverse 8→2 = 6 steps; pick forward.
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "8",
            "2",
            cfg(CycleDirectionPolicy::Shortest),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["8", "9", "0", "1", "2"]);
        assert_eq!(route.selected_direction, CycleDirectionPolicy::Forward);
    }

    #[test]
    fn decimal_shortest_2_to_8_picks_reverse() {
        // forward 2→8 = 6 steps; reverse 2→8 = 4 steps; pick reverse.
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "2",
            "8",
            cfg(CycleDirectionPolicy::Shortest),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["2", "1", "0", "9", "8"]);
    }

    #[test]
    fn decimal_shortest_tie_resolves_via_tie_breaker() {
        // 0 -> 5 is 5 steps either way; tie_breaker decides.
        let cycle = decimal_cycle();
        let forward_tie = MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Shortest,
            tie_breaker: CycleTieBreaker::Forward,
            extra_rotations: 0,
            missing_face: CycleMissingFacePolicy::Error,
        };
        let route = route_between(&cycle, "0", "5", forward_tie, None, tile(1, 1)).unwrap();
        assert_eq!(route_values(&route), vec!["0", "1", "2", "3", "4", "5"]);

        let reverse_tie = MechanicalRouteConfig {
            direction: CycleDirectionPolicy::Shortest,
            tie_breaker: CycleTieBreaker::Reverse,
            extra_rotations: 0,
            missing_face: CycleMissingFacePolicy::Error,
        };
        let route = route_between(&cycle, "0", "5", reverse_tie, None, tile(1, 1)).unwrap();
        assert_eq!(route_values(&route), vec!["0", "9", "8", "7", "6", "5"]);
    }

    #[test]
    fn pair_resolved_cycle_returns_from_to() {
        let pair = resolve_mechanical_cycle(&MechanicalContentSource::Pair, tile(1, 1)).unwrap();
        let route = route_between(
            &pair,
            "X",
            "Y",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["X", "Y"]);
    }

    #[test]
    fn missing_face_strict_errors() {
        let cycle = decimal_cycle();
        let err = route_between(
            &cycle,
            "?",
            "0",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap_err();
        match err {
            MechanicalCycleError::MissingFace { value } => assert_eq!(value, "?"),
            other => panic!("expected MissingFace, got {other:?}"),
        }
    }

    #[test]
    fn missing_face_pair_fallback_returns_pair_route() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "?",
            "0",
            MechanicalRouteConfig {
                direction: CycleDirectionPolicy::Forward,
                tie_breaker: CycleTieBreaker::Forward,
                extra_rotations: 0,
                missing_face: CycleMissingFacePolicy::PairFallback,
            },
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["?", "0"]);
    }

    #[test]
    fn missing_face_insert_at_end_walks_through_appended_face() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "9",
            "X",
            MechanicalRouteConfig {
                direction: CycleDirectionPolicy::Forward,
                tie_breaker: CycleTieBreaker::Forward,
                extra_rotations: 0,
                missing_face: CycleMissingFacePolicy::InsertAtEnd,
            },
            None,
            tile(1, 1),
        )
        .unwrap();
        // X is appended at index 10; forward 9->10 takes one step.
        assert_eq!(route_values(&route), vec!["9", "X"]);
    }

    #[test]
    fn extra_rotations_appends_full_wraps() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "8",
            "2",
            MechanicalRouteConfig {
                direction: CycleDirectionPolicy::Forward,
                tie_breaker: CycleTieBreaker::Forward,
                extra_rotations: 1,
                missing_face: CycleMissingFacePolicy::Error,
            },
            None,
            tile(1, 1),
        )
        .unwrap();
        // forward 8→2 = 8,9,0,1,2 then one extra wrap = 3,4,5,6,7,8,9,0,1,2.
        assert_eq!(
            route_values(&route),
            vec![
                "8", "9", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "1", "2",
            ],
        );
    }

    #[test]
    fn extra_rotations_on_bounded_cycle_rejected() {
        let bounded_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&bounded_src, tile(1, 1)).unwrap();
        let err = route_between(
            &cycle,
            "A",
            "C",
            MechanicalRouteConfig {
                direction: CycleDirectionPolicy::Forward,
                tie_breaker: CycleTieBreaker::Forward,
                extra_rotations: 1,
                missing_face: CycleMissingFacePolicy::Error,
            },
            None,
            tile(1, 1),
        )
        .unwrap_err();
        assert_eq!(err, MechanicalCycleError::ExtraRotationsRequireCircular);
    }

    #[test]
    fn bounded_cycle_rejects_reverse_walk_off_either_end() {
        let bounded_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&bounded_src, tile(1, 1)).unwrap();
        let err = route_between(
            &cycle,
            "A",
            "C",
            cfg(CycleDirectionPolicy::Reverse),
            None,
            tile(1, 1),
        )
        .unwrap_err();
        assert!(matches!(
            err,
            MechanicalCycleError::BoundedRouteImpossible { .. }
        ));
    }

    #[test]
    fn bounded_cycle_walks_forward_when_in_order() {
        let bounded_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&bounded_src, tile(1, 1)).unwrap();
        let route = route_between(
            &cycle,
            "A",
            "C",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["A", "B", "C"]);
    }

    #[test]
    fn shortest_on_bounded_cycle_rejected() {
        let bounded_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&bounded_src, tile(1, 1)).unwrap();
        let err = route_between(
            &cycle,
            "A",
            "C",
            cfg(CycleDirectionPolicy::Shortest),
            None,
            tile(1, 1),
        )
        .unwrap_err();
        assert_eq!(err, MechanicalCycleError::ShortestRequiresCircular);
    }

    #[test]
    fn numeric_delta_with_increment_hint_routes_forward() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "9",
            "0",
            cfg(CycleDirectionPolicy::NumericDelta),
            Some(NumericRouteHint::Increment),
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["9", "0"]);
    }

    #[test]
    fn numeric_delta_with_decrement_hint_routes_reverse() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "0",
            "9",
            cfg(CycleDirectionPolicy::NumericDelta),
            Some(NumericRouteHint::Decrement),
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["0", "9"]);
    }

    #[test]
    fn numeric_delta_no_hint_infers_from_numeric_compare_increment() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "2",
            "8",
            cfg(CycleDirectionPolicy::NumericDelta),
            None,
            tile(1, 1),
        )
        .unwrap();
        // 8 > 2 → infer increment → forward → 2,3,4,5,6,7,8.
        assert_eq!(
            route_values(&route),
            vec!["2", "3", "4", "5", "6", "7", "8"]
        );
    }

    #[test]
    fn numeric_delta_on_non_digit_cycle_rejected() {
        let alpha_src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&alpha_src, tile(1, 1)).unwrap();
        let err = route_between(
            &cycle,
            "A",
            "C",
            cfg(CycleDirectionPolicy::NumericDelta),
            Some(NumericRouteHint::Increment),
            tile(1, 1),
        )
        .unwrap_err();
        assert_eq!(err, MechanicalCycleError::NumericDeltaRequiresDigits);
    }

    #[test]
    fn authored_direction_rejected() {
        let cycle = decimal_cycle();
        let err = route_between(
            &cycle,
            "0",
            "9",
            cfg(CycleDirectionPolicy::Authored),
            None,
            tile(1, 1),
        )
        .unwrap_err();
        assert_eq!(err, MechanicalCycleError::AuthoredDirectionReserved);
    }

    #[test]
    fn same_endpoint_with_no_extra_rotations_returns_single_face() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "5",
            "5",
            cfg(CycleDirectionPolicy::Forward),
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(route_values(&route), vec!["5"]);
    }

    #[test]
    fn same_endpoint_with_extra_rotations_spins_full_circle() {
        let cycle = decimal_cycle();
        let route = route_between(
            &cycle,
            "5",
            "5",
            MechanicalRouteConfig {
                direction: CycleDirectionPolicy::Forward,
                tie_breaker: CycleTieBreaker::Forward,
                extra_rotations: 1,
                missing_face: CycleMissingFacePolicy::Error,
            },
            None,
            tile(1, 1),
        )
        .unwrap();
        assert_eq!(
            route_values(&route),
            vec!["5", "6", "7", "8", "9", "0", "1", "2", "3", "4", "5"],
        );
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_route_between.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
