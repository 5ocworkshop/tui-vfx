// <FILE>crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs</FILE> - <DESC>Compose preset expansion, deterministic shuffle, and face normalization into a ResolvedMechanicalCycle</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: source config to ResolvedMechanicalCycle resolver consumed by route_between.</WCTX>
// <CLOG>0.1.0: introduce resolve_mechanical_cycle handling Pair, Ordered, Preset, Randomized, Weighted; enforce duplicate-face and circular-min-size rules.</CLOG>

use std::collections::HashSet;

use crate::types::{CycleWrapMode, MechanicalContentSource};

use super::cls_resolved_cycle::{ResolvedMechanicalCycle, ResolvedMechanicalFace};
use super::enum_cycle_error::MechanicalCycleError;
use super::fnc_expand_cycle_preset::expand_cycle_preset;
use super::fnc_normalize_cycle_face::normalize_cycle_face;
use super::fnc_weighted_cycle_order::{shuffle_in_place, weighted_cycle_order};
use super::types::MechanicalTile;

/// Resolve a `MechanicalContentSource` into a `ResolvedMechanicalCycle`
/// for the given mechanism tile size.
///
/// `Pair` resolves to an empty cycle; the route helper will inject
/// source and target endpoints when building a route. The non-Pair
/// variants resolve to a fully populated, validated cycle whose face
/// grids are normalized to `tile`.
pub(crate) fn resolve_mechanical_cycle(
    source: &MechanicalContentSource,
    tile: MechanicalTile,
) -> Result<ResolvedMechanicalCycle, MechanicalCycleError> {
    match source {
        MechanicalContentSource::Pair => Ok(ResolvedMechanicalCycle {
            faces: Vec::new(),
            wrap: CycleWrapMode::Bounded,
        }),
        MechanicalContentSource::Ordered { faces, wrap } => {
            resolve_from_face_strings(faces, *wrap, tile)
        }
        MechanicalContentSource::Preset { preset, wrap } => {
            let faces = expand_cycle_preset(*preset);
            resolve_from_face_strings(&faces, *wrap, tile)
        }
        MechanicalContentSource::Randomized { faces, seed, wrap } => {
            let mut shuffled = faces.clone();
            shuffle_in_place(&mut shuffled, *seed);
            resolve_from_face_strings(&shuffled, *wrap, tile)
        }
        MechanicalContentSource::Weighted { faces, seed, wrap } => {
            let order = weighted_cycle_order(faces, *seed)?;
            // weighted_cycle_order intentionally retains duplicates
            // (a face with weight 3 appears 3 times). Skip the
            // duplicate-face rejection in resolve_from_face_strings by
            // building the resolved cycle directly here.
            if order.is_empty() {
                return Err(MechanicalCycleError::EmptyFaces);
            }
            let mut resolved_faces = Vec::with_capacity(order.len());
            for value in order {
                let grid = normalize_cycle_face(&value, tile)?;
                resolved_faces.push(ResolvedMechanicalFace { value, grid });
            }
            if matches!(wrap, CycleWrapMode::Circular)
                && distinct_value_count(&resolved_faces) < 2
            {
                return Err(MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
            }
            Ok(ResolvedMechanicalCycle {
                faces: resolved_faces,
                wrap: *wrap,
            })
        }
    }
}

fn resolve_from_face_strings(
    faces: &[String],
    wrap: CycleWrapMode,
    tile: MechanicalTile,
) -> Result<ResolvedMechanicalCycle, MechanicalCycleError> {
    if faces.is_empty() {
        return Err(MechanicalCycleError::EmptyFaces);
    }
    let mut seen: HashSet<&str> = HashSet::with_capacity(faces.len());
    let mut resolved = Vec::with_capacity(faces.len());
    for value in faces {
        if value.is_empty() {
            return Err(MechanicalCycleError::EmptyFaceValue);
        }
        if !seen.insert(value.as_str()) {
            return Err(MechanicalCycleError::DuplicateFace {
                value: value.clone(),
            });
        }
        let grid = normalize_cycle_face(value, tile)?;
        resolved.push(ResolvedMechanicalFace {
            value: value.clone(),
            grid,
        });
    }
    if matches!(wrap, CycleWrapMode::Circular) && resolved.len() < 2 {
        return Err(MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
    }
    Ok(ResolvedMechanicalCycle {
        faces: resolved,
        wrap,
    })
}

fn distinct_value_count(faces: &[ResolvedMechanicalFace]) -> usize {
    let mut seen: HashSet<&str> = HashSet::with_capacity(faces.len());
    for f in faces {
        seen.insert(f.value.as_str());
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use super::super::fnc_grid_text::grid_to_text;
    use super::*;
    use crate::types::{MechanicalCyclePreset, WeightedCycleFace};

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    fn values(cycle: &ResolvedMechanicalCycle) -> Vec<&str> {
        cycle.faces.iter().map(|f| f.value.as_str()).collect()
    }

    #[test]
    fn pair_resolves_to_empty_face_list() {
        let cycle = resolve_mechanical_cycle(&MechanicalContentSource::Pair, tile(1, 1)).unwrap();
        assert!(cycle.faces.is_empty());
    }

    #[test]
    fn ordered_three_faces_preserves_order() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&cycle), vec!["A", "B", "C"]);
        assert_eq!(cycle.wrap, CycleWrapMode::Circular);
    }

    #[test]
    fn ordered_face_grid_padded_to_tile() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into()],
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 1)).unwrap();
        assert_eq!(grid_to_text(&cycle.faces[0].grid), "A  ");
        assert_eq!(grid_to_text(&cycle.faces[1].grid), "B  ");
    }

    #[test]
    fn preset_decimal_digits_resolves_to_ten_faces() {
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(cycle.faces.len(), 10);
        assert_eq!(values(&cycle), vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }

    #[test]
    fn duplicate_ordered_face_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "A".into()],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert!(
            matches!(err, MechanicalCycleError::DuplicateFace { ref value } if value == "A"),
            "{err:?}",
        );
    }

    #[test]
    fn empty_ordered_face_list_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec![],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert_eq!(err, MechanicalCycleError::EmptyFaces);
    }

    #[test]
    fn circular_with_one_face_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into()],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert_eq!(err, MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
    }

    #[test]
    fn bounded_with_one_face_is_allowed() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&cycle), vec!["A"]);
        assert_eq!(cycle.wrap, CycleWrapMode::Bounded);
    }

    #[test]
    fn randomized_is_deterministic_per_seed() {
        let src = MechanicalContentSource::Randomized {
            faces: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            seed: 42,
            wrap: CycleWrapMode::Circular,
        };
        let a = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        let b = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&a), values(&b));
    }

    #[test]
    fn weighted_resolves_with_multiplicity() {
        let src = MechanicalContentSource::Weighted {
            faces: vec![
                WeightedCycleFace {
                    value: "7".into(),
                    weight: 1,
                },
                WeightedCycleFace {
                    value: "$".into(),
                    weight: 3,
                },
            ],
            seed: 99,
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(cycle.faces.len(), 4);
        let count_seven = values(&cycle).iter().filter(|v| **v == "7").count();
        let count_dollar = values(&cycle).iter().filter(|v| **v == "$").count();
        assert_eq!(count_seven, 1);
        assert_eq!(count_dollar, 3);
    }

    #[test]
    fn ordered_oversized_face_is_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "ABC".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert!(matches!(err, MechanicalCycleError::FaceExceedsTile { .. }));
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
